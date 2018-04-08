
use std::sync::{Mutex,Arc};
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use resource::ResourceTrait;
use resource::ResourceAddress;
use resource::Cash;

use master_storage::get_master_storage;

static mut LOCAL_STORAGE: *const LocalStorage = 0 as *const LocalStorage;

pub struct LocalContainer(Arc<InnerLocalContainer>);

struct InnerLocalContainer {
    address:ResourceAddress,
    cash:Option<Cash>//TODO Maybe mutex?
}

pub struct LocalStorage(Mutex<InnerLocalStorage>);

struct InnerLocalStorage {
    resources:HashMap<ResourceAddress, *const LocalContainer> //TODO HashSet or something fast
}

impl LocalContainer {
    fn new(resource_address:ResourceAddress) -> Self {
        LocalContainer(Arc::new(InnerLocalContainer::new(resource_address)))
    }

    pub fn set_cash(&self, cash:Cash) -> *const Cash {
        unsafe {
            let container_cash=&mut *((&self.0.cash as *const Option<Cash>) as *mut Option<Cash>);
            *container_cash=Some(cash);

            let cash=match *container_cash {
                Some(ref cash) => cash as *const Cash,
                None => unimplemented!()
            };

            cash
        }
    }

    pub fn get_cash(&self) -> Option<*const Cash> {
        unsafe {
            let container_cash=&self.0.cash;

            match *container_cash {
                Some(ref cash) => Some(cash as *const Cash),
                None => None
            }
        }
    }
}

impl Clone for LocalContainer {
    fn clone(&self) -> Self {
        LocalContainer(self.0.clone())
    }
}

impl InnerLocalContainer {
    fn new(address:ResourceAddress) -> Self {
        InnerLocalContainer {
            address,
            cash:None
        }
    }
}

impl Drop for InnerLocalContainer {
    fn drop(&mut self) {
        if self.address.is_local() {
            get_master_storage().delete(&self.address);
        }
        //TODO remove from MasterStorage first
        get_local_storage().delete(&self.address);
    }
}

impl LocalStorage {
    pub fn new() -> Self {
        LocalStorage(Mutex::new(InnerLocalStorage::new()))
    }

    pub fn find_or_insert(&self, resource_address:&ResourceAddress) -> LocalContainer {
        let (local_container, send_inc) = {
            let mut local_storage = match self.0.lock() {
                Ok(local_storage) => local_storage,
                Err(_) => unimplemented!()
            };

            local_storage.find_or_insert(resource_address)
        };

        if send_inc {
            //TODO:send increment to server, if server has cashed or removed resource?
        }

        local_container
    }

    fn delete(&self, resource_address:&ResourceAddress) {
        let send_dec = {
            let mut local_storage=match self.0.lock() {
                Ok(local_storage) => local_storage,
                Err(_) => unimplemented!()
            };

            local_storage.delete(resource_address)
        };

        if send_dec {
            //TODO send decrement to server, if server has cashed?
        }
    }
}

impl InnerLocalStorage {
    pub fn new() -> Self {
        InnerLocalStorage{
            resources:HashMap::with_capacity(1024)
        }
    }

    fn find_or_insert(&mut self, resource_address:&ResourceAddress) -> (LocalContainer, bool) {
        match self.resources.entry(resource_address.clone()) {
            Entry::Occupied(e) => {
                let local_container=unsafe{&(**e.get())};
                (local_container.clone(), false)
            }
            Entry::Vacant(e) => {
                let local_container=LocalContainer::new(resource_address.clone());
                e.insert(&local_container as *const LocalContainer);
                (local_container, !resource_address.is_local())
            }
        }
    }

    fn delete(&mut self, resource_address:&ResourceAddress) -> bool {
        self.resources.remove(resource_address);

        !resource_address.is_local()
    }
}

pub fn create_local_storage() {
    let local_storage=Box::new(LocalStorage::new());
    unsafe{LOCAL_STORAGE=Box::into_raw(local_storage);}
}

pub fn get_local_storage() -> &'static LocalStorage {
    unsafe{&*(LOCAL_STORAGE)}
}

pub fn delete_local_storage() {
    unsafe{
        if LOCAL_STORAGE!=0 as *const LocalStorage {
            let local_storage=Box::from_raw(LOCAL_STORAGE as *mut LocalStorage);
            LOCAL_STORAGE = 0 as *const LocalStorage;

            //TODO drop LocalStorage
        }
    }
}
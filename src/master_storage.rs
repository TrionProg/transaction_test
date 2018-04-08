
use std::sync::Mutex;
use std::sync::Arc;//TODO temp
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use object_pool::growable::{Pool,ID};

use resource::ResourceTrait;
use resource::ResourceAddress;
use resource::ResourceReference;
use resource::Cash;

use arbiter::ArbiterTrait;

static mut MASTER_STORAGE: *const MasterStorage = 0 as *const MasterStorage;

pub type MasterContainer=Box<MasterContainerTrait>;

pub struct MasterStorage(Mutex<InnerMasterStorage>);

struct InnerMasterStorage {
    resources:Pool<MasterContainer, MasterContainer> //TODO use multiple pools
}

impl MasterStorage {
    pub fn new() -> Self {
        MasterStorage(Mutex::new(InnerMasterStorage::new()))
    }

    pub fn insert_resource<R:ResourceTrait>(&self, container:MasterContainer, resource:Box<ResourceTrait>) -> ResourceReference<R> {
        let (resource_address, container) = {
            let mut master_storage = match self.0.lock() {
                Ok(master_storage) => master_storage,
                Err(_) => unimplemented!()
            };

            master_storage.insert_resource(container)
        };

        let mut resource_reference=ResourceReference::new(resource_address.clone());

        let cash_resource=unsafe {
            let container=&mut *(container as *mut MasterContainer);
            container.init(resource_address, resource)
        };

        match cash_resource {
            Some(resource) => resource_reference.set_cash(resource),
            None => {}
        }

        resource_reference
    }

    /*
    pub fn get_resource<F,T>(&self, address:ResourceAddress) -> T where
        F:FnOnce(&MasterContainer) -> T
    {
        let container={
            let mut master_storage = match self.0.lock() {
                Ok(master_storage) => master_storage,
                Err(_) => unimplemented!()
            };

            master_storage.get_resource(&address)
        };

        //downcast container


        let mut master_storage = match self.0.lock() {
            Ok(master_storage) => master_storage,
            Err(_) => unimplemented!()
        };

        f(master_storage.get_resource(&address))
    }
    */

    pub fn delete(&self, resource_address:&ResourceAddress) {
        //TODO check arbiter is not empty and then do not delete
    }
}

impl InnerMasterStorage {
    fn new() -> Self{
        InnerMasterStorage {
            resources:Pool::new()
        }
    }

    fn insert_resource(&mut self, container:MasterContainer) -> (ResourceAddress, *const MasterContainer) {
        let id=self.resources.insert(container);//TODO get *const MasterContainer
        let resource_address=ResourceAddress::new(id.slot_index as u64);
        let container=match self.resources.get(id) {
            Some(container) => container as *const MasterContainer,
            None => unimplemented!()
        };

        (resource_address,container)
    }

    fn get_resource(&self, address:&ResourceAddress) -> *const MasterContainer {
        match self.resources.get(ID::new(address.local_resource_address as usize)){
            Some(container) => container as *const MasterContainer,
            None => unimplemented!()
        }
    }

    //fn delete
}

pub trait MasterContainerTrait {
    fn init(&mut self, resource_address:ResourceAddress, resource:Box<ResourceTrait>) -> Option<Box<ResourceTrait>>;
}

pub fn create_master_storage() {
    let master_storage=Box::new(MasterStorage::new());
    unsafe{MASTER_STORAGE=Box::into_raw(master_storage);}
}

pub fn get_master_storage() -> &'static MasterStorage {
    unsafe{&*(MASTER_STORAGE)}
}

pub fn delete_master_storage() {
    unsafe{
        if MASTER_STORAGE!=0 as *const MasterStorage {
            let local_storage=Box::from_raw(MASTER_STORAGE as *mut MasterStorage);
            MASTER_STORAGE = 0 as *const MasterStorage;

            //TODO drop MasterStorage
        }
    }
}

use std::sync::Mutex;
use std::sync::Arc;//TODO temp
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use object_pool::growable::{Pool,ID};

use super::resource::ResourceTrait;
use super::resource::ResourceAddress;
use super::resource::ResourceReference;

static mut MASTER_STORAGE: *const MasterStorage = 0 as *const MasterStorage;

pub struct MasterStorage(Mutex<InnerMasterStorage>);

struct InnerMasterStorage {
    resources:Pool<(), ()>
}

impl MasterStorage {
    pub fn new() -> Self {
        MasterStorage(Mutex::new(InnerMasterStorage::new()))
    }

    pub fn insert_resource<R: ResourceTrait+'static>(&self, resource: R) -> ResourceReference<R> {
        let resource_address = {
            let mut master_storage = match self.0.lock() {
                Ok(master_storage) => master_storage,
                Err(_) => unimplemented!()
            };

            master_storage.insert_resource()
        };

        let resource_reference = ResourceReference::new(resource_address, resource);

        resource_reference
    }
}

impl InnerMasterStorage {
    fn new() -> Self{
        InnerMasterStorage {
            resources:Pool::new()
        }
    }

    fn insert_resource(&mut self) -> ResourceAddress {
        let id=self.resources.insert(());
        let resource_address=ResourceAddress::new(id.slot_index as u64);

        resource_address
    }
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
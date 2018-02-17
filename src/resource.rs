
use server::ServerID;
use server::get_server_id;

use local_storage::LocalContainer;
use local_storage::get_local_storage;

pub type LocalResourceAddress=u64;

#[derive(Eq,PartialEq,Clone,Debug,Hash)]
pub struct ResourceAddress {
    server_id:ServerID,
    local_resource_address:LocalResourceAddress,
}

pub struct ResourceReference<R:ResourceTrait> {
    address:ResourceAddress,
    local:Option<LocalContainer>,
    cash:Option<*const R>
}

impl ResourceAddress {
    pub fn new(local_resource_address:LocalResourceAddress) -> Self {
        ResourceAddress {
            server_id:get_server_id(),
            local_resource_address
        }
    }

    pub fn invalid() -> Self {
        ResourceAddress {
            server_id:0,
            local_resource_address:0
        }
    }

    pub fn is_local(&self) -> bool {
        self.server_id==get_server_id()
    }
}

impl<R:ResourceTrait> ResourceReference<R> {
    pub fn new(address:ResourceAddress) -> Self {//TODO
        ResourceReference {
            address,
            local:None,
            cash:None
        }
    }

    fn is_local(&self) -> bool {
        self.address.is_local()
    }
}

impl<R:ResourceTrait> Clone for ResourceReference<R> {
    fn clone(&self) -> Self {
        match self.local {
            Some(ref local) => {
                ResourceReference {
                    address:self.address.clone(),
                    local:self.local.clone(),
                    cash:self.cash.clone(),
                }
            },
            None => {
                let local=Some(get_local_storage().find_or_insert(&self.address));
                let cash=None;//TODO

                ResourceReference {
                    address:self.address.clone(),
                    local,
                    cash,
                }
            }
        }
    }
}

pub trait ResourceTrait {}
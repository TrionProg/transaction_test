
use server::ServerID;
use server::get_server_id;

use local_storage::LocalContainer;
use local_storage::get_local_storage;
use master_storage::MasterContainer;

pub type LocalResourceAddress=u64;

pub type Cash=Box<ResourceTrait>;

#[derive(Eq,PartialEq,Clone,Debug,Hash)]
pub struct ResourceAddress {
    pub server_id:ServerID,
    pub local_resource_address:LocalResourceAddress,
}

pub struct ResourceReference<R:ResourceTrait> {
    address:ResourceAddress,
    local:Option<LocalContainer>,
    //master:Option<*const MasterContainer>,//selten benutzt
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

impl<R:ResourceTrait+'static> ResourceReference<R> {
    //only by master
    pub fn new(address:ResourceAddress) -> Self {//TODO
        let local=Some(get_local_storage().find_or_insert(&address));
        let cash=None;//TODO

        ResourceReference {
            address,
            local:local,
            cash:cash
        }
    }

    //only by master after new
    pub fn set_cash(&mut self, cash:Box<R>) {
        match self.local {
            Some(local) => {
                let cash=local.set_cash(cash);

                //TODO self.cash=Some(cash);
            }
        }
    }

    //get cash (&mut self)

    fn is_local(&self) -> bool {
        self.address.is_local()
    }

    pub fn get_address(&self) -> ResourceAddress {
        self.address.clone()
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

pub trait ResourceTrait{}
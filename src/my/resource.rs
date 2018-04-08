
use std::sync::Arc;

pub type LocalResourceAddress=u64;

#[derive(Eq,PartialEq,Clone,Debug,Hash)]
pub struct ResourceAddress {
    pub local_resource_address:LocalResourceAddress,
}

pub struct ResourceReference<R:ResourceTrait> {
    address:ResourceAddress,
    resource:Arc<R>,
}

impl ResourceAddress {
    pub fn new(local_resource_address:LocalResourceAddress) -> Self {
        ResourceAddress {
            local_resource_address
        }
    }

    pub fn invalid() -> Self {
        ResourceAddress {
            local_resource_address:0
        }
    }

    pub fn is_local(&self) -> bool {
        true
    }
}

impl<R:ResourceTrait+'static> ResourceReference<R> {
    //only by master
    pub fn new(address:ResourceAddress, resource:R) -> Self {
        ResourceReference {
            address,
            resource:Arc::new(resource)
        }
    }

    fn is_local(&self) -> bool {
        self.address.is_local()
    }

    pub fn get_address(&self) -> ResourceAddress {
        self.address.clone()
    }

    pub fn get_resource(&self) -> &R {
        &&self.resource
    }
}

impl<R:ResourceTrait> Clone for ResourceReference<R> {
    fn clone(&self) -> Self {
        ResourceReference {
            address: self.address.clone(),
            resource: self.resource.clone()
        }
    }
}

pub trait ResourceTrait{}
pub trait FieldTrait{}
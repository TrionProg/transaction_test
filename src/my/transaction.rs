
use std::cell::UnsafeCell;
use std::collections::HashMap;

use super::resource::ResourceAddress;
use super::resource::FieldTrait;
use super::view::{ObjectView,ObjectViewTrait};

use common::TransactionInfo;

pub struct Transaction {
    inner:UnsafeCell<InnerTransaction>
}

struct InnerTransaction {
    transaction:TransactionInfo,
    modifiers:Vec<Box<ModifierTrait>>,
    cached_fields:Vec<Box<FieldTrait>>,
    //object_views:Vec<Box<ObjectView<ObjectViewTrait>>>
    //accessed_resources:HashMap<ResourceAddress, Box<AccessedResourceTrait>>
}

impl Transaction {
    pub fn new(transaction:TransactionInfo) -> Self {
        let inner=InnerTransaction {
            transaction,
            modifiers:Vec::with_capacity(2),
            cached_fields:Vec::with_capacity(2),
            //object_views:Vec::with_capacity(2),
            //accessed_resources:HashMap::with_capacity(2)
        };

        Transaction {
            inner:UnsafeCell::new(inner)
        }
    }

    pub fn get_info(&self) -> TransactionInfo {
        let transaction=unsafe{ &mut *self.inner.get() };

        transaction.transaction.clone()
    }

    pub fn add_modifier(&self, modifier:Box<ModifierTrait>) -> &Box<ModifierTrait> {
        let transaction=unsafe{ &mut *self.inner.get() };

        let len=transaction.modifiers.len();
        transaction.modifiers.push(modifier);

        unsafe{ &*(&transaction.modifiers[len] as *const Box<ModifierTrait>)}
    }
}

pub trait ModifierTrait {
    fn apply(self);
}
/*
pub trait AccessedResourceTrait {

}
*/
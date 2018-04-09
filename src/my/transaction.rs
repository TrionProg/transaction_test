
use std::cell::UnsafeCell;
use std::collections::HashMap;

use super::resource::ResourceAddress;
use super::resource::FieldTrait;
use super::view::{ObjectView,ObjectViewTrait,BigObjectViewTrait,FieldViewTrait};

use common::TransactionInfo;

pub struct Transaction {
    inner:UnsafeCell<InnerTransaction>
}

struct InnerTransaction {
    transaction:TransactionInfo,

    modifiers:Vec<Box<ModifierTrait>>,
    cached_fields:Vec<Box<FieldTrait>>,
    field_views:Vec<Box<FieldViewTrait>>,
    object_views:Vec<Box<BigObjectViewTrait>>,

    object_views_hash_map:HashMap<ResourceAddress, *const Box<BigObjectViewTrait>>
    //modified object views
}

impl Transaction {
    pub fn new(transaction:TransactionInfo) -> Self {
        let inner=InnerTransaction {
            transaction,
            modifiers:Vec::with_capacity(2),
            cached_fields:Vec::with_capacity(2),
            field_views:Vec::with_capacity(2),
            object_views:Vec::with_capacity(2),

            object_views_hash_map:HashMap::with_capacity(2)
        };

        Transaction {
            inner:UnsafeCell::new(inner)
        }
    }

    pub fn get_info(&self) -> TransactionInfo {
        let transaction=unsafe{ &mut *self.inner.get() };

        transaction.transaction.clone()
    }

    pub fn add_modifier(&self, modifier:Box<ModifierTrait>) -> *const Box<ModifierTrait> {
        let transaction=unsafe{ &mut *self.inner.get() };

        let len=transaction.modifiers.len();
        transaction.modifiers.push(modifier);

        unsafe{ (&transaction.modifiers[len] as *const Box<ModifierTrait>)}
    }

    pub fn get_object_view(&self, object_address:&ResourceAddress) -> Option<*const Box<BigObjectViewTrait>> {
        let transaction=unsafe{ &mut *self.inner.get() };

        match transaction.object_views_hash_map.get(object_address) {
            Some(object_view) => Some(*object_view),
            None => None
        }
    }

    pub fn add_object_view(&self, object_address:ResourceAddress, object_view:Box<BigObjectViewTrait>) -> *const Box<BigObjectViewTrait> {
        let transaction=unsafe{ &mut *self.inner.get() };

        let len=transaction.object_views.len();
        transaction.object_views.push(object_view);

        let address=unsafe{ (&transaction.object_views[len] as *const Box<BigObjectViewTrait>)};

        transaction.object_views_hash_map.insert(object_address, address);

        address
    }

    //pub fn add_field
}

pub trait ModifierTrait {
    fn apply(self);
}
/*
pub trait AccessedResourceTrait {

}
*/
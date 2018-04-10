
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

    object_views_hash_map:HashMap<ResourceAddress, *const Box<BigObjectViewTrait>>,
    modified_object:Vec<bool>,
}

impl Transaction {
    pub fn new(transaction:TransactionInfo) -> Self {
        let inner=InnerTransaction {
            transaction,
            modifiers:Vec::with_capacity(2000),//TODO зачем-то дропит боксы... ааа, яже ссылаюсь на &Box
            cached_fields:Vec::with_capacity(2000),
            field_views:Vec::with_capacity(2000),
            object_views:Vec::with_capacity(2000),

            object_views_hash_map:HashMap::with_capacity(2000),
            modified_object:Vec::with_capacity(2000)
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
        unsafe {
            let transaction = &mut *self.inner.get();

            match transaction.object_views_hash_map.get(object_address) {
                Some(object_view) => Some(*object_view),
                None => None
            }
        }
    }

    pub fn add_object_view(&self, object_address:ResourceAddress, object_view:Box<BigObjectViewTrait>) -> *const Box<BigObjectViewTrait> {
        unsafe {
            let transaction = &mut *self.inner.get();

            let len = transaction.object_views.len();
            transaction.object_views.push(object_view);

            let address = unsafe { (&transaction.object_views[len] as *const Box<BigObjectViewTrait>) };

            transaction.object_views_hash_map.insert(object_address, address);

            address
        }
    }

    pub fn add_field_view(&self, field_view:Box<FieldViewTrait>) -> *const Box<FieldViewTrait> {
        let transaction=unsafe{ &mut *self.inner.get() };

        let len=transaction.field_views.len();
        transaction.field_views.push(field_view);

        let address=unsafe{ (&transaction.field_views[len] as *const Box<FieldViewTrait>)};

        address
    }

    pub fn object_modified(&self) {
        let transaction=unsafe{ &mut *self.inner.get() };

        transaction.modified_object.push(false);
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        let transaction=unsafe{ &mut *self.inner.get() };

        //println!("drop {}", transaction.object_views_hash_map.len());


        for (k,object_view) in transaction.object_views_hash_map.iter() {
            //println!("drop2 {:?}",k);
            let object_view:&Box<BigObjectViewTrait>=unsafe{ &**object_view };
            object_view.release(&transaction.transaction);
        }

        println!("Trans Finished: {:?} {} {}", transaction.transaction, transaction.modified_object.len(),transaction.object_views.len());
    }
}

pub trait ModifierTrait {
    fn apply(self);
}
/*
pub trait AccessedResourceTrait {

}
*/
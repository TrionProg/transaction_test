
use std::collections::HashMap;

use super::resource::ResourceAddress;
use super::resource::FieldTrait;

use common::TransactionInfo;

pub struct Transaction {
    transaction:TransactionInfo,
    modifiers:Vec<Box<ModifierTrait>>,
    cached_fields:Vec<Box<FieldTrait>>,
    accessed_resources:HashMap<ResourceAddress, Box<AccessedResourceTrait>>
}

impl Transaction {
    pub fn new(transaction:TransactionInfo) -> Self {
        Transaction {
            transaction,
            modifiers:Vec::with_capacity(2),
            cached_fields:Vec::with_capacity(2),
            accessed_resources:HashMap::with_capacity(2)
        }
    }

    pub fn get_info(&self) -> TransactionInfo {
        self.transaction.clone()
    }
}

pub trait ModifierTrait {
    fn apply(self);
}

pub trait AccessedResourceTrait {

}
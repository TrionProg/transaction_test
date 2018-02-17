
use std::cmp::Ordering;

use transaction::TransactionID;
use access::AccessPriority;

use generic_array::{GenericArray,ArrayLength};

pub struct Access<AM:AccessModeTrait> {
    pub transaction_id:TransactionID,
    pub priority:AccessPriority,
    pub access_mode:AM
}

pub struct ArbiterCore<FC:ArrayLength<FieldState>> {
    field_states: GenericArray<FieldState, FC>
}

#[derive(Clone)]
pub struct FieldState {
    pub ref_count:u16
}

pub trait ArbiterTrait<AM:AccessModeTrait> {
    fn lock(&self, access:Access<AM>) -> bool;
    fn unlock(&self, transaction_id:TransactionID) -> Option<TransactionID>;
}

pub trait AccessModeTrait {
    fn involved(&self) -> &[u8];
    fn mode(&self) -> &[u8];
    fn prev(&self) -> &[u8];
    fn recursion(&self) -> &[u8];
    fn involved_list(&self) -> &[u8];
}

impl FieldState {
    pub fn new() -> Self {
        FieldState {
            ref_count:0
        }
    }

    pub fn max_ref_count() -> u16 {
        u16::max_value()-1
    }
}

impl<FC:ArrayLength<FieldState>> ArbiterCore<FC> {
    pub fn new() -> Self {
        ArbiterCore {
            field_states: GenericArray::generate(|_|FieldState::new()),
        }
    }
    
    pub fn can_lock(&self, mode:&[u8], recursion:&[u8], fi:usize) -> bool {
        match mode[fi/8] & 1<<fi%8 > 0 {
            true => {//write
                if self.field_states[fi].ref_count == 0 {//write
                    if !(recursion[fi/8] & 1<<fi%8 > 0) {//NOT( write->write and recursion => locked by this transaction)
                        return false;
                    }
                }else if self.field_states[fi].ref_count > 1 {//read
                    if !(recursion[fi/8] & 1<<fi%8 > 0 && self.field_states[fi].ref_count==2) {//NOT( read->write and recursion and only one => locked by this transaction)
                        return false;
                    }
                }
            },
            false => {//read
                if self.field_states[fi].ref_count == 0 {//write
                    if !(recursion[fi/8] & 1<<fi%8 > 0) {//NOT( write->read and recursion => locked by this transaction)
                        return false;
                    }
                }else if self.field_states[fi].ref_count==FieldState::max_ref_count() {//overflow
                    return false;
                }
            }
        }

        true
    }

    pub fn lock(&mut self, mode:&[u8], recursion:&[u8], fi:usize) {
        match mode[fi/8] & 1<<fi%8 > 0 {
            true => {//write
                if self.field_states[fi].ref_count >= 1 {//read or not locked
                    self.field_states[fi].ref_count=0;
                }
                //write => recursion => do not change
            },
            false => {//read
                if self.field_states[fi].ref_count > 1 {//read
                    if recursion[fi/8] & 1<<fi%8 == 0 {//no recursion
                        self.field_states[fi].ref_count+=1;
                    }
                }else if self.field_states[fi].ref_count == 1 {//not locked
                    self.field_states[fi].ref_count = 2;
                }
                //write => recursion => do not change
            }
        }
    }

    pub fn unlock(&mut self, mode:&[u8], prev:&[u8], recursion:&[u8], fi:usize) {
        match mode[fi/8] & 1<<fi%8 > 0 {
            true => {//write
                if recursion[fi/8] & 1<<fi%8 == 0 {//recursion
                    if prev[fi/8] & 1<<fi%8 == 0 {//read -> write
                        self.field_states[fi].ref_count = 2;
                    }
                    //write -> write => do not change
                }else{//not locked -> write
                    self.field_states[fi].ref_count=1;
                }
            },
            false => {//read
                if recursion[fi/8] & 1<<fi%8 == 0 {//no recursion
                    self.field_states[fi].ref_count-=1;
                }
                //write|read -> read => do not change
            }
        }
    }
}

impl<AM:AccessModeTrait> Access<AM> {
    pub fn new(transaction_id:TransactionID, priority:AccessPriority, access_mode:AM) -> Self {
        Access {
            transaction_id,
            priority,
            access_mode
        }
    }
}

impl<AM:AccessModeTrait> Eq for Access<AM> {}

impl<AM:AccessModeTrait> PartialEq for Access<AM>{
    fn eq(&self, other: &Access<AM>) -> bool {
        self.cmp(other)==Ordering::Equal
    }
}

impl<AM:AccessModeTrait> PartialOrd for Access<AM>{
    fn partial_cmp(&self, other: &Access<AM>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<AM:AccessModeTrait> Ord for Access<AM> {
    fn cmp(&self, other: &Access<AM>) -> Ordering {
        let ord = self.priority.cmp(&other.priority);

        match ord {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => ord
        }
    }
}
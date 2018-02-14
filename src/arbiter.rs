
use transaction::TransactionID;
use access::AccessPriority;

use generic_array::ArrayLength;

pub type FieldStateLength=ArrayLength<FieldState>;

pub struct Access<AM:AccessModeTrait> {
    transaction_id:TransactionID,
    priority:AccessPriority,
    access_mode:AM
}

pub struct ArbiterCore<FC:FieldStateLength> {
    field_states: GenericArray<FieldState, FC>
}

struct FieldState {
    ref_count:u16
}

pub trait ArbiterTrait<AM:AccessModeTrait> {
    fn lock(&self, access:Access<AM>) -> bool;
    fn unlock(&self, process_id:ProcessID) -> Option<TransactionID>;
}

pub trait AccessModeTrait {
    fn involved(&self) -> &[u8];
    fn mode(&self) -> &[u8];
    fn recursion(&self) -> &[u8];
    fn involved_list(&self) -> &[u8];
}

impl<FC:FieldStateLength> ArbiterCore<FC> {
    fn can_lock(&self, mode:&[u8], recursion:&[u8], fi:usize) -> bool {
        match mode[fi/8] & 1<<fi%8 > 0 {
            true => {//write
                if self.field_states[fi].ref_count == 0 {//write
                    if !(recursion[fi/8] & 1<<fi%8 > 0) {//NOT write->write => locked by this transaction
                        return false;
                    }
                }else if self.field_states[fi].ref_count > 1 {//read
                    if !(recursion[fi/8] & 1<<fi%8 > 0 && self.field_states[fi].ref_count==2) {//NOT read->write and read is this => locked by this transaction
                        return false;
                    }
                }
            },
            false => {//read
                if self.field_states[fi].ref_count == 0 {//write
                    if !(recursion[fi/8] & 1<<fi%8 > 0) {//NOT write->read => locked by this transaction
                        return false;
                    }
                }
            }
        }

        true
    }

    fn lock(&mut self, mode:&[u8], recursion:&[u8], fi:usize) {
        match mode[fi/8] & 1<<fi%8 > 0 {
            true => {//write
                if self.field_states[fi].ref_count >= 1 {//read or not locked
                    self.field_states[fi].ref_count=0;
                }
            },
            false => {//read
                if self.field_states[fi].ref_count > 1 {//read
                    if recursion[fi/8] & 1<<fi%8 == 0 {
                        self.field_states[fi].ref_count+=1;
                    }
                }else if self.field_states[fi].ref_count == 1 {//not locked
                    self.field_states[fi].ref_count = 2;
                }
            }
        }
    }

    fn unlock(&mut self, mode:&[u8], prev:&[u8], recursion:&[u8], fi:usize) {
        match mode[fi/8] & 1<<fi%8 > 0 {
            true => {//write
                if recursion[fi/8] & 1<<fi%8 == 0 {
                    self.field_states[fi].ref_count=1;
                }
                //todo about ref_count=2?
            },
            false => {//read
                if recursion[fi/8] & 1<<fi%8 == 0 {
                    self.field_states[fi].ref_count-=1;
                }
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

    pub fn get_access_mode(&self) -> &AM{
        &self.access_mode
    }
}

//impl<AM:AccessModeTrait> Eq for Access<AM> {}

impl<AM:AccessModeTrait> Ord for Access<AM> {
    fn cmp(&self, other: &Access<AM>) -> Ordering {
        let ord = self.priority.cmp(other.priority);

        match ord {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => ord
        }
    }
}
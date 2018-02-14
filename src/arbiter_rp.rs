
use std::sync::Mutex;
use std::collections::BinaryHeap;

use arbiter::ArbiterTrait;
use arbiter::AccessModeTrait;
use arbiter::Access;
use arbiter::ArbiterCore;
use arbiter::FieldStateLength;

pub struct ArbiterRP<AM:AccessModeTrait, FC: FieldStateLength>(
    Mutex(InnerArbiterRP<AM,FC>)
);

pub struct InnerArbiterRP<AM:AccessModeTrait,FC: FieldStateLength> {
    field_states: GenericArray<FieldState, FC>,
    waiting: BinaryHeap<Access<AM>>,
    locking: Vec<Access<AM>>
}

impl<AM:AccessModeTrait, FC: FieldStateLength> InnerArbiterRP<AM,FC> {
    fn new() -> Self {
        InnerArbiterRP {
            field_states: GenericArray::generate(|x|0),
            waiting: BinaryHeap::with_capacity(2),
            locking: Vec::with_capacity(2),
        }
    }

    fn match_2_accesses(&self, access:&Access<AM>, hp_access:&Access<AM>) -> bool {
        let access_mode=access.get_access_mode();
        let involved=access_mode.involved();
        let mode=access_mode.mode();

        let hp_access_mode=hp_access.get_access_mode();
        let hp_involved=hp_access_mode.involved();
        let hp_mode=hp_access_mode.mode();

        for i in access_mode.involved_list().iter() {

        }
    }

    fn match_access(&self, access:&Access<AM>) -> bool {
        let access_mode=access.get_access_mode();
        let involved=access_mode.involved();
        let mode=access_mode.mode();
        let recursion=access_mode.recursion();

        for i in access_mode.involved_list().iter() {
            let fi=*i as usize;

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
                    }else if self.field_states[fi].ref_count > 1 {//read
                        return true;
                    }
                }
            }
        }
    }
}

impl<AM:AccessModeTrait, FC: FieldStateLength> ArbiterTrait<AM> for ArbiterRP<AM,FC> {
    fn lock(&self, access:Access<AM>) -> bool {
        let mut arbiter=match self.0.lock() {
            Ok(arbiter) => arbiter,
            Err(_) => unimplemented!()
        };

        let lock=match arbiter.waiting.peak() {
            None => true,
            Some(ref hp_access) => {
                match hp_access.cmp(&access) {
                    Ordering::Greater => arbiter.match_2_accesses(&access, hp_access),
                    _ => arbiter.match_access(&access)
                }
            }
        };
    }

    fn unlock(&self, process_id:ProcessID) -> Option<TransactionID> {
        let mut arbiter=match self.0.lock() {
            Ok(arbiter) => arbiter,
            Err(_) => unimplemented!()
        };

        //TODO

        None
    }
}


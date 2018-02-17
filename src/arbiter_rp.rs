
use std::sync::Mutex;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

use generic_array::ArrayLength;

use transaction::TransactionID;

use arbiter::ArbiterTrait;
use arbiter::AccessModeTrait;
use arbiter::Access;
use arbiter::ArbiterCore;
use arbiter::FieldState;

pub struct ArbiterRP<AM:AccessModeTrait, FC: ArrayLength<FieldState>>(
    Mutex<InnerArbiterRP<AM,FC>>
);

pub struct InnerArbiterRP<AM:AccessModeTrait,FC: ArrayLength<FieldState>> {
    arbiter_core:ArbiterCore<FC>,
    waiting: BinaryHeap<Access<AM>>,
    locking: Vec<Access<AM>>
}

impl<AM:AccessModeTrait, FC: ArrayLength<FieldState>> InnerArbiterRP<AM,FC> {
    fn new() -> Self {
        InnerArbiterRP {
            arbiter_core: ArbiterCore::new(),
            waiting: BinaryHeap::with_capacity(2),
            locking: Vec::with_capacity(2),
        }
    }

    fn match_2_accesses(&self, access:&Access<AM>, hp_access:&Access<AM>) -> bool {
        let access_mode=&access.access_mode;
        let involved=access_mode.involved();
        let mode=access_mode.mode();
        let recursion=access_mode.recursion();

        let hp_access_mode=&hp_access.access_mode;
        let hp_involved=hp_access_mode.involved();

        for fi in access_mode.involved_list().iter() {
            let fi=*fi as usize;

            if involved[fi/8] & 1<<fi%8 == hp_involved[fi/8] & 1<<fi%8 {//must not intersect
                //TODO more details(mode)
                return false;
            }

            if !self.arbiter_core.can_lock(mode, recursion, fi) {
                return false;
            }
        }

        true
    }

    fn match_access(&self, access:&Access<AM>) -> bool {
        let access_mode=&access.access_mode;
        let mode=access_mode.mode();
        let recursion=access_mode.recursion();

        for fi in access_mode.involved_list().iter() {
            let fi=*fi as usize;

            if !self.arbiter_core.can_lock(mode, recursion, fi) {
                return false;
            }
        }

        true
    }

    fn lock(&mut self, access:Access<AM>) {
        {
            let access_mode=&access.access_mode;
            let mode=access_mode.mode();
            let recursion=access_mode.recursion();

            for fi in access_mode.involved_list().iter() {
                let fi=*fi as usize;

                self.arbiter_core.lock(mode, recursion, fi);
            }
        }

        self.locking.push(access);
    }

    fn unlock(&mut self, transaction_id:TransactionID) {
        let mut delete=Vec::with_capacity(4);

        for (i,ref access) in self.locking.iter().enumerate().rev() {
            if access.transaction_id==transaction_id {
                let access_mode=&access.access_mode;
                let mode=access_mode.mode();
                let prev=access_mode.prev();
                let recursion=access_mode.recursion();

                for fi in access_mode.involved_list().iter() {
                    let fi=*fi as usize;

                    self.arbiter_core.unlock(mode, prev, recursion, fi);
                }

                delete.push(i);
            }
        }

        //TODO:Drain filter
        for i in delete.iter() {
            self.locking.remove(*i);
        }
    }
}

impl<AM:AccessModeTrait, FC: ArrayLength<FieldState>> ArbiterTrait<AM> for ArbiterRP<AM,FC> {
    fn lock(&self, access:Access<AM>) -> bool {
        let mut arbiter=match self.0.lock() {
            Ok(arbiter) => arbiter,
            Err(_) => unimplemented!()
        };

        let lock=match arbiter.waiting.peek() {
            None => true,
            Some(hp_access) => {
                match hp_access.cmp(&access) {
                    Ordering::Greater => arbiter.match_2_accesses(&access, hp_access),
                    _ => arbiter.match_access(&access)
                }
            }
        };

        if lock {
            arbiter.lock(access);
        }else{
            arbiter.waiting.push(access);
        }

        lock
    }

    fn unlock(&self, transaction_id:TransactionID) -> Option<TransactionID> {
        let mut arbiter=match self.0.lock() {
            Ok(arbiter) => arbiter,
            Err(_) => unimplemented!()
        };

        arbiter.unlock(transaction_id);

        let continue_transaction=match arbiter.waiting.peek() {
            Some(ref hp_access) =>
                arbiter.match_access(&hp_access),
            None => false
        };

        if continue_transaction {
            let hp_access=match arbiter.waiting.pop() {
                Some(hp_access) => hp_access,
                None => unimplemented!()
            };

            let transaction_id=hp_access.transaction_id.clone();

            arbiter.lock(hp_access);
            Some(transaction_id)
        }else{
            None
        }
    }
}


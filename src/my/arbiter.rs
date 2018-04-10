
use std::sync::{Mutex,Arc,Condvar};

use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use std::cmp::Ordering;

use common::TransactionInfo;

use super::access::BitMask;
use super::access::{Access,UnlockReadAccess};

type WaitHandle=Arc<(Mutex<bool>, Condvar)>;

pub struct Arbiter<BM:BitMask>(Mutex<InnerArbiter<BM>>);

struct InnerArbiter<BM:BitMask> {
    field_write_state:BM,
    field_counter:Vec<u16>,

    //waiting: BinaryHeap<WaitingTransaction<BM>>,
    waiting1:Vec<WaitingTransaction<BM>>,
    waiting2:Vec<WaitingTransaction<BM>>,
    waiting3:Vec<WaitingTransaction<BM>>,
    locking: HashMap<TransactionInfo, LockingTransaction<BM>>
}

struct LockingTransaction<BM:BitMask> {
    involved:BM,
    mode:BM,
}

struct WaitingTransaction<BM:BitMask> {
    wait_handle:WaitHandle,
    access:Access<BM>
}


impl<BM:BitMask> Arbiter<BM> {
    pub fn new() -> Self {
        Arbiter(Mutex::new(InnerArbiter::new()))
    }

    pub fn lock(&self, access:Access<BM>) {
        let wait={//NOTE: wait_handle may be unlocked before wait
            let mut arbiter=match self.0.lock() {
                Ok(arbiter) => arbiter,
                Err(_) => unimplemented!()
            };

            arbiter.lock(access)
        };

        match wait {
            Ok(_) => {},
            Err(wait_handle) => {
                let &(ref lock, ref condvar) = &*wait_handle;

                let mut started = lock.lock().unwrap();
                while !*started {
                    started = condvar.wait(started).unwrap();
                }
            }//TODO
        }
    }

    pub fn unlock_read(&self, access:UnlockReadAccess<BM>) {
        let mut arbiter=match self.0.lock() {
            Ok(arbiter) => arbiter,
            Err(_) => unimplemented!()
        };

        arbiter.unlock_read(access);
    }

    pub fn unlock(&self, transaction:&TransactionInfo) {
        let mut arbiter=match self.0.lock() {
            Ok(arbiter) => arbiter,
            Err(_) => unimplemented!()
        };

        arbiter.unlock(transaction);
    }
}

impl<BM:BitMask> InnerArbiter<BM> {
    fn new() -> Self {
        let field_counter = vec![0; BM::field_count()];

        InnerArbiter {
            field_write_state:BM::zeroed(),
            field_counter,
            //waiting: BinaryHeap::with_capacity(2),
            waiting1:Vec::with_capacity(2),
            waiting2:Vec::with_capacity(2),
            waiting3:Vec::with_capacity(2),
            locking: HashMap::with_capacity(2),
        }
    }

    fn try_lock(field_write_state:&BM, access:&Access<BM>) -> bool {
        !(field_write_state.and(&access.involved) == BM::zeroed())
    }

    fn lock(&mut self, access:Access<BM>) -> Result<(),WaitHandle> {
        if Self::try_lock(&self.field_write_state, &access) {
            println!("collision");
            //TODO collision
            let class_code=access.transaction.class_code;
            let (waiting_transaction, wait_handle)=WaitingTransaction::new(access);

            match class_code {
                1 => self.waiting1.push(waiting_transaction),
                2 => self.waiting2.push(waiting_transaction),
                3 => self.waiting3.push(waiting_transaction),
                _ => unreachable!()
            }

            Err(wait_handle)
        }else{
            if self.apply_lock(access) {
                Ok(())
            }else{//overflow
                unreachable!()
            }
        }
    }

    fn apply_lock(&mut self, access:Access<BM>) -> bool {
        let locking_transaction = self.locking.entry(access.transaction.clone()).or_insert(LockingTransaction::new());

        for i in 0..BM::field_count() {
            if access.involved.get(i) {
                if access.mode.get(i) {//_->w or r->w
                    self.field_counter[i]=0;
                    self.field_write_state.set(i);

                    locking_transaction.involved.set(i);
                    locking_transaction.mode.set(i);
                }else{//_->r or r->r
                    self.field_counter[i]+=1;//TODO first check and then increment

                    locking_transaction.involved.set(i);
                }
            }
        }

        true
    }

    fn unlock_read(&mut self, access:UnlockReadAccess<BM>) {
        let delete_locking_transaction= {
            let locking_transaction = match self.locking.get_mut(&access.transaction) {
                Some(locking_transaction) => locking_transaction,
                None => unreachable!()
            };

            for i in 0..BM::field_count() {
                if access.involved.get(i) {//r->_ or r->r
                    self.field_counter[i] -= 1;

                    locking_transaction.involved.clear(i);
                }
            }

            locking_transaction.involved == BM::zeroed()//TODO may be recursion filter knows it?
        };

        if delete_locking_transaction {
            self.locking.remove(&access.transaction);
        }

        self.try_continue_transactions();
    }

    fn unlock(&mut self, transaction:&TransactionInfo) {
        match self.locking.get_mut(transaction) {
            Some(locking_transaction) => {
                for i in 0..BM::field_count() {
                    if locking_transaction.involved.get(i) {
                        if locking_transaction.mode.get(i) {//w->_
                            self.field_write_state.clear(i);
                        }else{//r->_ or r->r
                            self.field_counter[i] -= 1;
                        }
                    }
                }
            },
            None => unreachable!()
        }

        self.locking.remove(&transaction);

        println!("{:?} {} {}", self.field_write_state, self.field_counter[0], self.field_counter[1]);

        self.try_continue_transactions();
    }

    fn try_continue_transactions(&mut self) {
        if self.waiting1.len()>0 {
            if Self::try_lock(&self.field_write_state, &self.waiting1.last().unwrap().access) {
                let waiting_transaction=self.waiting1.pop().unwrap();

                let wait_handle=waiting_transaction.wait_handle;
                let access=waiting_transaction.access;

                println!("continue");

                self.apply_lock(access);

                let &(ref lock, ref condvar) = &*wait_handle;
                let mut started = lock.lock().unwrap();
                *started = true;
                condvar.notify_one();
            }
        }
    }
}

impl<BM:BitMask> LockingTransaction<BM> {
    pub fn new() -> Self {
        LockingTransaction {
            involved:BM::zeroed(),
            mode:BM::zeroed()
        }
    }
}

impl<BM:BitMask> WaitingTransaction<BM> {
    pub fn new(access:Access<BM>) -> (Self,WaitHandle) {
        let wait_handle=Arc::new((Mutex::new(false),Condvar::new()));

        let waiting_transaction=WaitingTransaction {
            wait_handle:wait_handle.clone(),
            access
        };

        (waiting_transaction,wait_handle)
    }
}

impl<BM:BitMask> Eq for WaitingTransaction<BM> {}

impl<BM:BitMask> PartialEq for WaitingTransaction<BM>{
    fn eq(&self, other: &WaitingTransaction<BM>) -> bool {
        self.cmp(other)==Ordering::Equal
    }
}

impl<BM:BitMask> PartialOrd for WaitingTransaction<BM>{
    fn partial_cmp(&self, other: &WaitingTransaction<BM>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<BM:BitMask> Ord for WaitingTransaction<BM> {
    fn cmp(&self, other: &WaitingTransaction<BM>) -> Ordering {
        let ord = self.access.priority.cmp(&other.access.priority);

        match ord {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => ord
        }
    }
}
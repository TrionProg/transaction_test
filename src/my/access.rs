
use std::ops::BitAnd;

use common::TransactionInfo;

pub struct Access<BM:BitMask> {
    pub transaction:TransactionInfo,
    pub priority:u16,
    pub involved:BM,
    pub mode:BM
}

/*
pub struct Priority {

}
*/

pub struct UnlockReadAccess<BM:BitMask> {
    pub transaction:TransactionInfo,
    pub involved:BM,
}

pub trait BitMask:BitAnd+PartialEq+Eq+Sized {
    fn field_count() -> usize;
    fn zeroed() -> Self;
    fn get(&self, index:usize) -> bool;
    fn set(&mut self, index:usize);
    fn clear(&mut self, index:usize);
    fn and(&self, other: &Self) -> Self;
}
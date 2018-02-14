
use std::sync::Mutex;

use generic_array::{GenericArray,ArrayLength};

use arbiter::ArbiterTrait;
use arbiter::Access;

pub struct ArbiterWP<FC: ArrayLength<u16>>(Mutex(InnerArbiterWP<FC>));

pub struct InnerArbiterWP<FC: ArrayLength<u16>> {
    data: GenericArray<u16, FC>
}

impl<FC: ArrayLength<u16>> InnerArbiterWP<FC> {
    fn new() -> Self {
        InnerArbiterWP {
            data:GenericArray::generate(|x|0)
        }
    }
}

impl<FC: ArrayLength<u16>,AM:AccessModeTrait> ArbiterTrait<AM> for ArbiterWP<N> {
    fn lock(&self, access:Access<AM>) -> bool {
        let mut arbiter=match self.0.lock() {
            Ok(arbiter) => arbiter,
            Err(_) => unimplemented!()
        };

        
    }

    fn unlock(&self, process_id:ProcessID) -> Option<TransactionID> {
        let mut arbiter=match self.0.lock() {
            Ok(arbiter) => arbiter,
            Err(_) => unimplemented!()
        };
    }
}


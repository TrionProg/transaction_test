
use generic_array::{GenericArray,ArrayLength};

pub struct BitSet<BC: ArrayLength<u8>>{
    bytes: GenericArray<u8, BC>
}

impl<BC: ArrayLength<u8>> BitSet<BC> {
    fn new() -> Self {
        BitSet {
            bytes: GenericArray::generate(|_| 0)
        }
    }

    fn as_slice(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut[u8] {
        self.bytes.as_mut_slice()
    }
}
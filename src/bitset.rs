
use generic_array::{GenericArray,ArrayLength};

pub type BitSetLength=ArrayLength<u8>;

pub struct BitSet<BC: BitSetLength>{
    bytes: GenericArray<u8, BC>
}

impl<BC: BitSetLength> BitSet<BC> {
    fn new() -> Self {
        BitSet {
            bytes: GenericArray::generate(|_| 0)
        }
    }

    fn as_slice(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    fn as_mut_slice(&self) -> &mut[u8] {
        self.bytes.as_mut_slice()
    }
}
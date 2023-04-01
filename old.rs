use std::fmt::Debug;

#[derive(Debug)] // I am not able to expand this derive.
pub struct ConstGeneric<const CHUNK_SIZE: usize> {
    _p: [(); CHUNK_SIZE],
}

/////// MOVE START
impl<const CHUNK_SIZE: usize> ConstGeneric<CHUNK_SIZE> {}

struct Empty {}

impl core::fmt::Debug for Empty {
    fn fmt(&self, _: &mut core::fmt::Formatter) -> core::fmt::Result {
        loop {}
    }
}
/////// MOVE END

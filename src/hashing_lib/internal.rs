pub use blake2::digest::{
    generic_array::{sequence::Split, typenum, ArrayLength, GenericArray},
    Digest, FixedOutputReset, OutputSizeUser, Reset,
};

#[derive(Clone)]
pub struct InternalStreamHasher<D: Digest + Reset + FixedOutputReset> {
    hasher: D,
}

impl<D: Digest + Reset + FixedOutputReset> InternalStreamHasher<D> {
    pub fn new() -> Self {
        Self { hasher: D::new() }
    }

    pub fn write<T: AsRef<[u8]>>(&mut self, in_bytes: T) {
        Digest::update(&mut self.hasher, in_bytes);
    }

    pub fn reset(&mut self) {
        Digest::reset(&mut self.hasher)
    }

    pub fn finalize(&mut self) -> GenericArray<u8, <D as OutputSizeUser>::OutputSize> {
        self.hasher.finalize_reset()
    }
}

use std::num::{NonZeroU64, NonZeroUsize};

use k12::digest::typenum::Unsigned;

use crate::hashing_lib::{sized_hasher::SizedHasher, unsized_hasher::UnsizedHasher};

pub struct GenericSizedHasher<H: SizedHasher> {
    hasher: H,
    iters: NonZeroU64,
}

impl<H: SizedHasher> GenericSizedHasher<H> {
    pub fn new(iters: NonZeroU64) -> Self {
        Self {
            hasher: H::new(),
            iters,
        }
    }

    pub fn write(&mut self, data: &[u8]) -> &mut Self {
        self.hasher.write(data.as_ref());
        self
    }

    pub fn finalize_and_reset(&mut self) -> Vec<u8> {
        let mut hash = self.hasher.finalize_and_reset();
        // We hashed the data once, let's do the remaining iterations
        for _ in 0..self.iters.get() - 1 {
            self.hasher.write(hash);
            hash = self.hasher.finalize_and_reset();
        }
        hash.to_vec()
    }

    pub fn output_size(&self) -> NonZeroUsize {
        <H::OutputSize as Unsigned>::USIZE
            .try_into()
            .expect("Sized cannot be zero or less")
    }
}

pub struct GenericUnsizedHasher<H: UnsizedHasher> {
    hasher: H,
    iters: NonZeroU64,
}

impl<H: UnsizedHasher> GenericUnsizedHasher<H> {
    pub fn new(output_size: NonZeroUsize, iters: NonZeroU64) -> Self {
        Self {
            hasher: H::new(output_size),
            iters,
        }
    }

    pub fn write(&mut self, data: &[u8]) -> &mut Self {
        self.hasher.write(data.as_ref());
        self
    }

    pub fn finalize_and_reset(&mut self) -> Vec<u8> {
        let mut hash = self.hasher.finalize_and_reset();
        // We hashed the data once, let's do the remaining iterations
        for _ in 0..self.iters.get() - 1 {
            self.hasher.write(hash);
            hash = self.hasher.finalize_and_reset();
        }
        hash.to_vec()
    }

    pub fn output_size(&self) -> NonZeroUsize {
        self.hasher.output_size()
    }
}

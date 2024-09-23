use std::num::NonZeroU64;

use hashing_lib::Hasher;

pub struct GenericHasher<H: Hasher> {
    hasher: H,
    iters: NonZeroU64,
}

impl<H: Hasher> GenericHasher<H> {
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

    pub fn finalize(&mut self) -> Vec<u8> {
        let mut hash = self.hasher.finalize();
        // We hashed the data once, let's do the remaining iterations
        for _ in 0..self.iters.get() - 1 {
            self.hasher.reset();
            self.hasher.write(hash);
            hash = self.hasher.finalize();
        }
        self.hasher.reset();
        hash.to_vec()
    }
}

pub trait DynHasher {
    fn write(&mut self, data: &[u8]);

    fn finalize(&mut self) -> Vec<u8>;
}

impl<H: Hasher> DynHasher for GenericHasher<H> {
    fn write(&mut self, data: &[u8]) {
        self.write(data);
    }

    fn finalize(&mut self) -> Vec<u8> {
        self.finalize()
    }
}

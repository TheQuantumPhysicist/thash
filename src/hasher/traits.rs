use std::{
    fmt::Debug,
    num::NonZeroUsize,
    ops::{Deref, DerefMut},
};

use crate::hashing_lib::{sized_hasher::SizedHasher, unsized_hasher::UnsizedHasher};

use super::generic::{GenericSizedHasher, GenericUnsizedHasher};

impl<H: SizedHasher> DynHasher for GenericSizedHasher<H> {
    fn algorithm_name(&self) -> &'static str {
        H::algorithm_name()
    }

    fn write(&mut self, data: &[u8]) {
        self.write(data);
    }

    fn finalize_and_reset(&mut self) -> Vec<u8> {
        self.finalize_and_reset()
    }

    fn output_size(&self) -> NonZeroUsize {
        self.output_size()
    }
}

impl<T: Deref + DerefMut> DynHasher for T
where
    T::Target: DynHasher,
{
    fn algorithm_name(&self) -> &'static str {
        self.deref().algorithm_name()
    }

    fn write(&mut self, data: &[u8]) {
        self.deref_mut().write(data)
    }

    fn finalize_and_reset(&mut self) -> Vec<u8> {
        self.deref_mut().finalize_and_reset()
    }

    fn output_size(&self) -> NonZeroUsize {
        self.deref().output_size()
    }
}

pub trait DynHasher {
    fn algorithm_name(&self) -> &'static str;

    fn write(&mut self, data: &[u8]);

    fn finalize_and_reset(&mut self) -> Vec<u8>;

    fn output_size(&self) -> NonZeroUsize;
}

impl Debug for dyn DynHasher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("<{} hasher>", self.algorithm_name()).fmt(f)
    }
}

impl<H: UnsizedHasher> DynHasher for GenericUnsizedHasher<H> {
    fn algorithm_name(&self) -> &'static str {
        H::algorithm_name()
    }

    fn write(&mut self, data: &[u8]) {
        self.write(data);
    }

    fn finalize_and_reset(&mut self) -> Vec<u8> {
        self.finalize_and_reset()
    }

    fn output_size(&self) -> NonZeroUsize {
        self.output_size()
    }
}

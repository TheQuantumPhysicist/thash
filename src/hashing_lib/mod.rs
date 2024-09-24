mod internal;

use internal::{typenum, ArrayLength, GenericArray, InternalStreamHasher, Split};

pub trait Hasher {
    type OutputSize: ArrayLength<u8>;

    fn new() -> Self;

    fn write<T: AsRef<[u8]>>(&mut self, in_bytes: T) -> &mut Self;

    fn reset(&mut self);

    fn finalize(&mut self) -> GenericArray<u8, Self::OutputSize>;
}

macro_rules! impl_hasher_stream {
    ($stream_type:ident, $stream_size:ty) => {
        impl Hasher for $stream_type {
            type OutputSize = $stream_size;

            fn new() -> Self {
                Self(InternalStreamHasher::new())
            }

            fn write<T: AsRef<[u8]>>(&mut self, in_bytes: T) -> &mut Self {
                self.0.write(in_bytes);
                self
            }

            fn finalize(&mut self) -> GenericArray<u8, Self::OutputSize> {
                self.0.finalize().split().0
            }

            fn reset(&mut self) {
                self.0.reset()
            }
        }
    };
}

#[derive(Clone)]
pub struct Blake2b(InternalStreamHasher<blake2::Blake2b<typenum::U64>>);
#[derive(Clone)]
pub struct Blake2s(InternalStreamHasher<blake2::Blake2s<typenum::U32>>);
#[derive(Clone)]
pub struct Md5(InternalStreamHasher<md5::Md5>);
#[derive(Clone)]
pub struct Sha1(InternalStreamHasher<sha1::Sha1>);
#[derive(Clone)]
pub struct Sha224(InternalStreamHasher<sha2::Sha224>);
#[derive(Clone)]
pub struct Sha256(InternalStreamHasher<sha2::Sha256>);
#[derive(Clone)]
pub struct Sha384(InternalStreamHasher<sha2::Sha384>);
#[derive(Clone)]
pub struct Sha512(InternalStreamHasher<sha2::Sha512>);
#[derive(Clone)]
pub struct Sha3_224(InternalStreamHasher<sha3::Sha3_224>);
#[derive(Clone)]
pub struct Sha3_256(InternalStreamHasher<sha3::Sha3_256>);
#[derive(Clone)]
pub struct Sha3_384(InternalStreamHasher<sha3::Sha3_384>);
#[derive(Clone)]
pub struct Sha3_512(InternalStreamHasher<sha3::Sha3_512>);

impl_hasher_stream!(Blake2b, typenum::U64);
impl_hasher_stream!(Blake2s, typenum::U32);
impl_hasher_stream!(Md5, typenum::U16);
impl_hasher_stream!(Sha1, typenum::U20);
impl_hasher_stream!(Sha224, typenum::U28);
impl_hasher_stream!(Sha256, typenum::U32);
impl_hasher_stream!(Sha384, typenum::U48);
impl_hasher_stream!(Sha512, typenum::U64);
impl_hasher_stream!(Sha3_224, typenum::U28);
impl_hasher_stream!(Sha3_256, typenum::U32);
impl_hasher_stream!(Sha3_384, typenum::U48);
impl_hasher_stream!(Sha3_512, typenum::U64);

#[derive(Clone)]
pub struct Blake3 {
    hasher: blake3::Hasher,
}

impl Hasher for Blake3 {
    type OutputSize = typenum::U32;

    fn new() -> Self {
        Self {
            hasher: blake3::Hasher::new(),
        }
    }

    fn write<T: AsRef<[u8]>>(&mut self, in_bytes: T) -> &mut Self {
        self.hasher.update(in_bytes.as_ref());
        self
    }

    fn reset(&mut self) {
        self.hasher.reset();
    }

    fn finalize(&mut self) -> GenericArray<u8, Self::OutputSize> {
        let result = self.hasher.finalize();
        let array: [u8; 32] = result.into();
        array.into()
    }
}

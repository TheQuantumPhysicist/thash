use super::internal::{typenum, ArrayLength, GenericArray, InternalStreamHasher, Split};

pub trait SizedHasher: Sized {
    type OutputSize: ArrayLength<u8>;

    fn algorithm_name() -> &'static str;

    fn new() -> Self;

    fn write<T: AsRef<[u8]>>(&mut self, in_bytes: T) -> &mut Self;

    fn reset(&mut self);

    fn finalize_and_reset(&mut self) -> GenericArray<u8, Self::OutputSize>;
}

macro_rules! impl_hasher_stream {
    ($stream_type:ident, $stream_size:ty, $algo_name:expr) => {
        impl SizedHasher for $stream_type {
            type OutputSize = $stream_size;

            fn algorithm_name() -> &'static str {
                $algo_name
            }

            fn new() -> Self {
                Self(InternalStreamHasher::new())
            }

            fn write<T: AsRef<[u8]>>(&mut self, in_bytes: T) -> &mut Self {
                self.0.write(in_bytes);
                self
            }

            fn finalize_and_reset(&mut self) -> GenericArray<u8, Self::OutputSize> {
                let result: GenericArray<u8, Self::OutputSize> = self.0.finalize().split().0;
                self.reset();
                result
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

impl_hasher_stream!(Blake2b, typenum::U64, "Blake2b");
impl_hasher_stream!(Blake2s, typenum::U32, "Blake2s");
impl_hasher_stream!(Md5, typenum::U16, "Md5");
impl_hasher_stream!(Sha1, typenum::U20, "Sha1");
impl_hasher_stream!(Sha224, typenum::U28, "Sha224");
impl_hasher_stream!(Sha256, typenum::U32, "Sha256");
impl_hasher_stream!(Sha384, typenum::U48, "Sha384");
impl_hasher_stream!(Sha512, typenum::U64, "Sha512");
impl_hasher_stream!(Sha3_224, typenum::U28, "Sha3-224");
impl_hasher_stream!(Sha3_256, typenum::U32, "Sha3-256");
impl_hasher_stream!(Sha3_384, typenum::U48, "Sha3-384");
impl_hasher_stream!(Sha3_512, typenum::U64, "Sha3-512");

#[derive(Clone)]
pub struct Blake3 {
    hasher: blake3::Hasher,
}

impl SizedHasher for Blake3 {
    type OutputSize = typenum::U32;

    fn algorithm_name() -> &'static str {
        "Blake3"
    }

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

    fn finalize_and_reset(&mut self) -> GenericArray<u8, Self::OutputSize> {
        let result = self.hasher.finalize();
        self.reset();
        let array: [u8; 32] = result.into();
        array.into()
    }
}

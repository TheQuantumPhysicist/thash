mod internal;

use internal::{typenum, ArrayLength, GenericArray, InternalStreamHasher, Split};

pub trait Hasher: Sized {
    type OutputSize: ArrayLength<u8>;

    fn new() -> Self;

    fn write<T: AsRef<[u8]>>(&mut self, in_bytes: T) -> &mut Self;

    fn reset(&mut self);

    fn finalize_and_reset(&mut self) -> GenericArray<u8, Self::OutputSize>;
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

    fn finalize_and_reset(&mut self) -> GenericArray<u8, Self::OutputSize> {
        let result = self.hasher.finalize();
        self.reset();
        let array: [u8; 32] = result.into();
        array.into()
    }
}

pub struct K12<'a> {
    hasher: Option<k12::KangarooTwelve<'a>>,
}

impl<'a> K12<'a> {
    fn hasher(&mut self) -> &mut k12::KangarooTwelve<'a> {
        // An invariant is used that the value is always Some(), as long as the object is alive
        debug_assert!(self.hasher.is_some());
        unsafe { self.hasher.as_mut().unwrap_unchecked() }
    }

    fn finalize_and_reset_inner(&mut self) -> GenericArray<u8, <Self as Hasher>::OutputSize> {
        use k12::digest::ExtendableOutput;
        const EXPECTED_SIZE: usize = 32;

        let result: Box<[u8]> = self
            .hasher
            .take()
            .expect("Must exist")
            .finalize_boxed(EXPECTED_SIZE);

        let result = result.as_ref();

        let array: [u8; EXPECTED_SIZE] = result.try_into().expect("Cannot fail");

        // Reset the hasher, to maintain the invariant that hasher is Some
        self.hasher = Some(k12::KangarooTwelve::default());

        array.into()
    }
}

impl<'a> Hasher for K12<'a> {
    type OutputSize = typenum::U32;

    fn new() -> Self {
        Self {
            hasher: Some(k12::KangarooTwelve::default()),
        }
    }

    fn write<T: AsRef<[u8]>>(&mut self, in_bytes: T) -> &mut Self {
        use k12::digest::Update;
        self.hasher().update(in_bytes.as_ref());
        self
    }

    fn reset(&mut self) {
        use k12::digest::Reset;
        self.hasher().reset();
    }

    fn finalize_and_reset(&mut self) -> GenericArray<u8, Self::OutputSize> {
        self.finalize_and_reset_inner()
    }
}

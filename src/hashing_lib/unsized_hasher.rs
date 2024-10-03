use std::num::NonZeroUsize;

pub trait UnsizedHasher: Sized {
    fn algorithm_name() -> &'static str;

    fn new(output_size: NonZeroUsize) -> Self;

    fn output_size(&self) -> NonZeroUsize;

    fn write<T: AsRef<[u8]>>(&mut self, in_bytes: T) -> &mut Self;

    fn finalize_and_reset(&mut self) -> Box<[u8]>;
}

pub struct K12<'a> {
    hasher: Option<k12::KangarooTwelve<'a>>,
    output_size: NonZeroUsize,
}

impl<'a> K12<'a> {
    fn hasher(&mut self) -> &mut k12::KangarooTwelve<'a> {
        // An invariant is used that the value is always Some(), as long as the object is alive
        debug_assert!(self.hasher.is_some());
        unsafe { self.hasher.as_mut().unwrap_unchecked() }
    }

    fn finalize_and_reset_inner(&mut self) -> Box<[u8]> {
        use k12::digest::ExtendableOutput;

        let result = self
            .hasher
            .take()
            .expect("Must exist")
            .finalize_boxed(self.output_size.get());

        // Reset the hasher, to maintain the invariant that hasher is Some
        self.hasher = Some(k12::KangarooTwelve::default());

        result
    }
}

impl<'a> UnsizedHasher for K12<'a> {
    fn algorithm_name() -> &'static str {
        "K12"
    }

    fn new(output_size: NonZeroUsize) -> Self {
        Self {
            hasher: Some(k12::KangarooTwelve::default()),
            output_size,
        }
    }

    fn write<T: AsRef<[u8]>>(&mut self, in_bytes: T) -> &mut Self {
        use k12::digest::Update;
        self.hasher().update(in_bytes.as_ref());
        self
    }

    fn finalize_and_reset(&mut self) -> Box<[u8]> {
        self.finalize_and_reset_inner()
    }

    fn output_size(&self) -> NonZeroUsize {
        self.output_size
    }
}

mod generic;

use std::{collections::BTreeMap, num::NonZeroU64};

use crate::hashing_lib::{
    Blake2b, Blake2s, Blake3, Md5, Sha1, Sha224, Sha256, Sha384, Sha3_224, Sha3_256, Sha3_384,
    Sha3_512, Sha512,
};
use generic::{DynHasher, GenericHasher};

use crate::options::HashAlgorithm;

pub fn make_hasher(
    algo: HashAlgorithm,
    iters: NonZeroU64,
    options: &BTreeMap<String, String>,
) -> anyhow::Result<Box<dyn DynHasher>> {
    let f: Box<dyn DynHasher> = match algo {
        HashAlgorithm::Blake2b => Box::new(make_blake2b_hasher(options, iters)?),
        HashAlgorithm::Blake2s => Box::new(make_blake2s_hasher(options, iters)?),
        HashAlgorithm::Blake3 => Box::new(make_blake3_hasher(options, iters)?),
        HashAlgorithm::Md5 => Box::new(make_md5_hasher(options, iters)?),
        HashAlgorithm::Sha1 => Box::new(make_sha1_hasher(options, iters)?),
        HashAlgorithm::Sha224 => Box::new(make_sha224_hasher(options, iters)?),
        HashAlgorithm::Sha256 => Box::new(make_sha256_hasher(options, iters)?),
        HashAlgorithm::Sha384 => Box::new(make_sha384_hasher(options, iters)?),
        HashAlgorithm::Sha512 => Box::new(make_sha512_hasher(options, iters)?),
        HashAlgorithm::Sha3_224 => Box::new(make_sha3_224_hasher(options, iters)?),
        HashAlgorithm::Sha3_256 => Box::new(make_sha3_256_hasher(options, iters)?),
        HashAlgorithm::Sha3_384 => Box::new(make_sha3_384_hasher(options, iters)?),
        HashAlgorithm::Sha3_512 => Box::new(make_sha3_512_hasher(options, iters)?),
    };

    Ok(f)
}

fn make_sha1_hasher(
    options: &BTreeMap<String, String>,
    iters: NonZeroU64,
) -> anyhow::Result<GenericHasher<Sha1>> {
    ensure_empty_options(options)?;

    Ok(GenericHasher::new(iters))
}

fn make_sha224_hasher(
    options: &BTreeMap<String, String>,
    iters: NonZeroU64,
) -> anyhow::Result<GenericHasher<Sha224>> {
    ensure_empty_options(options)?;

    Ok(GenericHasher::new(iters))
}

fn make_sha256_hasher(
    options: &BTreeMap<String, String>,
    iters: NonZeroU64,
) -> anyhow::Result<GenericHasher<Sha256>> {
    ensure_empty_options(options)?;

    Ok(GenericHasher::new(iters))
}

fn make_sha384_hasher(
    options: &BTreeMap<String, String>,
    iters: NonZeroU64,
) -> anyhow::Result<GenericHasher<Sha384>> {
    ensure_empty_options(options)?;

    Ok(GenericHasher::new(iters))
}

fn make_sha512_hasher(
    options: &BTreeMap<String, String>,
    iters: NonZeroU64,
) -> anyhow::Result<GenericHasher<Sha512>> {
    ensure_empty_options(options)?;

    Ok(GenericHasher::new(iters))
}

fn make_sha3_224_hasher(
    options: &BTreeMap<String, String>,
    iters: NonZeroU64,
) -> anyhow::Result<GenericHasher<Sha3_224>> {
    ensure_empty_options(options)?;

    Ok(GenericHasher::new(iters))
}

fn make_sha3_256_hasher(
    options: &BTreeMap<String, String>,
    iters: NonZeroU64,
) -> anyhow::Result<GenericHasher<Sha3_256>> {
    ensure_empty_options(options)?;

    Ok(GenericHasher::new(iters))
}

fn make_sha3_384_hasher(
    options: &BTreeMap<String, String>,
    iters: NonZeroU64,
) -> anyhow::Result<GenericHasher<Sha3_384>> {
    ensure_empty_options(options)?;

    Ok(GenericHasher::new(iters))
}

fn make_sha3_512_hasher(
    options: &BTreeMap<String, String>,
    iters: NonZeroU64,
) -> anyhow::Result<GenericHasher<Sha3_512>> {
    ensure_empty_options(options)?;

    Ok(GenericHasher::new(iters))
}

fn make_blake2b_hasher(
    options: &BTreeMap<String, String>,
    iters: NonZeroU64,
) -> anyhow::Result<GenericHasher<Blake2b>> {
    ensure_empty_options(options)?;

    Ok(GenericHasher::new(iters))
}

fn make_blake2s_hasher(
    options: &BTreeMap<String, String>,
    iters: NonZeroU64,
) -> anyhow::Result<GenericHasher<Blake2s>> {
    ensure_empty_options(options)?;

    Ok(GenericHasher::new(iters))
}

fn make_blake3_hasher(
    options: &BTreeMap<String, String>,
    iters: NonZeroU64,
) -> anyhow::Result<GenericHasher<Blake3>> {
    ensure_empty_options(options)?;

    Ok(GenericHasher::new(iters))
}

fn make_md5_hasher(
    options: &BTreeMap<String, String>,
    iters: NonZeroU64,
) -> anyhow::Result<GenericHasher<Md5>> {
    ensure_empty_options(options)?;

    Ok(GenericHasher::new(iters))
}

fn ensure_empty_options(opts: &BTreeMap<String, String>) -> anyhow::Result<()> {
    if !opts.is_empty() {
        return Err(anyhow::anyhow!(
            "The selected algorithm does not have any options"
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests;

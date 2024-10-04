mod hasher;
mod hashing_lib;
mod program_options;

use std::io::{Read, Write};

use anyhow::Context;
use clap::Parser;
use hasher::{make_hasher, traits::DynHasher};
use program_options::OutputFormat;

fn main() -> anyhow::Result<()> {
    let args: program_options::THashOptions = program_options::THashOptions::parse();

    let options = args.options()?;

    let hash_digest = {
        let mut hasher = make_hasher(args.hash_algo, args.iters(), options)?;

        match args.file {
            Some(f) => {
                let reader = open_file(f)?;
                buffer_into_hasher(&mut hasher, reader)?;
            }
            None => {
                let stdin = std::io::stdin();
                buffer_into_hasher(&mut hasher, stdin.lock())?
            }
        }
        hasher.finalize_and_reset()
    };

    {
        let stdout = std::io::stdout();
        let mut stdout_handle = stdout.lock();
        let output = convert_output(hash_digest, args.output_format);
        stdout_handle
            .write_all(&output)
            .expect("Writing result to stdout failing");
    }

    Ok(())
}

fn open_file(p: impl AsRef<std::path::Path>) -> anyhow::Result<impl Read> {
    let p = p.as_ref();
    if !p.exists() {
        return Err(anyhow::anyhow!("File not found: {}", p.display()));
    }

    if !p.is_file() {
        return Err(anyhow::anyhow!(
            "Path provided is not a file or unreadable: {}",
            p.display()
        ));
    }

    let f = std::fs::File::open(p).context(format!("Opening file failed: {}", p.display()))?;

    Ok(f)
}

fn buffer_into_hasher(hasher: &mut impl DynHasher, mut source: impl Read) -> anyhow::Result<()> {
    let mut buffer = [0; 4096];

    loop {
        let bytes_read = source.read(&mut buffer)?;

        if bytes_read == 0 {
            break Ok(());
        }

        hasher.write(&buffer[..bytes_read]);
    }
}

fn convert_output(output: Vec<u8>, output_format: OutputFormat) -> Vec<u8> {
    use base64::prelude::*;

    match output_format {
        OutputFormat::Binary => output, // We don't add a new line to binary mode
        OutputFormat::HexLower => newlined(hex::encode(&output)).as_bytes().to_vec(),
        OutputFormat::HexUpper => newlined(hex::encode_upper(&output)).as_bytes().to_vec(),
        OutputFormat::Base64 => newlined(BASE64_STANDARD.encode(output)).as_bytes().to_vec(),
        OutputFormat::Base64NoPad => newlined(BASE64_STANDARD_NO_PAD.encode(output))
            .as_bytes()
            .to_vec(),
        OutputFormat::Base64UrlSafe => newlined(BASE64_URL_SAFE.encode(output)).as_bytes().to_vec(),
        OutputFormat::Base64UrlSafeNoPad => newlined(BASE64_URL_SAFE_NO_PAD.encode(output))
            .as_bytes()
            .to_vec(),
    }
}

/// Adds a new line to an owned string
fn newlined(s: impl Into<String>) -> String {
    let s = s.into();
    format!("{}\n", s)
}

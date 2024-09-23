mod hasher;
mod options;

use std::io::{Read, Write};

use clap::Parser;
use hasher::make_hasher;
use options::OutputFormat;

fn main() -> anyhow::Result<()> {
    let args: options::THashOptions = options::THashOptions::parse();

    let options = args.options()?;

    let mut hasher = make_hasher(args.hash_algo, args.iters(), &options)?;

    {
        let mut buffer = [0; 4096];
        let stdin = std::io::stdin();
        let mut stdin_handle = stdin.lock();

        loop {
            let bytes_read = stdin_handle.read(&mut buffer)?;

            if bytes_read == 0 {
                break;
            }

            hasher.write(&buffer[..bytes_read]);
        }
    }
    let hash_digest = hasher.finalize();

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

fn convert_output(output: Vec<u8>, output_format: OutputFormat) -> Vec<u8> {
    use base64::prelude::*;

    match output_format {
        OutputFormat::Binary => output,
        OutputFormat::HexLower => hex::encode(&output).as_bytes().to_vec(),
        OutputFormat::HexUpper => hex::encode_upper(&output).as_bytes().to_vec(),
        OutputFormat::Base64 => BASE64_STANDARD.encode(output).as_bytes().to_vec(),
        OutputFormat::Base64NoPad => BASE64_STANDARD_NO_PAD.encode(output).as_bytes().to_vec(),
        OutputFormat::Base64UrlSafe => BASE64_URL_SAFE.encode(output).as_bytes().to_vec(),
        OutputFormat::Base64UrlSafeNoPad => {
            BASE64_URL_SAFE_NO_PAD.encode(output).as_bytes().to_vec()
        }
    }
}

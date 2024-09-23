mod hasher;
mod options;

use std::io::Read;

use clap::Parser;
use hasher::make_hasher;

fn main() -> anyhow::Result<()> {
    let args: options::THashOptions = options::THashOptions::parse();

    let options = args.options()?;

    let mut hasher = make_hasher(args.hash_algo, args.iters(), &options)?;

    let mut buffer = [0; 4096];
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();

    loop {
        let bytes_read = handle.read(&mut buffer)?;

        if bytes_read == 0 {
            break;
        }

        hasher.write(&buffer[..bytes_read]);
    }

    println!("{}", hex::encode(hasher.finalize()));

    Ok(())
}

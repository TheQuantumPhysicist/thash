use std::{collections::BTreeMap, num::NonZeroU64};

use clap::{Parser, ValueEnum};

#[derive(Parser, Clone, Debug, Default)]
pub struct THashOptions {
    /// The algorithm that will be used to hash the given data.
    #[arg(long, short('a'), value_name("ALGORITHM"), default_value_t = HashAlgorithm::default())]
    pub hash_algo: HashAlgorithm,

    /// The number of iterations for hashing the data, recursively.
    /// This works by using the output of a hash function (as bytes) as input for the same hash function.
    #[arg(long, short('i'), value_name("NUMBER"), default_value_t = 1, value_parser=parse_iters)]
    iters: u64,

    /// Output format. The result will go into stdout.
    #[arg(long, short('F'), default_value_t = OutputFormat::default())]
    pub output_format: OutputFormat,

    /// An optional path of the source file to read, in case you do not want to use stdin.
    /// If not provided, the program expects to get data from stdin.
    #[arg(long, short('f'))]
    pub file: Option<std::path::PathBuf>,

    /// Options related to hashing algorithms. Some algorithms provide options (or parameters)
    /// related to their hashing. These can be set here.
    #[arg(
        short = 'o',
        long = "option",
        action = clap::ArgAction::Append,
        number_of_values(1)
    )]
    options: Vec<String>,
}

impl THashOptions {
    pub fn options(&self) -> anyhow::Result<BTreeMap<String, String>> {
        parse_options(&self.options)
    }

    pub fn iters(&self) -> NonZeroU64 {
        NonZeroU64::new(self.iters).expect("Already checked iters > 0 while parsing")
    }
}

fn parse_options(opts: &[String]) -> anyhow::Result<BTreeMap<String, String>> {
    let mut result = BTreeMap::new();
    for item in opts.iter() {
        let (key, val) = item.split_once('=').ok_or(anyhow::anyhow!(
            "In options, key {item} provided with no value"
        ))?;
        if let Some(old_key) = result.insert(key.to_string(), val.to_string()) {
            return Err(anyhow::anyhow!(
                "Option `{old_key} appeared more than once.`"
            ));
        }
    }

    Ok(result)
}

fn parse_iters(value: &str) -> anyhow::Result<u64> {
    let v = value
        .parse::<u64>()
        .map_err(|e| anyhow::anyhow!("Number of iterations must be a number: {e}"))?;
    let result = NonZeroU64::new(v)
        .ok_or_else(|| anyhow::anyhow!("Number of iterations must be a positive number"))?;
    Ok(result.get())
}

#[derive(ValueEnum, Debug, Clone, Default, Copy, strum_macros::Display)]
#[strum(serialize_all = "kebab-case")]
#[clap(rename_all = "kebab_case")]
pub enum HashAlgorithm {
    #[default]
    Blake2b,
    Blake2s,
    Blake3,
    K12,
    Md5,
    Sha1,
    Sha224,
    Sha256,
    Sha384,
    Sha512,
    Sha3_224,
    Sha3_256,
    Sha3_384,
    Sha3_512,
}

#[derive(ValueEnum, Debug, Clone, Default, Copy, strum_macros::Display)]
#[strum(serialize_all = "kebab-case")]
#[clap(rename_all = "kebab_case")]
pub enum OutputFormat {
    Binary,
    #[default]
    HexLower,
    HexUpper,
    Base64,
    Base64NoPad,
    Base64UrlSafe,
    Base64UrlSafeNoPad,
}

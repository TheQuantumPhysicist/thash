use std::{collections::BTreeMap, str::FromStr};

use anyhow::Context;

pub const OUTPUT_SIZE_KEY: &str = "output-size";

pub fn parse_option<T: FromStr>(
    options: &BTreeMap<String, String>,
    option_key: impl AsRef<str>,
    default_value: T,
) -> anyhow::Result<T>
where
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    let parsed: T = options
        .get(option_key.as_ref())
        .cloned()
        .map(|s| s.parse())
        .transpose()
        .context(format!("While parsing option `{}`", option_key.as_ref()))?
        .unwrap_or(default_value);

    Ok(parsed)
}

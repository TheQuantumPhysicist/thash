use std::{collections::BTreeMap, num::NonZeroUsize};

use super::{
    common::{parse_option, OUTPUT_SIZE_KEY},
    traits::HashingOptions,
};

pub const DEFAULT_OUTPUT_SIZE: NonZeroUsize = match NonZeroUsize::new(32) {
    Some(v) => v,
    None => unreachable!(),
};

pub struct K12Options {
    pub output_size: NonZeroUsize,
}

impl TryFrom<BTreeMap<String, String>> for K12Options {
    type Error = anyhow::Error;

    fn try_from(options: BTreeMap<String, String>) -> Result<Self, Self::Error> {
        let output_size =
            parse_option::<NonZeroUsize>(&options, OUTPUT_SIZE_KEY, DEFAULT_OUTPUT_SIZE)?;

        Ok(Self { output_size })
    }
}

impl HashingOptions for K12Options {
    fn options_descriptions() -> BTreeMap<String, String> {
        [(
            OUTPUT_SIZE_KEY.to_string(),
            "The size of the output as a positive integer".to_string(),
        )]
        .into_iter()
        .collect()
    }

    fn algo_name() -> &'static str {
        "K12"
    }
}

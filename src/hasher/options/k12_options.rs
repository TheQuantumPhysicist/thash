use std::{collections::BTreeMap, num::NonZeroUsize, rc::Rc};

use anyhow::Context;

use super::{common::OUTPUT_SIZE_KEY, traits::HashingOptions};

pub const DEFAULT_OUTPUT_SIZE: usize = 32;

pub struct K12Options {
    pub output_size: NonZeroUsize,
}

impl TryFrom<Rc<BTreeMap<String, String>>> for K12Options {
    type Error = anyhow::Error;

    fn try_from(options: Rc<BTreeMap<String, String>>) -> Result<Self, Self::Error> {
        let output_size: NonZeroUsize = options
            .get(OUTPUT_SIZE_KEY)
            .cloned()
            .unwrap_or(DEFAULT_OUTPUT_SIZE.to_string())
            .parse()
            .context("While parsing output size")?;

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

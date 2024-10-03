use std::{collections::BTreeMap, num::NonZeroUsize, rc::Rc};

use anyhow::Context;

pub const OUTPUT_SIZE_KEY: &str = "output-size";
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

pub trait HashingOptions: TryFrom<Rc<BTreeMap<String, String>>, Error = anyhow::Error> {
    fn algo_name() -> &'static str;
    fn options_descriptions() -> BTreeMap<String, String>;
    fn options_descriptions_table() -> String {
        use prettytable::Table;

        let table = {
            let mut table = Table::new();
            table.add_row(prettytable::row![
                "Option name".to_string(),
                "Description".to_string()
            ]);
            table
        };

        let table =
            Self::options_descriptions()
                .into_iter()
                .fold(table, |mut so_far, (curr_k, curr_v)| {
                    so_far.add_row(prettytable::row![curr_k, curr_v]);
                    so_far
                });

        table.to_string()
    }

    fn parse(options: Rc<BTreeMap<String, String>>) -> anyhow::Result<Self> {
        let options_desc = Self::options_descriptions();

        for option_key in options.keys() {
            if !options_desc.contains_key(option_key) {
                return Err(anyhow::anyhow!(
                    "Option {option_key} is not a valid option for algorithm {}. Available options:\n\n{}",
                    Self::algo_name(),
                    Self::options_descriptions_table()
                ));
            }
        }

        let result = <Self as TryFrom<Rc<BTreeMap<String, String>>>>::try_from(options)?;

        Ok(result)
    }
}

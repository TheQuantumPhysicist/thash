use std::collections::BTreeMap;

pub trait HashingOptions: TryFrom<BTreeMap<String, String>, Error = anyhow::Error> {
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

    fn parse(options: BTreeMap<String, String>) -> anyhow::Result<Self> {
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

        let result = <Self as TryFrom<BTreeMap<String, String>>>::try_from(options)?;

        Ok(result)
    }
}

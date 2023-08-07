use crate::bdf::BusDeviceFunction;
use clap::{Arg, ArgMatches, Command};
use std::str::FromStr;

pub struct Parser {
    matches: ArgMatches,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            matches: Command::new("lspci")
                .arg(
                    Arg::new("slot")
                        .short('s')
                        .help(format!(
                            "{} {}",
                            BusDeviceFunction::FORMAT,
                            "Show only devices in selected slots"
                        ))
                        .value_parser(BusDeviceFunction::from_str)
                        .action(clap::ArgAction::Append),
                )
                .get_matches(),
        }
    }

    pub fn slots(&self) -> Option<Vec<&BusDeviceFunction>> {
        self.matches
            .get_many::<BusDeviceFunction>("slot")
            .map(|s| s.collect())
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

use crate::{bdf::BusDeviceFunction, function::Function};
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

    pub fn filter_slots(&self, functions: Vec<Function>) -> Vec<Function> {
        match self.matches.get_many::<BusDeviceFunction>("slot") {
            Some(mut slots) => functions
                .into_iter()
                .filter(|function| slots.any(|slot| function.bdf == *slot))
                .collect(),
            None => functions,
        }
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

use crate::{bdf::BusDeviceFunction, vdc::VendorDeviceClass};
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
                            "{}\t{}",
                            BusDeviceFunction::FORMAT,
                            "Show only devices in selected slots"
                        ))
                        .value_parser(BusDeviceFunction::from_str)
                        .action(clap::ArgAction::Append),
                )
                .arg(
                    Arg::new("id")
                        .short('d')
                        .help(format!(
                            "{}\t{}",
                            VendorDeviceClass::FORMAT,
                            "Show only devices with specified ID's"
                        ))
                        .value_parser(VendorDeviceClass::from_str)
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

    pub fn ids(&self) -> Option<Vec<&VendorDeviceClass>> {
        self.matches
            .get_many::<VendorDeviceClass>("id")
            .map(|id| id.collect())
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

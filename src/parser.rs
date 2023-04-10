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
                            "{} {}",
                            "Show only devices in selected slots",
                            BusDeviceFunction::FORMAT,
                        ))
                        .value_parser(BusDeviceFunction::from_str)
                        .action(clap::ArgAction::Append),
                )
                .arg(
                    Arg::new("id")
                        .short('d')
                        .help(format!(
                            "{} {}",
                            "Show only devices with specified ID's",
                            VendorDeviceClass::FORMAT,
                        ))
                        .value_parser(VendorDeviceClass::from_str)
                        .action(clap::ArgAction::Append),
                )
                .arg(
                    Arg::new("hexdump")
                        .short('x')
                        .help("Show hex-dump of the standard part of the config space".to_string())
                        .action(clap::ArgAction::SetTrue),
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

    pub fn hexdump(&self) -> bool {
        self.matches.get_flag("hexdump")
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

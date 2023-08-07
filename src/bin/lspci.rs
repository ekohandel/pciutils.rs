use pciutils::error::Result;
use pciutils::parser::Parser;
use pciutils::sysfs::Sysfs;

use pretty_hex::{config_hex, HexConfig};

fn main() -> Result<()> {
    env_logger::init();

    let parser = Parser::new();

    let mut functions = Sysfs::discover()?;

    if let Some(slots) = parser.slots() {
        functions.retain(|function| slots.clone().into_iter().any(|slot| *function == *slot))
    }

    if let Some(ids) = parser.ids() {
        functions.retain(|function| ids.clone().into_iter().any(|id| *function == *id))
    }

    let hex_config = HexConfig {
        title: false,
        width: 16,
        group: 0,
        ascii: false,
        ..HexConfig::default()
    };
    for f in functions {
        println!("{}", f);
        if parser.hexdump() {
            println!("{}", config_hex(&f.config()?, hex_config))
        }
    }

    Ok(())
}

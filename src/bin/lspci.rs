use pciutils::error::Result;
use pciutils::parser::Parser;
use pciutils::sysfs::Sysfs;

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

    for f in functions {
        println!("{}", f);
    }

    Ok(())
}

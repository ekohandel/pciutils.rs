use pciutils::error::Result;
use pciutils::parser::Parser;
use pciutils::sysfs::Sysfs;

fn main() -> Result<()> {
    env_logger::init();

    let parser = Parser::new();

    let mut functions = Sysfs::discover()?;
    if let Some(slots) = parser.slots() {
        functions.retain(|f| slots.clone().into_iter().any(|s| f.bdf == *s))
    }

    for f in functions {
        println!("{}", f);
    }

    Ok(())
}

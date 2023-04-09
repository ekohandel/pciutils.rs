use pciutils::error::Result;
use pciutils::parser::Parser;
use pciutils::sysfs::Sysfs;

fn main() -> Result<()> {
    let parser = Parser::new();

    let functions = Sysfs::discover()?;
    let functions = parser.filter_slots(functions);

    for f in functions {
        println!("{}", f);
    }

    Ok(())
}

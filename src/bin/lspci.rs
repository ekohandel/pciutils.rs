use pciutils::error::Result;
use pciutils::sysfs::Sysfs;

fn main() -> Result<()> {
    let functions = Sysfs::discover()?;

    for f in functions {
        println!("{}", f);
    }

    Ok(())
}

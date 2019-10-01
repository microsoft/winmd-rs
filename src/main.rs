// #![allow(unused_variables)]
// #![allow(dead_code)]

mod database;
mod enums;
mod flags;
mod tables;

use database::*;

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}

fn run() -> std::io::Result<()> {
    let db = Database::new(r"c:\windows\system32\winmetadata\Windows.Foundation.winmd")?;

    for type_def in db.type_def() {
        let flags = type_def.flags()?;

        if !flags.windows_runtime() {
            continue;
        }

        if flags.interface() {
            print!("interface");
        }

        println!(" {}.{}", type_def.namespace()?, type_def.name()?);
    }

    Ok(())
}

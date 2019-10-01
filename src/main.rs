// #![allow(unused_variables)]
// #![allow(dead_code)]

mod database;
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
        println!("{}.{}", type_def.namespace()?, type_def.name()?);
    }

    Ok(())
}

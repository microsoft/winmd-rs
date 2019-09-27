// TODO: remove these once Database is working.
#![allow(unused_variables)]
#![allow(dead_code)]

mod database;

use database::*;

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    };
}

fn run() -> std::io::Result<()> {
    let db = Database::new(r"c:\windows\system32\winmetadata\Windows.Foundation.winmd")?;

    for t in db.type_def() {
        println!("{}.{}", t.namespace()?, t.name()?);
    }

    Ok(())
}

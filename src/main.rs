// #![allow(unused_variables)]
// #![allow(dead_code)]

mod database;
mod error;
mod flags;
mod tables;
mod codes;
use database::*;
use tables::*;

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}

fn run() -> std::io::Result<()> {
    let db = Database::new(r"c:\windows\system32\WinMetadata\Windows.Foundation.winmd")?;

    let types = db.type_def();

    let tt = types.row::<TypeDef>(1);

    println!("{}.{}", tt.namespace()?, tt.name()?);

    for t in types.iter::<TypeDef>() {
        println!("{}.{}", t.namespace()?, t.name()?);
    }

    Ok(())
}

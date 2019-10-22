// #![allow(unused_variables)]
// #![allow(dead_code)]

mod codes;
mod database;
mod error;
mod flags;
mod tables;
mod reader;
use database::*;
use tables::*;
use reader::*;

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

    let reader = Reader::from_files(&[r"c:\windows\system32\WinMetadata\Windows.Foundation.winmd"])?;

    if let Some(t) = reader.find("Windows.Foundation", "IStringable")
    {
        println!("{}.{}", t.namespace()?, t.name()?);
    }

    Ok(())
}

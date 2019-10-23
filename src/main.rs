// #![allow(unused_variables)]
// #![allow(dead_code)]

mod codes;
mod database;
mod error;
mod flags;
mod reader;
mod tables;
use database::*;
use reader::*;
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

    let reader = Reader::from_files(&[r"c:\windows\system32\WinMetadata\Windows.Foundation.winmd"])?;

    if let Some(t) = reader.find("Windows.Foundation", "IUriRuntimeClass") {
        println!("{}.{}", t.namespace()?, t.name()?);

        for m in t.methods()? {
            println!("    {}", m.name()?);
        }
    }

    Ok(())
}

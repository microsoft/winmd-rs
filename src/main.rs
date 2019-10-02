// #![allow(unused_variables)]
// #![allow(dead_code)]

mod codes;
mod database;
mod flags;
mod tables;
use database::*;
use tables::*;

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}

fn run() -> std::io::Result<()> {
    let db = Database::new(r"c:\windows\system32\winmetadata\Windows.Foundation.winmd")?;

    // for type_ref in db.type_ref()
    // {
    //     println!(" {}.{}", type_ref.namespace()?, type_ref.name()?);
    // }

    for type_def in db.type_def() {
        let flags = type_def.flags()?;

        if !flags.windows_runtime() {
            continue;
        }

        match type_def.category()? {
            Category::Interface => print!("interface"),
            Category::Class => print!("class"),
            Category::Enum => print!("enum"),
            Category::Struct => print!("struct"),
            Category::Delegate => print!("delegate"),
            Category::Attribute => print!("attribute"),
            Category::Contract => print!("contract"),
        }

        print!(" {}.{}\n", type_def.namespace()?, type_def.name()?);
    }

    Ok(())
}

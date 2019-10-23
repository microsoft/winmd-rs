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
    let reader = Reader::from_os()?;

    if let Some(t) = reader.find("Windows.Foundation", "IUriRuntimeClass") {
        println!("{}.{}", t.namespace()?, t.name()?);

        for m in t.methods()? {
            println!("    {}", m.name()?);
        }
    }
    if let Some(t) = reader.find("Windows.Foundation", "IStringable") {
        println!("{}.{}", t.namespace()?, t.name()?);

        for m in t.methods()? {
            println!("    {}", m.name()?);
        }
    }

    for ns in reader.namespaces() {
        println!("namespace {}", ns.name());

        for t in ns.interfaces() {
            println!("\n    interface {}", t.name()?);
            for m in t.methods()? {
                println!("        {}", m.name()?);
            }
        }

        for t in ns.classes() {
            println!("    class {}", t.name()?);
        }
    }

    Ok(())
}

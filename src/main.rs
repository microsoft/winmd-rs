// #![allow(unused_variables)]
// #![allow(dead_code)]

mod codes;
mod database;
mod error;
mod flags;
mod reader;
mod tables;
use reader::*;

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}

fn run() -> std::io::Result<()> {
    let reader = Reader::from_os()?;

    // if let Some(t) = reader.find("Windows.Foundation", "AsyncStatus") {
    //     println!("{}.{}", t.namespace()?, t.name()?);

    //     for f in t.fields()? {
    //         println!("    {}", f.name()?);
    //     }
    // }

    for ns in reader.namespaces() {

        if ns.name() != "Windows.Foundation"
        {
            continue;
        }

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

        for t in ns.enums() {
            println!("    enum {}", t.name()?);
            for f in t.fields()? {
                println!("        {}", f.name()?);
            }
        }

        for t in ns.structs() {
            println!("    struct {}", t.name()?);
        }

        for t in ns.delegates() {
            println!("    delegate {}", t.name()?);
        }
    }

    Ok(())
}

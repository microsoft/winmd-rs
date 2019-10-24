// #![allow(unused_variables)]
// #![allow(dead_code)]

mod codes;
mod database;
mod error;
mod flags;
mod reader;
mod signatures;
mod tables;
use reader::*;

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}

fn run() -> std::io::Result<()> {
    let reader = Reader::from_os()?;

    if let Some(t) = reader.find("Windows.Foundation", "IGuidHelperStatics") {
        println!("{}.{}", t.namespace()?, t.name()?);

        for m in t.methods()? {
            if m.name()? == "Equals" {
                for p in m.params()? {
                    println!("            param {}", p.name()?);
                }
                m.signature()?;
            }
        }
    }

    // for ns in reader.namespaces() {
    //     if ns.name() != "Windows.Foundation" {
    //         continue;
    //     }

    //     println!("namespace {}", ns.name());

    //     for t in ns.interfaces() {
    //         println!("\n    interface {}", t.name()?);
    //         for m in t.methods()? {
    //             println!("        method {} - {}", m.name()?, m.flags()?.special());
    //             for p in m.params()? {
    //                 println!("            param {}", p.name()?);
    //             }
    //         }
    //     }

    //     for t in ns.classes() {
    //         println!("    class {}", t.name()?);
    //     }

    //     for t in ns.enums() {
    //         println!("    enum {}", t.name()?);
    //         for f in t.fields()? {
    //             for c in f.constants()? {
    //                 println!("        {} = {}", f.name()?, c.value()?);
    //             }
    //         }
    //     }

    //     for t in ns.structs() {
    //         println!("    struct {}", t.name()?);
    //         for f in t.fields()? {
    //             println!("        field {}", f.name()?);
    //         }
    //     }

    //     for t in ns.delegates() {
    //         println!("    delegate {}", t.name()?);
    //     }
    // }

    Ok(())
}

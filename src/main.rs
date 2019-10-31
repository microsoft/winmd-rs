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
use std::io::Result;
use std::io::*;
use tables::*;

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}

fn snake_case(preamble: &str, source: &str) -> String {
    let mut result = String::with_capacity(preamble.len() + source.len() + 2);
    result.push_str(preamble);
    let mut last = false;
    for c in source.chars() {
        if c.is_uppercase() {
            if last {
                result.push('_');
                last = false;
            }

            for lower in c.to_lowercase() {
                result.push(lower);
            }
        } else {
            result.push(c);
            last = true;
        }
    }
    result
}

// Add impl on MethoDef
fn rust_name(method: &MethodDef) -> Result<String> {
    let source = method.name()?;

    Ok(if method.flags()?.special() {
        if source.starts_with("get_") || source.starts_with("add_") {
            snake_case("", &source[4..])
        } else if source.starts_with("put_") {
            snake_case("set_", &source[4..])
        } else if source.starts_with("remove_") {
            snake_case("revoke_", &source[7..])
        } else {
            snake_case("", source)
        }
    } else {
        snake_case("", source)
    })
}

fn run() -> std::io::Result<()> {
    let reader = Reader::from_os()?;

    if let Some(t) = reader.find("Windows.Foundation", "IUriRuntimeClass") {
        println!("{}.{}", t.namespace()?, t.name()?);

        for m in t.methods()? {
            println!("\n{}", m.name()?);
            println!("  {}", rust_name(&m)?);
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
    //             let sig = m.signature()?;
    //             print!("        fn {}(", m.name()?);

    //             if let Some((last, rest)) = sig.params.split_last() {
    //                 for (param, signature) in rest {
    //                     print!("{}: {}, ", param.name()?, signature.type_sig);
    //                 }
    //                 let (param, signature) = last;
    //                 print!("{}: {}", param.name()?, signature.type_sig);
    //             }

    //             match sig.return_sig {
    //                 Some(value) => println!(") -> {};", value),
    //                 None => println!(");"),
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

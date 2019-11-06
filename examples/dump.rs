fn main() {
    if let Err(e) = run() {
        println!("{:?}", e);
    }
}

fn run() -> Result<(), winmd::Error> {
    let reader = winmd::Reader::from_os()?;

    for ns in reader.namespaces() {
        if ns.name() != "Windows.Foundation" {
            continue;
        }

        println!("namespace {}", ns.name());

        for t in ns.interfaces() {
            println!("\n    interface {}", t.name()?);
            for m in t.methods()? {
                let sig = m.signature()?;
                print!("        fn {}(", m.rust_name()?);

                if let Some((last, rest)) = sig.params().split_last() {
                    for (param, signature) in rest {
                        print!("{}: {}, ", param.name()?, signature.sig_type());
                    }
                    let (param, signature) = last;
                    print!("{}: {}", param.name()?, signature.sig_type());
                }

                match sig.return_type() {
                    Some(value) => println!(") -> {};", value),
                    None => println!(");"),
                }
            }
        }

        for t in ns.classes() {
            println!("    class {}", t.name()?);
        }

        for t in ns.enums() {
            println!("    enum {}", t.name()?);
            for f in t.fields()? {
                for c in f.constants()? {
                    println!("        {} = {}", f.name()?, c.value()?);
                }
            }
        }

        for t in ns.structs() {
            println!("    struct {}", t.name()?);
            for f in t.fields()? {
                println!("        field {}", f.name()?);
            }
        }

        for t in ns.delegates() {
            println!("    delegate {}", t.name()?);
        }
    }

    Ok(())
}

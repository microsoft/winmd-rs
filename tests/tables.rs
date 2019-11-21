#[test]
fn type_def() -> Result<(), winmd::Error> {
    let reader = winmd::Reader::from_os()?;
    let t: winmd::TypeDef = reader.find("Windows.Foundation", "IStringable").unwrap();

    let flags = t.flags()?;
    assert!(flags.windows_runtime());
    assert!(flags.interface());

    assert!(t.name()? == "IStringable");
    assert!(t.namespace()? == "Windows.Foundation");
    assert!(t.methods()?.count() == 1);

    for m in t.methods()? {
        assert!(m.name()? == "to_string");
        assert!(m.abi_name()? == "ToString");
        let sig = m.signature()?;
        assert!(sig.return_type().is_some());

        if let Some(return_type) = sig.return_type() {
            if let winmd::TypeSigType::ElementType(value) = return_type.sig_type() {
                assert!(*value == winmd::ElementType::String);
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }

    assert!(t.has_attribute("Windows.Foundation.Metadata", "GuidAttribute")?);

    let attribute = t.find_attribute("Windows.Foundation.Metadata", "GuidAttribute")?.unwrap();
    let args = attribute.arguments()?;
    let format = format!("{:X}-{:X}-{:X}-{:X}{:X}-{:X}{:X}{:X}{:X}{:X}{:X}", args[0].1, args[1].1, args[2].1, args[3].1, args[4].1, args[5].1, args[6].1, args[7].1, args[8].1, args[9].1, args[10].1,);
    assert!(format == "96369F54-8EB6-48F0-ABCE-C1B211E627C3");

    Ok(())
}

#[test]
fn type_ref() -> Result<(), winmd::Error> {
    let reader = winmd::Reader::from_os()?;
    let t = reader.find("Windows.Foundation", "AsyncStatus").unwrap();

    if let winmd::TypeDefOrRef::TypeRef(value) = t.extends()? {
        let t: winmd::TypeRef = value;
        assert!(t.name()? == "Enum");
        assert!(t.namespace()? == "System");
    } else {
        assert!(false);
    }
    Ok(())
}

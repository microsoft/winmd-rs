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
        assert!(m.name()? == "ToString");
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

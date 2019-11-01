#[test]
fn type_def() {
    let reader = winmd::Reader::from_os().unwrap();
    let t: winmd::TypeDef = reader.find("Windows.Foundation", "IStringable").unwrap();

    let flags = t.flags().unwrap();
    assert!(flags.windows_runtime());
    assert!(flags.interface());

    assert!(t.name().unwrap() == "IStringable");
    assert!(t.namespace().unwrap() == "Windows.Foundation");
    assert!(t.methods().unwrap().count() == 1);

    for m in t.methods().unwrap() {
        assert!(m.name().unwrap() == "ToString");
    }

    assert!(t.has_attribute("Windows.Foundation.Metadata", "GuidAttribute").unwrap());
}

#[test]
fn type_ref() {
    let reader = winmd::Reader::from_os().unwrap();
    let t = reader.find("Windows.Foundation", "AsyncStatus").unwrap();

    if let winmd::TypeDefOrRef::TypeRef(value) = t.extends().unwrap() {
        let t: winmd::TypeRef = value;
        assert!(t.name().unwrap() == "Enum");
        assert!(t.namespace().unwrap() == "System");
    } else {
        assert!(false);
    }
}

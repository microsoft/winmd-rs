#[test]
fn namespaces_and_iterators() {
    let reader = winmd::Reader::from_os().unwrap();

    assert!(reader.namespaces().find(|ns| ns.name() == "Nonexistent").is_none());

    let ns = reader.namespaces().find(|ns| ns.name() == "Windows.Foundation").unwrap();
    assert!(ns.name() == "Windows.Foundation");

    let t = ns.interfaces().find(|t| t.name().unwrap() == "IStringable").unwrap();
    assert!(t.name().unwrap() == "IStringable");

    let t = ns.classes().find(|t| t.name().unwrap() == "Uri").unwrap();
    assert!(t.name().unwrap() == "Uri");

    let t = ns.enums().find(|t| t.name().unwrap() == "AsyncStatus").unwrap();
    assert!(t.name().unwrap() == "AsyncStatus");

    let t = ns.structs().find(|t| t.name().unwrap() == "Point").unwrap();
    assert!(t.name().unwrap() == "Point");

    let t = ns.delegates().find(|t| t.name().unwrap() == "AsyncActionCompletedHandler").unwrap();
    assert!(t.name().unwrap() == "AsyncActionCompletedHandler");
}

#[test]
fn finding_types() {
    let reader = winmd::Reader::from_os().unwrap();

    assert!(reader.find("Windows.Foundation", "Nonexistent").is_none());
    assert!(reader.find("Nonexistent", "IStringable").is_none());

    let t = reader.find("Windows.Foundation", "IStringable").unwrap();
    assert!(t.name().unwrap() == "IStringable");
}

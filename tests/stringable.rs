#[test]
fn stringable() {
    let path = std::path::PathBuf::from("winmds\\Windows.Foundation.FoundationContract.winmd");
    let reader = &winmd::TypeReader::from_iter(vec![path]);

    let def = reader.resolve_type_def(("Windows.Foundation", "IStringable"));
    assert!(def.name(reader) == ("Windows.Foundation", "IStringable"));

    let methods : Vec<winmd::MethodDef> = def.methods(reader).collect();
    assert!(methods.len() == 1);
    assert!(methods[0].name(reader) == "ToString");
}

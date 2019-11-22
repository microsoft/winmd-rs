fn test_reader() -> Result<winmd::Reader, winmd::Error> {
    let mut path = std::path::PathBuf::new();
    path.push(std::env::var("windir").expect("'windir' environment variable not found"));
    path.push(SYSTEM32);
    path.push("winmetadata");

    let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(path)?.filter_map(|value| value.ok().map(|value| value.path())).collect();
    files.push(std::path::PathBuf::from("tests/metadata.winmd"));
    winmd::Reader::from_files(&files)
}

#[cfg(target_pointer_width = "64")]
const SYSTEM32: &str = "System32";

#[cfg(target_pointer_width = "32")]
const SYSTEM32: &str = "SysNative";

#[test]
fn attributes() -> Result<(), winmd::Error> {
    let reader = test_reader()?;

    let t: winmd::TypeDef = reader.find("Test.ITypeAttribute").unwrap();

    let attribute = t.find_attribute("Test.TypeAttribute")?;
    let args = attribute.arguments()?;

    println!("{}", args.len());

    assert!(args.len() == 4);

    assert!(args[0].0 == "B");
    assert!(args[0].1 == winmd::ArgumentSig::Bool(true));

    assert!(args[1].0 == "I32");
    assert!(args[1].1 == winmd::ArgumentSig::I32(123));

    assert!(args[2].0 == "S");
    assert!(args[2].1 == winmd::ArgumentSig::String("Test"));

    assert!(args[3].0 == "T");
    assert!(args[3].1 == winmd::ArgumentSig::Type(reader.find("Test.TypeStruct")?));

    Ok(())
}

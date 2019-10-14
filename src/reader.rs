use crate::database::*;
use crate::tables::*;
use std::io::Result;

#[derive(Default)]
pub struct Namespace {
    types: std::collections::BTreeMap<String, (u32, u32)>,
    interfaces: std::vec::Vec<(u32, u32)>,
    classes: std::vec::Vec<(u32, u32)>,
    enums: std::vec::Vec<(u32, u32)>,
    structs: std::vec::Vec<(u32, u32)>,
    delegates: std::vec::Vec<(u32, u32)>,
    attributes: std::vec::Vec<(u32, u32)>,
    contracts: std::vec::Vec<(u32, u32)>,
}

pub struct Reader {
    databases: std::vec::Vec<Database>,
    namespaces: std::collections::BTreeMap<String, Namespace>,
}

impl<'a> Reader {
    // TODO: Can't this be an iterator to avoid creating the collection in from_dir()?
    pub fn from_files<P: AsRef<std::path::Path>>(filenames: &[P]) -> Result<Self> {
        let mut databases = std::vec::Vec::new();
        databases.reserve(filenames.len());

        let mut namespaces = std::collections::BTreeMap::new();

        for filename in filenames {
            let db = Database::new(filename)?;

            for t in db.type_def() {
                if !t.flags()?.windows_runtime() {
                    continue;
                }

                let namespace = namespaces.entry(t.namespace()?.to_string()).or_insert_with(|| Namespace { ..Default::default() });
                namespace.types.entry(t.name()?.to_string()).or_insert((databases.len() as u32, t.index()));
            }

            databases.push(db);

            for (_, namespace) in &mut namespaces {
                for (_, index) in &namespace.types {
                    let t = TypeDef::new(&databases[index.0 as usize], index.1);

                    if t.flags()?.interface() {
                        namespace.interfaces.push(*index);
                        continue;
                    }
                    match t.extends()?.name()? {
                        "Enum" => namespace.enums.push(*index),
                        "MulticastDelegate" => namespace.delegates.push(*index),
                        "Attribute" => {}
                        "ValueType" => {
                            if !t.has_attribute("Windows.Foundation.Metadata", "ApiContractAttribute")? {
                                namespace.structs.push(*index);
                            }
                        }
                        _ => namespace.classes.push(*index),
                    }
                }
            }
        }

        Ok(Self { databases, namespaces })
    }
    pub fn from_dir<P: AsRef<std::path::Path>>(directory: P) -> Result<Self> {
        let files: Vec<std::path::PathBuf> = std::fs::read_dir(directory)?
            .filter_map(|value| match value {
                Ok(value) => Some(value.path()),
                Err(_) => None,
            })
            .collect();
        Self::from_files(&files)
    }
    pub fn from_local() -> Result<Self> {
        let mut path = std::path::PathBuf::new();
        path.push(match std::env::var("windir") {
            Ok(value) => value,
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "WINDIR environment variable not found")),
        });
        path.push(system32());
        path.push("winmetadata");
        Self::from_dir(path)
    }
    pub fn namespaces(&self) -> std::vec::Vec<String> {
       self.namespaces.keys().cloned().collect()
    }



    // TODO: from_sdk
    // namespaces (iterator)
    // types (namespace)
    pub fn find(&self, namespace: &str, name: &str) -> Option<TypeDef> {
        match self.namespaces.get(namespace) {
            Some(namespace) => match namespace.types.get(name) {
                Some(index) => Some(TypeDef::new(&self.databases[index.0 as usize], index.1)),
                None => None,
            },
            None => None,
        }
    }
}

#[cfg(target_pointer_width = "64")]
fn system32() -> &'static str {
    "System32"
}
#[cfg(target_pointer_width = "32")]
fn system32() -> &'static str {
    "SysNative"
}

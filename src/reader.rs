use crate::database::*;
use crate::tables::*;
use std::io::Result;

pub struct NamespaceIterator<'a> {
    iter: std::collections::btree_map::Keys<'a, String, NamespaceData>,
}
impl<'a> Iterator for NamespaceIterator<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some(value) => Some(value),
        }
    }
}

pub struct TypeIterator<'a> {
    reader: &'a Reader,
    iter: std::slice::Iter<'a, (u32, u32)>,
}
impl<'a> Iterator for TypeIterator<'a> {
    type Item = TypeDef2<'a>;
    fn next(&mut self) -> Option<TypeDef2<'a>> {
        match self.iter.next() {
            None => None,
            Some(&(db, index)) => Some(TypeDef2::new(&self.reader.databases[db as usize], index)),
        }
    }
}

#[derive(Default)]
struct NamespaceData {
    index: std::collections::BTreeMap<String, (u32, u32)>,
    interfaces: std::vec::Vec<(u32, u32)>,
    classes: std::vec::Vec<(u32, u32)>,
    enums: std::vec::Vec<(u32, u32)>,
    structs: std::vec::Vec<(u32, u32)>,
    delegates: std::vec::Vec<(u32, u32)>,
}

pub struct Namespace<'a> {
    // add name
    reader: &'a Reader,
    types: &'a NamespaceData,
}
impl<'a> Namespace<'a> {
    pub fn interfaces(&self) -> TypeIterator {
        TypeIterator { reader: self.reader, iter: self.types.interfaces.iter() }
    }
    pub fn classes(&self) -> TypeIterator {
        TypeIterator { reader: self.reader, iter: self.types.classes.iter() }
    }
    pub fn enums(&self) -> TypeIterator {
        TypeIterator { reader: self.reader, iter: self.types.enums.iter() }
    }
    pub fn structs(&self) -> TypeIterator {
        TypeIterator { reader: self.reader, iter: self.types.structs.iter() }
    }
    pub fn delegates(&self) -> TypeIterator {
        TypeIterator { reader: self.reader, iter: self.types.delegates.iter() }
    }
}

pub struct Reader {
    databases: std::vec::Vec<Database>,
    namespaces: std::collections::BTreeMap<String, NamespaceData>,
}
impl<'a> Reader {
    // TODO: Can't this be an iterator to avoid creating the collection in from_dir()?
    pub fn from_files<P: AsRef<std::path::Path>>(filenames: &[P]) -> Result<Self> {
        let mut databases = std::vec::Vec::new();
        databases.reserve(filenames.len());
        let mut namespaces = std::collections::BTreeMap::new();
        for filename in filenames {
            let db = Database::new(filename)?;
            for t in db.type_def2() {
                if !t.flags()?.windows_runtime() {
                    continue;
                }
                let types = namespaces.entry(t.namespace()?.to_string()).or_insert_with(|| NamespaceData { ..Default::default() });
                types.index.entry(t.name()?.to_string()).or_insert((databases.len() as u32, t.index()));
            }
            databases.push(db);
        }
        for (_, types) in &mut namespaces {
            for (_, index) in &types.index {
                let t = TypeDef2::new(&databases[index.0 as usize], index.1);
                if t.flags()?.interface() {
                    types.interfaces.push(*index);
                    continue;
                }
                match t.extends()?.name()? {
                    "Enum" => types.enums.push(*index),
                    "MulticastDelegate" => types.delegates.push(*index),
                    "Attribute" => {}
                    "ValueType" => {
                        if !t.has_attribute("Windows.Foundation.Metadata", "ApiContractAttribute")? {
                            types.structs.push(*index);
                        }
                    }
                    _ => types.classes.push(*index),
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
    pub fn from_os() -> Result<Self> {
        let mut path = std::path::PathBuf::new();
        path.push(match std::env::var("windir") {
            Ok(value) => value,
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "WINDIR environment variable not found")),
        });
        path.push(system32());
        path.push("winmetadata");
        Self::from_dir(path)
    }
    pub fn namespaces(&self) -> NamespaceIterator {
        NamespaceIterator { iter: self.namespaces.keys() }
    }
    pub fn types(&self, namespace: &str) -> Option<Namespace> {
        match self.namespaces.get(namespace) {
            None => None,
            Some(value) => Some(Namespace { reader: self, types: value }),
        }
    }
    // TODO: from_sdk
    pub fn find(&self, namespace: &str, name: &str) -> Option<TypeDef2> {
        match self.namespaces.get(namespace) {
            Some(types) => match types.index.get(name) {
                Some(&(db, index)) => Some(TypeDef2::new(&self.databases[db as usize], index)),
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

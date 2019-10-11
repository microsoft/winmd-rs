use crate::database::*;
use crate::tables::*;
use std::io::Result;

#[derive(Default)]
struct Members {
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
    namespaces: std::collections::BTreeMap<String, Members>,
}

impl<'a> Reader {
    pub fn new<P: AsRef<std::path::Path>>(filenames: &[P]) -> Result<Reader> {
        let mut databases = std::vec::Vec::new();
        databases.reserve(filenames.len());

        let mut namespaces = std::collections::BTreeMap::new();

        for filename in filenames {
            let db = Database::new(filename)?;

            for t in db.type_def() {
                if !t.flags()?.windows_runtime() {
                    continue;
                }

                let members = namespaces.entry(t.namespace()?.to_string()).or_insert_with(|| Members { ..Default::default() });
                members.types.entry(t.name()?.to_string()).or_insert((databases.len() as u32, t.index()));
            }

            for (_, members) in &mut namespaces {
                for (_, index) in &members.types {
                    let t = TypeDef::new(&db, index.1);

                    if t.flags()?.interface() {
                        members.interfaces.push(*index);
                        continue;
                    }
                    match t.extends()?.name()? {
                        "Enum" => members.enums.push(*index),
                        "MulticastDelegate" => members.delegates.push(*index),
                        "Attribute" => {}
                        "ValueType" => {
                            if !t.has_attribute("Windows.Foundation.Metadata", "ApiContractAttribute")? {
                                members.structs.push(*index);
                            }
                        }
                        _ => members.classes.push(*index),
                    }
                }
            }

            databases.push(db);
        }

        Ok(Reader { databases, namespaces })
    }
    pub fn find(&self, namespace: &str, name: &str) -> Option<TypeDef>
    {
        match self.namespaces.get(namespace)
        {
            Some(members) => match members.types.get(name)
            {
                Some(index) => Some(TypeDef::new(&self.databases[index.0 as usize], index.1)),
                None => None,
            }
            None => None,
        }
    }
}

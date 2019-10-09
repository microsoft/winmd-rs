use crate::database::*;
use std::io::Result;

#[derive(Default)]
struct Members
{
    types: std::collections::BTreeMap<String, u32>,
    interfaces: std::vec::Vec<u32>,
    classes: std::vec::Vec<u32>,
    enums: std::vec::Vec<u32>,
    structs: std::vec::Vec<u32>,
    delegates: std::vec::Vec<u32>,
    attributes: std::vec::Vec<u32>,
    contracts: std::vec::Vec<u32>,
}

pub struct Cache{
    databases: std::vec::Vec<Database>,
    namespaces: std::collections::BTreeMap<String, Members>,
}

impl Cache
{
    pub fn new<P: AsRef<std::path::Path>>(filenames: &[P]) -> Result<Cache>{
        
        let mut databases = std::vec::Vec::new();
        databases.reserve(filenames.len());

        let mut namespaces = std::collections::BTreeMap::new();

        for filename in filenames{
            let db = Database::new(filename)?;

            for t in db.type_def()
            {
                if !t.flags()?.windows_runtime()
                {
                    continue;
                }

                let members = namespaces.entry(t.namespace()?.to_string()).or_insert_with(||Members{..Default::default()});
                members.types.entry(t.name()?.to_string()).or_insert(t.first);
            }

            for (name, members) in namespaces{
                for (_, index) in members.types
                {

                }
                // if t.flags()?.interface() {
                //     members.interfaces.push(t.first);
                //     continue;
                // }
                // match t.extends()?.name()? {
                //     "Enum" => members.enums.push(t.first),
                //     "MulticastDelegate" => members.delegates.push(t.first),
                //     "Attribute" => {},
                //     "ValueType" =>{
                //         if !t.has_attribute("Windows.Foundation.Metadata", "ApiContractAttribute")?
                //         {
                //             members.structs.push(t.first);
                //         }
                //     },
                //     _ => members.classes.push(t.first),
                // }
            }

            databases.push(db);
        }

        Ok(Cache{databases, namespaces})
    }
}

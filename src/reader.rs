use crate::error::*;
use crate::file::*;
use crate::helpers::*;
use crate::tables::*;

pub struct Reader {
    files: Vec<File>,
    namespaces: std::collections::BTreeMap<String, NamespaceData>,
}

pub struct NamespaceIterator<'a> {
    reader: &'a Reader,
    iter: std::collections::btree_map::Iter<'a, String, NamespaceData>,
}

pub struct TypeIterator<'a> {
    reader: &'a Reader,
    iter: std::slice::Iter<'a, (u32, u32)>,
}

pub struct Namespace<'a> {
    reader: &'a Reader,
    name: &'a str,
    types: &'a NamespaceData,
}

#[derive(Default)]
struct NamespaceData {
    index: std::collections::BTreeMap<String, (u32, u32)>,
    interfaces: Vec<(u32, u32)>,
    classes: Vec<(u32, u32)>,
    enums: Vec<(u32, u32)>,
    structs: Vec<(u32, u32)>,
    delegates: Vec<(u32, u32)>,
}

impl<'a> Iterator for NamespaceIterator<'a> {
    type Item = Namespace<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let (key, value) = self.iter.next()?;
        Some(Namespace { reader: self.reader, name: key, types: value })
    }
}

impl<'a> Iterator for TypeIterator<'a> {
    type Item = TypeDef<'a>;
    fn next(&mut self) -> Option<TypeDef<'a>> {
        let &(file, index) = self.iter.next()?;
        Some(TypeDef::new(&self.reader.files[file as usize].type_def(self.reader), index))
    }
}

impl<'a> Namespace<'a> {
    pub fn name(&self) -> &str {
        self.name
    }
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

impl<'a> Reader {
    pub fn from_files<P: AsRef<std::path::Path>>(filenames: &[P]) -> Result<Self, Error> {
        let mut files = Vec::with_capacity(filenames.len());
        let mut namespaces = std::collections::BTreeMap::<String, NamespaceData>::new();
        let reader = Reader { files: Vec::new(), namespaces: std::collections::BTreeMap::new() };

        for filename in filenames {
            let file = File::new(filename)?;
            for t in file.type_def(&reader).iter::<TypeDef>() {
                if t.flags()?.windows_runtime() {
                    let types = namespaces.entry(t.namespace()?.to_string()).or_insert_with(|| Default::default());
                    types.index.entry(t.name()?.to_string()).or_insert((files.len() as u32, t.row.index));
                }
            }
            files.push(file);
        }

        for (_, types) in &mut namespaces {
            for (_, index) in &types.index {
                let t = TypeDef::new(&files[index.0 as usize].type_def(&reader), index.1);
                if t.flags()?.interface() {
                    types.interfaces.push(*index);
                } else {
                    match t.extends()?.name()? {
                        "Enum" => types.enums.push(*index),
                        "MulticastDelegate" => types.delegates.push(*index),
                        "Attribute" => {}
                        "ValueType" => {
                            if !t.has_attribute("Windows.Foundation.Metadata.ApiContractAttribute")? {
                                types.structs.push(*index);
                            }
                        }
                        _ => types.classes.push(*index),
                    }
                }
            }
        }

        Ok(Reader { files, namespaces })
    }

    pub fn from_dir<P: AsRef<std::path::Path>>(directory: P) -> Result<Self, Error> {
        let files: Vec<std::path::PathBuf> = std::fs::read_dir(directory)?.filter_map(|value| value.ok().map(|value| value.path())).collect();
        Self::from_files(&files)
    }

    pub fn from_os() -> Result<Self, Error> {
        let mut path = std::path::PathBuf::new();
        path.push(std::env::var("windir").expect("'windir' environment variable not found"));
        path.push(SYSTEM32);
        path.push("winmetadata");
        Self::from_dir(path)
    }

    pub fn namespaces(&self) -> NamespaceIterator {
        NamespaceIterator { reader: self, iter: self.namespaces.iter() }
    }

    pub fn find(&self, full_name: &str) -> ParseResult<TypeDef> {
        let (namespace, name) = split_type_name(full_name)?;
        let types = self.namespaces.get(namespace).ok_or_else(|| ParseError::MissingType(full_name.to_string()))?;
        let &(file, index) = types.index.get(name).ok_or_else(|| ParseError::MissingType(full_name.to_string()))?;
        Ok(TypeDef::new(&self.files[file as usize].type_def(self), index))
    }
}

#[cfg(target_pointer_width = "64")]
const SYSTEM32: &str = "System32";

#[cfg(target_pointer_width = "32")]
const SYSTEM32: &str = "SysNative";

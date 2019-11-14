// TODO: Many tables still need columns defined and tested.
#![allow(dead_code)]
#![allow(exceeding_bitshifts)]

use crate::error::*;
use std::marker::PhantomData;

macro_rules! table_fn {
    ($name:ident) => {
        pub fn $name(&self) -> Table {
            self.$name.table(self)
        }
    };
}

pub struct RowIterator<'a, T: Row<'a>> {
    table: Table<'a>,
    first: u32,
    last: u32,
    phantom: PhantomData<T>,
}

impl<'a, T: Row<'a>> RowIterator<'a, T> {
    pub fn new(table: &Table<'a>, first: u32, last: u32) -> RowIterator<'a, T> {
        RowIterator { table: *table, first: first, last, phantom: PhantomData }
    }
}

impl<'a, T: Row<'a>> Iterator for RowIterator<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.first >= self.last {
            return None;
        }
        self.first += 1;
        Some(T::new(&self.table, self.first - 1))
    }
}

pub trait Row<'a> {
    fn new(table: &Table<'a>, index: u32) -> Self;
    fn from_rows(table: &Table<'a>, range: (u32, u32)) -> RowIterator<'a, Self>
    where
        Self: Sized,
    {
        RowIterator::new(table, range.0, range.1)
    }
}

#[derive(Copy, Clone)]
pub(crate) struct RowData<'a> {
    pub(crate) table: Table<'a>,
    pub(crate) index: u32,
}

impl<'a> RowData<'a> {
    pub fn str(&self, column: u32) -> ParseResult<&str> {
        self.table.str(self.index, column)
    }

    pub fn blob(&self, column: u32) -> ParseResult<&[u8]> {
        self.table.blob(self.index, column)
    }

    pub fn blob_as<T>(&self, column: u32) -> ParseResult<&T> {
        self.blob(column)?.view_as::<T>(0)
    }

    pub fn u32(&self, column: u32) -> ParseResult<u32> {
        self.table.u32(self.index, column)
    }

    pub fn list<T: Row<'a>>(&self, column: u32, table: &Table<'a>) -> ParseResult<RowIterator<'a, T>> {
        let first = self.u32(column)? - 1;
        let last = if self.index + 1 < self.table.len() { self.table.u32(self.index + 1, column)? - 1 } else { table.len() };
        Ok(RowIterator::new(table, first, last))
    }
}

#[derive(Copy, Clone)]
pub struct Table<'a> {
    pub(crate) db: &'a Database,
    data: &'a TableData,
}

impl<'a> Table<'a> {
    // TODO: make iter/rows/row work like slice.get
    pub fn iter<T: Row<'a>>(&self) -> RowIterator<'a, T> {
        T::from_rows(self, (0, self.data.row_count))
    }

    pub fn rows<T: Row<'a>>(&self, first: u32, last: u32) -> RowIterator<'a, T> {
        T::from_rows(self, (first, last))
    }

    pub fn row<T: Row<'a>>(&self, index: u32) -> T {
        T::new(self, index)
    }

    pub fn len(&self) -> u32 {
        self.data.row_count
    }

    pub fn str(&self, row: u32, column: u32) -> ParseResult<&str> {
        let offset = (self.db.strings + self.u32(row, column)?) as usize;
        let last = self.db.bytes[offset..].iter().position(|c| *c == b'\0').ok_or_else(unexpected_eof)?;
        std::str::from_utf8(&self.db.bytes[offset..offset + last]).map_err(|_| ParseError::InvalidData("Bytes not valid UTF-8"))
    }

    pub fn blob(&self, row: u32, column: u32) -> ParseResult<&[u8]> {
        let offset = (self.db.blobs + self.u32(row, column)?) as usize;
        let initial_byte = self.db.bytes[offset];
        let (mut blob_size, blob_size_bytes) = match initial_byte >> 5 {
            0..=3 => (initial_byte & 0x7f, 1),
            4..=5 => (initial_byte & 0x3f, 2),
            6 => (initial_byte & 0x1f, 4),
            _ => return Err(ParseError::InvalidData("Invalid blob encoding")),
        };
        for byte in &self.db.bytes[offset + 1..offset + blob_size_bytes] {
            blob_size = (blob_size << 8) + byte;
        }
        Ok(&self.db.bytes[offset + blob_size_bytes..offset + blob_size_bytes + blob_size as usize])
    }

    pub fn u32(&self, row: u32, column: u32) -> ParseResult<u32> {
        let offset = self.data.data + row * self.data.row_size + self.data.columns[column as usize].0;
        match self.data.columns[column as usize].1 {
            1 => Ok(*(self.db.bytes.view_as::<u8>(offset)?) as u32),
            2 => Ok(*(self.db.bytes.view_as::<u16>(offset)?) as u32),
            4 => Ok(*(self.db.bytes.view_as::<u32>(offset)?) as u32),
            _ => Ok(*(self.db.bytes.view_as::<u64>(offset)?) as u32),
        }
    }

    fn lower_bound_of(&self, mut first: u32, last: u32, column: u32, value: u32) -> ParseResult<u32> {
        let mut count = last - first;
        while count > 0 {
            let count2 = count / 2;
            let middle = first + count2;
            if self.u32(middle, column)? < value {
                first = middle + 1;
                count -= count2 + 1;
            } else {
                count = count2;
            }
        }
        Ok(first)
    }

    pub fn upper_bound<T: Row<'a>>(&self, column: u32, value: u32) -> ParseResult<T> {
        let index = self.upper_bound_of(0, self.data.row_count, column, value)?;
        if index == self.data.row_count {
            return Err(ParseError::InvalidData("Invalid row index"));
        }
        Ok(T::new(self, index))
    }

    fn upper_bound_of(&self, mut first: u32, last: u32, column: u32, value: u32) -> ParseResult<u32> {
        let mut count = last - first;
        while count > 0 {
            let count2 = count / 2;
            let middle = first + count2;
            if value < self.u32(middle, column)? {
                count = count2
            } else {
                first = middle + 1;
                count -= count2 + 1;
            }
        }
        Ok(first)
    }

    pub fn equal_range<T: Row<'a>>(&self, column: u32, value: u32) -> ParseResult<RowIterator<'a, T>> {
        Ok(T::from_rows(self, self.equal_range_of(0, self.data.row_count, column, value)?))
    }

    fn equal_range_of(&self, mut first: u32, mut last: u32, column: u32, value: u32) -> ParseResult<(u32, u32)> {
        let mut count = last - first;
        loop {
            if count <= 0 {
                last = first;
                break;
            }
            let count2 = count / 2;
            let middle = first + count2;
            let middle_value = self.u32(middle, column)?;
            if middle_value < value {
                first = middle + 1;
                count -= count2 + 1;
            } else if value < middle_value {
                count = count2;
            } else {
                let first2 = self.lower_bound_of(first, middle, column, value)?;
                first += count;
                last = self.upper_bound_of(middle + 1, first, column, value)?;
                first = first2;
                break;
            }
        }
        Ok((first, last))
    }
}

#[derive(Default)]
pub struct TableData {
    data: u32,
    row_count: u32,
    row_size: u32,
    columns: [(u32, u32); 6],
}

impl TableData {
    pub fn table<'a>(&'a self, db: &'a Database) -> Table<'a> {
        Table { db, data: self }
    }

    fn index_size(&self) -> u32 {
        if self.row_count < (1 << 16) {
            2
        } else {
            4
        }
    }

    fn set_columns(&mut self, a: u32, b: u32, c: u32, d: u32, e: u32, f: u32) {
        self.row_size = (a + b + c + d + e + f).into();
        self.columns[0] = (0, a);
        if b != 0 {
            self.columns[1] = ((a), b);
        }
        if c != 0 {
            self.columns[2] = ((a + b), c);
        }
        if d != 0 {
            self.columns[3] = ((a + b + c), d);
        }
        if e != 0 {
            self.columns[4] = ((a + b + c + d), e);
        }
        if f != 0 {
            self.columns[5] = ((a + b + c + d + e), f);
        }
    }

    fn set_data(&mut self, data: &mut u32) {
        if self.row_count != 0 {
            let next = *data + self.row_count * self.row_size;
            self.data = *data;
            *data = next;
        }
    }
}

#[derive(Default)]
pub struct Database {
    bytes: std::vec::Vec<u8>,
    strings: u32,
    blobs: u32,
    guids: u32,

    pub assembly: TableData,
    pub assembly_os: TableData,
    pub assembly_processor: TableData,
    pub assembly_ref: TableData,
    pub assembly_ref_os: TableData,
    pub assembly_ref_processor: TableData,
    pub class_layout: TableData,
    pub constant: TableData,
    pub custom_attribute: TableData,
    pub decl_security: TableData,
    pub event: TableData,
    pub event_map: TableData,
    pub exported_type: TableData,
    pub field: TableData,
    pub field_layout: TableData,
    pub field_marshal: TableData,
    pub field_rva: TableData,
    pub file: TableData,
    pub generic_param: TableData,
    pub generic_param_constraint: TableData,
    pub impl_map: TableData,
    pub interface_impl: TableData,
    pub manifest_resource: TableData,
    pub member_ref: TableData,
    pub method_def: TableData,
    pub method_impl: TableData,
    pub method_semantics: TableData,
    pub method_spec: TableData,
    pub module: TableData,
    pub module_ref: TableData,
    pub nested_class: TableData,
    pub param: TableData,
    pub permission: TableData,
    pub property: TableData,
    pub property_map: TableData,
    pub standalone_sig: TableData,
    pub type_def: TableData,
    pub type_ref: TableData,
    pub type_spec: TableData,
}

impl Database {
    pub fn new<P: AsRef<std::path::Path>>(filename: P) -> ParseResult<Self> {
        let mut db = Self { bytes: std::fs::read(filename)?, ..Default::default() };
        let dos = db.bytes.view_as::<ImageDosHeader>(0)?;
        if dos.signature != IMAGE_DOS_SIGNATURE {
            return Err(ParseError::InvalidData("Invalid DOS signature"));
        }
        let pe = db.bytes.view_as::<ImageNtHeader>(dos.lfanew as u32)?;
        let (com_virtual_address, sections) = match pe.optional_header.magic {
            MAGIC_PE32 => (pe.optional_header.data_directory[IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR as usize].virtual_address, db.bytes.view_as_slice_of::<ImageSectionHeader>(dos.lfanew as u32 + sizeof::<ImageNtHeader>(), pe.file_header.number_of_sections as u32)?),
            MAGIC_PE32PLUS => (db.bytes.view_as::<ImageNtHeaderPlus>(dos.lfanew as u32)?.optional_header.data_directory[IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR as usize].virtual_address, db.bytes.view_as_slice_of::<ImageSectionHeader>(dos.lfanew as u32 + sizeof::<ImageNtHeaderPlus>(), pe.file_header.number_of_sections as u32)?),
            _ => return Err(ParseError::InvalidData("Invalid optional header magic value")),
        };
        let cli = db.bytes.view_as::<ImageCorHeader>(offset_from_rva(section_from_rva(sections, com_virtual_address)?, com_virtual_address))?;
        if cli.cb != sizeof::<ImageCorHeader>() {
            return Err(ParseError::InvalidData("Invalid CLI header"));
        }
        let cli_offset = offset_from_rva(section_from_rva(sections, cli.meta_data.virtual_address)?, cli.meta_data.virtual_address);
        if *db.bytes.view_as::<u32>(cli_offset)? != STORAGE_MAGIC_SIG {
            return Err(ParseError::InvalidData("CLI metadata magic signature not found"));
        }
        let version_length = *db.bytes.view_as::<u32>(cli_offset + 12)?;
        let mut view = cli_offset + version_length + 20;
        let mut tables_data: (u32, u32) = (0, 0);
        for _ in 0..*db.bytes.view_as::<u16>(cli_offset + version_length + 18)? {
            let stream_offset = *db.bytes.view_as::<u32>(view)?;
            let stream_size = *db.bytes.view_as::<u32>(view + 4)?;
            let stream_name = db.bytes.view_as_str(view + 8)?;
            match stream_name {
                b"#Strings" => db.strings = cli_offset + stream_offset,
                b"#Blob" => db.blobs = cli_offset + stream_offset,
                b"#GUID" => db.guids = cli_offset + stream_offset,
                b"#~" => tables_data = (cli_offset + stream_offset, stream_size),
                b"#US" => {}
                _ => return Err(ParseError::InvalidData("Unknown metadata stream")),
            }
            let mut padding = 4 - stream_name.len() % 4;
            if padding == 0 {
                padding = 4;
            }
            view = view + (8 + stream_name.len() + padding) as u32;
        }
        let heap_sizes = *db.bytes.view_as::<u8>(tables_data.0 + 6)?;
        let string_index_size = if (heap_sizes & 1) == 1 { 4 } else { 2 };
        let guid_index_size = if (heap_sizes >> 1 & 1) == 1 { 4 } else { 2 };
        let blob_index_size = if (heap_sizes >> 2 & 1) == 1 { 4 } else { 2 };
        let valid_bits = *db.bytes.view_as::<u64>(tables_data.0 + 8)?;
        view = tables_data.0 + 24;
        for i in 0..64 {
            if (valid_bits >> i & 1) == 0 {
                continue;
            }
            let row_count = *db.bytes.view_as::<u32>(view)?;
            view = view + 4;
            match i {
                0x00 => db.module.row_count = row_count,
                0x01 => db.type_ref.row_count = row_count,
                0x02 => db.type_def.row_count = row_count,
                0x04 => db.field.row_count = row_count,
                0x06 => db.method_def.row_count = row_count,
                0x08 => db.param.row_count = row_count,
                0x09 => db.interface_impl.row_count = row_count,
                0x0a => db.member_ref.row_count = row_count,
                0x0b => db.constant.row_count = row_count,
                0x0c => db.custom_attribute.row_count = row_count,
                0x0d => db.field_marshal.row_count = row_count,
                0x0e => db.decl_security.row_count = row_count,
                0x0f => db.class_layout.row_count = row_count,
                0x10 => db.field_layout.row_count = row_count,
                0x11 => db.standalone_sig.row_count = row_count,
                0x12 => db.event_map.row_count = row_count,
                0x14 => db.event.row_count = row_count,
                0x15 => db.property_map.row_count = row_count,
                0x17 => db.property.row_count = row_count,
                0x18 => db.method_semantics.row_count = row_count,
                0x19 => db.method_impl.row_count = row_count,
                0x1a => db.module_ref.row_count = row_count,
                0x1b => db.type_spec.row_count = row_count,
                0x1c => db.impl_map.row_count = row_count,
                0x1d => db.field_rva.row_count = row_count,
                0x20 => db.assembly.row_count = row_count,
                0x21 => db.assembly_processor.row_count = row_count,
                0x22 => db.assembly_os.row_count = row_count,
                0x23 => db.assembly_ref.row_count = row_count,
                0x24 => db.assembly_ref_processor.row_count = row_count,
                0x25 => db.assembly_ref_os.row_count = row_count,
                0x26 => db.file.row_count = row_count,
                0x27 => db.exported_type.row_count = row_count,
                0x28 => db.manifest_resource.row_count = row_count,
                0x29 => db.nested_class.row_count = row_count,
                0x2a => db.generic_param.row_count = row_count,
                0x2b => db.method_spec.row_count = row_count,
                0x2c => db.generic_param_constraint.row_count = row_count,
                _ => return Err(ParseError::InvalidData("Unknown metadata table")),
            };
        }
        let empty_table = TableData::default();
        let has_constant = composite_index_size(&[&db.field, &db.param, &db.property]);
        let type_def_or_ref = composite_index_size(&[&db.type_def, &db.type_ref, &db.type_spec]);
        let has_custom_attribute = composite_index_size(&[&db.method_def, &db.field, &db.type_ref, &db.type_def, &db.param, &db.interface_impl, &db.member_ref, &db.module, &db.property, &db.event, &db.standalone_sig, &db.module_ref, &db.type_spec, &db.assembly, &db.assembly_ref, &db.file, &db.exported_type, &db.manifest_resource, &db.generic_param, &db.generic_param_constraint, &db.method_spec]);
        let has_field_marshal = composite_index_size(&[&db.field, &db.param]);
        let has_decl_security = composite_index_size(&[&db.type_def, &db.method_def, &db.assembly]);
        let member_ref_parent = composite_index_size(&[&db.type_def, &db.type_ref, &db.module_ref, &db.method_def, &db.type_spec]);
        let has_semantics = composite_index_size(&[&db.event, &db.property]);
        let method_def_or_ref = composite_index_size(&[&db.method_def, &db.member_ref]);
        let member_forwarded = composite_index_size(&[&db.field, &db.method_def]);
        let implementation = composite_index_size(&[&db.file, &db.assembly_ref, &db.exported_type]);
        let custom_attribute_type = composite_index_size(&[&db.method_def, &db.member_ref, &empty_table, &empty_table, &empty_table]);
        let resolution_scope = composite_index_size(&[&db.module, &db.module_ref, &db.assembly_ref, &db.type_ref]);
        let type_or_method_def = composite_index_size(&[&db.type_def, &db.method_def]);

        db.assembly.set_columns(4, 8, 4, blob_index_size, string_index_size, string_index_size);
        db.assembly_os.set_columns(4, 4, 4, 0, 0, 0);
        db.assembly_processor.set_columns(4, 0, 0, 0, 0, 0);
        db.assembly_ref.set_columns(8, 4, blob_index_size, string_index_size, string_index_size, blob_index_size);
        db.assembly_ref_os.set_columns(4, 4, 4, db.assembly_ref.index_size(), 0, 0);
        db.assembly_ref_processor.set_columns(4, db.assembly_ref.index_size(), 0, 0, 0, 0);
        db.class_layout.set_columns(2, 4, db.type_def.index_size(), 0, 0, 0);
        db.constant.set_columns(2, has_constant, blob_index_size, 0, 0, 0);
        db.custom_attribute.set_columns(has_custom_attribute, custom_attribute_type, blob_index_size, 0, 0, 0);
        db.decl_security.set_columns(2, has_decl_security, blob_index_size, 0, 0, 0);
        db.event_map.set_columns(db.type_def.index_size(), db.event.index_size(), 0, 0, 0, 0);
        db.event.set_columns(2, string_index_size, type_def_or_ref, 0, 0, 0);
        db.exported_type.set_columns(4, 4, string_index_size, string_index_size, implementation, 0);
        db.field.set_columns(2, string_index_size, blob_index_size, 0, 0, 0);
        db.field_layout.set_columns(4, db.field.index_size(), 0, 0, 0, 0);
        db.field_marshal.set_columns(has_field_marshal, blob_index_size, 0, 0, 0, 0);
        db.field_rva.set_columns(4, db.field.index_size(), 0, 0, 0, 0);
        db.file.set_columns(4, string_index_size, blob_index_size, 0, 0, 0);
        db.generic_param.set_columns(2, 2, type_or_method_def, string_index_size, 0, 0);
        db.generic_param_constraint.set_columns(db.generic_param.index_size(), type_def_or_ref, 0, 0, 0, 0);
        db.impl_map.set_columns(2, member_forwarded, string_index_size, db.module_ref.index_size(), 0, 0);
        db.interface_impl.set_columns(db.type_def.index_size(), type_def_or_ref, 0, 0, 0, 0);
        db.manifest_resource.set_columns(4, 4, string_index_size, implementation, 0, 0);
        db.member_ref.set_columns(member_ref_parent, string_index_size, blob_index_size, 0, 0, 0);
        db.method_def.set_columns(4, 2, 2, string_index_size, blob_index_size, db.param.index_size());
        db.method_impl.set_columns(db.type_def.index_size(), method_def_or_ref, method_def_or_ref, 0, 0, 0);
        db.method_semantics.set_columns(2, db.method_def.index_size(), has_semantics, 0, 0, 0);
        db.method_spec.set_columns(method_def_or_ref, blob_index_size, 0, 0, 0, 0);
        db.module.set_columns(2, string_index_size, guid_index_size, guid_index_size, guid_index_size, 0);
        db.module_ref.set_columns(string_index_size, 0, 0, 0, 0, 0);
        db.nested_class.set_columns(db.type_def.index_size(), db.type_def.index_size(), 0, 0, 0, 0);
        db.param.set_columns(2, 2, string_index_size, 0, 0, 0);
        db.property.set_columns(2, string_index_size, blob_index_size, 0, 0, 0);
        db.property_map.set_columns(db.type_def.index_size(), db.property.index_size(), 0, 0, 0, 0);
        db.standalone_sig.set_columns(blob_index_size, 0, 0, 0, 0, 0);
        db.type_def.set_columns(4, string_index_size, string_index_size, type_def_or_ref, db.field.index_size(), db.method_def.index_size());
        db.type_ref.set_columns(resolution_scope, string_index_size, string_index_size, 0, 0, 0);
        db.type_spec.set_columns(blob_index_size, 0, 0, 0, 0, 0);

        db.module.set_data(&mut view);
        db.type_ref.set_data(&mut view);
        db.type_def.set_data(&mut view);
        db.field.set_data(&mut view);
        db.method_def.set_data(&mut view);
        db.param.set_data(&mut view);
        db.interface_impl.set_data(&mut view);
        db.member_ref.set_data(&mut view);
        db.constant.set_data(&mut view);
        db.custom_attribute.set_data(&mut view);
        db.field_marshal.set_data(&mut view);
        db.decl_security.set_data(&mut view);
        db.class_layout.set_data(&mut view);
        db.field_layout.set_data(&mut view);
        db.standalone_sig.set_data(&mut view);
        db.event_map.set_data(&mut view);
        db.event.set_data(&mut view);
        db.property_map.set_data(&mut view);
        db.property.set_data(&mut view);
        db.method_semantics.set_data(&mut view);
        db.method_impl.set_data(&mut view);
        db.module_ref.set_data(&mut view);
        db.type_spec.set_data(&mut view);
        db.impl_map.set_data(&mut view);
        db.field_rva.set_data(&mut view);
        db.assembly.set_data(&mut view);
        db.assembly_processor.set_data(&mut view);
        db.assembly_os.set_data(&mut view);
        db.assembly_ref.set_data(&mut view);
        db.assembly_ref_processor.set_data(&mut view);
        db.assembly_ref_os.set_data(&mut view);
        db.file.set_data(&mut view);
        db.exported_type.set_data(&mut view);
        db.manifest_resource.set_data(&mut view);
        db.nested_class.set_data(&mut view);
        db.generic_param.set_data(&mut view);
        db.method_spec.set_data(&mut view);
        db.generic_param_constraint.set_data(&mut view);

        Ok(db)
    }

    table_fn!(assembly);
    table_fn!(assembly_os);
    table_fn!(assembly_processor);
    table_fn!(assembly_ref);
    table_fn!(assembly_ref_os);
    table_fn!(assembly_ref_processor);
    table_fn!(class_layout);
    table_fn!(constant);
    table_fn!(custom_attribute);
    table_fn!(decl_security);
    table_fn!(event);
    table_fn!(event_map);
    table_fn!(exported_type);
    table_fn!(field);
    table_fn!(field_layout);
    table_fn!(field_marshal);
    table_fn!(field_rva);
    table_fn!(file);
    table_fn!(generic_param);
    table_fn!(generic_param_constraint);
    table_fn!(impl_map);
    table_fn!(interface_impl);
    table_fn!(manifest_resource);
    table_fn!(member_ref);
    table_fn!(method_def);
    table_fn!(method_impl);
    table_fn!(method_semantics);
    table_fn!(method_spec);
    table_fn!(module);
    table_fn!(module_ref);
    table_fn!(nested_class);
    table_fn!(param);
    table_fn!(permission);
    table_fn!(property);
    table_fn!(property_map);
    table_fn!(standalone_sig);
    table_fn!(type_def);
    table_fn!(type_ref);
    table_fn!(type_spec);
}

fn section_from_rva(sections: &[ImageSectionHeader], rva: u32) -> ParseResult<&ImageSectionHeader> {
    sections.iter().find(|&s| rva >= s.virtual_address && rva < s.virtual_address + s.physical_address_or_virtual_size).ok_or_else(|| ParseError::InvalidData("Section containing RVA not found"))
}

fn offset_from_rva(section: &ImageSectionHeader, rva: u32) -> u32 {
    (rva - section.virtual_address + section.pointer_to_raw_data)
}

fn sizeof<T>() -> u32 {
    std::mem::size_of::<T>() as u32
}

fn composite_index_size(tables: &[&TableData]) -> u32 {
    fn small(row_count: u32, bits: u8) -> bool {
        (row_count as u64) < (1u64 << (16 - bits))
    }
    fn bits_needed(value: usize) -> u8 {
        let mut value = value - 1;
        let mut bits: u8 = 1;
        loop {
            value = value >> 1;
            if value == 0 {
                break;
            }
            bits += 1;
        }
        bits
    }
    let bits_needed = bits_needed(tables.len());
    if tables.iter().all(|table| small(table.row_count, bits_needed)) {
        2
    } else {
        4
    }
}

trait View {
    fn view_as<T>(&self, cli_offset: u32) -> ParseResult<&T>;
    fn view_as_slice_of<T>(&self, cli_offset: u32, len: u32) -> ParseResult<&[T]>;
    fn view_as_str(&self, cli_offset: u32) -> ParseResult<&[u8]>;
}

// TODO: remove use of unsafe blocks by simply indexing into the struct/fields with offsets
// and avoiding the structs altogether.

impl View for [u8] {
    fn view_as<T>(&self, cli_offset: u32) -> ParseResult<&T> {
        if cli_offset + sizeof::<T>() > self.len() as u32 {
            return Err(unexpected_eof());
        }
        unsafe { Ok(&*(&self[cli_offset as usize] as *const u8 as *const T)) }
    }

    fn view_as_slice_of<T>(&self, cli_offset: u32, len: u32) -> ParseResult<&[T]> {
        if cli_offset + sizeof::<T>() * len > self.len() as u32 {
            return Err(unexpected_eof());
        }
        unsafe { Ok(std::slice::from_raw_parts(&self[cli_offset as usize] as *const u8 as *const T, len as usize)) }
    }

    fn view_as_str(&self, cli_offset: u32) -> ParseResult<&[u8]> {
        let buffer = self.get(cli_offset as usize..).ok_or_else(unexpected_eof)?;
        let index = buffer.iter().position(|c| *c == b'\0').ok_or_else(unexpected_eof)?;
        Ok(&self[cli_offset as usize..cli_offset as usize + index])
    }
}

const IMAGE_DOS_SIGNATURE: u16 = 0x5A4D;
const MAGIC_PE32: u16 = 0x10B;
const MAGIC_PE32PLUS: u16 = 0x20B;
const IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR: u32 = 14;
const STORAGE_MAGIC_SIG: u32 = 0x424A5342;

#[repr(C)]
struct ImageDosHeader {
    signature: u16,
    cblp: u16,
    cp: u16,
    crlc: u16,
    cparhdr: u16,
    minalloc: u16,
    maxalloc: u16,
    ss: u16,
    sp: u16,
    csum: u16,
    ip: u16,
    cs: u16,
    lfarlc: u16,
    ovno: u16,
    res: [u16; 4],
    oemid: u16,
    oeminfo: u16,
    res2: [u16; 10],
    lfanew: i32,
}

#[repr(C)]
struct ImageFileHeader {
    machine: u16,
    number_of_sections: u16,
    time_date_stamp: u32,
    pointer_to_symbol_table: u32,
    number_of_symbols: u32,
    size_of_optional_header: u16,
    characteristics: u16,
}

#[repr(C)]
struct ImageDataDirectory {
    virtual_address: u32,
    size: u32,
}

#[repr(C)]
struct ImageOptionalHeader {
    magic: u16,
    major_linker_version: u8,
    minor_linker_version: u8,
    size_of_code: u32,
    size_of_initialized_data: u32,
    size_of_uninitialized_data: u32,
    address_of_entry_point: u32,
    base_of_code: u32,
    base_of_data: u32,
    image_base: u32,
    section_alignment: u32,
    file_alignment: u32,
    major_operating_system_version: u16,
    minor_operating_system_version: u16,
    major_image_version: u16,
    minor_image_version: u16,
    major_subsystem_version: u16,
    minor_subsystem_version: u16,
    win32_version_value: u32,
    size_of_image: u32,
    size_of_headers: u32,
    check_sum: u32,
    subsystem: u16,
    dll_characteristics: u16,
    size_of_stack_reserve: u32,
    size_of_stack_commit: u32,
    size_of_heap_reserve: u32,
    size_of_heap_commit: u32,
    loader_flags: u32,
    number_of_rva_and_sizes: u32,
    data_directory: [ImageDataDirectory; 16],
}

#[repr(C)]
struct ImageNtHeader {
    signature: u32,
    file_header: ImageFileHeader,
    optional_header: ImageOptionalHeader,
}

#[repr(C)]
struct ImageOptionalHeaderPlus {
    magic: u16,
    major_linker_version: u8,
    minor_linker_version: u8,
    size_of_code: u32,
    size_of_initialized_data: u32,
    size_of_uninitialized_data: u32,
    address_of_entry_point: u32,
    base_of_code: u32,
    image_base: u64,
    section_alignment: u32,
    file_alignment: u32,
    major_operating_system_version: u16,
    minor_operating_system_version: u16,
    major_image_version: u16,
    minor_image_version: u16,
    major_subsystem_version: u16,
    minor_subsystem_version: u16,
    win32_version_value: u32,
    size_of_image: u32,
    size_of_headers: u32,
    check_sum: u32,
    subsystem: u16,
    dll_characteristics: u16,
    size_of_stack_reserve: u64,
    size_of_stack_commit: u64,
    size_of_heap_reserve: u64,
    size_of_heap_commit: u64,
    loader_flags: u32,
    number_of_rva_and_sizes: u32,
    data_directory: [ImageDataDirectory; 16],
}

#[repr(C)]
struct ImageNtHeaderPlus {
    signature: u32,
    file_header: ImageFileHeader,
    optional_header: ImageOptionalHeaderPlus,
}

#[repr(C)]
struct ImageSectionHeader {
    name: [u8; 8],
    physical_address_or_virtual_size: u32,
    virtual_address: u32,
    size_of_raw_data: u32,
    pointer_to_raw_data: u32,
    pointer_to_relocations: u32,
    pointer_to_line_numbers: u32,
    number_of_relocations: u16,
    number_of_line_numbers: u16,
    characteristics: u32,
}

#[repr(C)]
struct ImageCorHeader {
    cb: u32,
    major_runtime_version: u16,
    minor_runtime_version: u16,
    meta_data: ImageDataDirectory,
    flags: u32,
    entry_point_token_or_entry_point_rva: u32,
    resources: ImageDataDirectory,
    strong_name_signature: ImageDataDirectory,
    code_manager_table: ImageDataDirectory,
    vtable_fixups: ImageDataDirectory,
    export_address_table_jumps: ImageDataDirectory,
    managed_native_header: ImageDataDirectory,
}

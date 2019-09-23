// TODO: remove these once Database is working.
#![allow(unused_variables)]
#![allow(dead_code)]

fn main() {
    for entry in std::fs::read_dir(r"c:\windows\system32\winmetadata").unwrap() {
        println!("{}", entry.unwrap().path().display());
    }

    let db = match Database::new(r"c:\windows\notepad.exe") {
        Ok(db) => db,
        Err(e) => return println!("{}", e),
    };

    // TODO: use 'db' here...
}

#[derive(Default)]
struct Table {
    row_count: u32,
    row_size: u32,
    columns: [(u32, u32); 6],
}

#[derive(Default)]
struct Tables {
    // TODO: remove comments once field types match comments
    type_ref: Table,                 // TypeRef
    generic_param_constraint: Table, // GenericParamConstraint
    type_spec: Table,                // TypeSpec
    type_def: Table,                 // TypeDef
    custom_attribute: Table,         // CustomAttribute
    method_def: Table,               // MethodDef
    member_ref: Table,               // MemberRef
    module: Table,                   // Module
    param: Table,                    // Param
    interface_impl: Table,           // InterfaceImpl
    constant: Table,                 // Constant
    field: Table,                    // Field
    field_marshal: Table,            // FieldMarshal
    decl_security: Table,            // DeclSecurity
    class_layout: Table,             // ClassLayout
    field_layout: Table,             // FieldLayout
    standalone_sig: Table,           // StandAloneSig
    event_map: Table,                // EventMap
    event: Table,                    // Event
    property_map: Table,             // PropertyMap
    property: Table,                 // Property
    method_semantics: Table,         // MethodSemantics
    method_impl: Table,              // MethodImpl
    module_ref: Table,               // ModuleRef
    impl_map: Table,                 // ImplMap
    field_rva: Table,                // FieldRVA
    assembly: Table,                 // Assembly
    assembly_processor: Table,       // AssemblyProcessor
    assembly_os: Table,              // AssemblyOS
    assembly_ref: Table,             // AssemblyRef
    assembly_ref_processor: Table,   // AssemblyRefProcessor
    assembly_ref_os: Table,          // AssemblyRefOS
    file: Table,                     // File
    exported_type: Table,            // ExportedType
    manifest_resource: Table,        // ManifestResource
    nested_class: Table,             // NestedClass
    generic_param: Table,            // GenericParam
    method_spec: Table,              // MethodSpec
}

struct Database {
    file: std::vec::Vec<u8>,
    strings: (u32, u32),
    blobs: (u32, u32),
    guids: (u32, u32),
    tables: Tables,
}

impl Database {
    fn new<P: AsRef<std::path::Path>>(filename: P) -> std::io::Result<Database> {
        let file = std::fs::read(filename)?;

        let dos = file.view_as::<ImageDosHeader>(0)?;

        if dos.signature != IMAGE_DOS_SIGNATURE {
            return Err(invalid_data("Invalid DOS signature"));
        }

        let pe = file.view_as::<ImageNtHeader>(dos.lfanew as u32)?;
        let com_virtual_address: u32;
        let sections: &[ImageSectionHeader];

        match pe.optional_header.magic {
            MAGIC_PE32 => {
                com_virtual_address = pe.optional_header.data_directory
                    [IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR as usize]
                    .virtual_address;
                sections = file.view_as_slice_of::<ImageSectionHeader>(
                    dos.lfanew as u32 + sizeof::<ImageNtHeader>(),
                    pe.file_header.number_of_sections as u32,
                )?;
            }

            MAGIC_PE32PLUS => {
                com_virtual_address = file
                    .view_as::<ImageNtHeaderPlus>(dos.lfanew as u32)?
                    .optional_header
                    .data_directory[IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR as usize]
                    .virtual_address;
                sections = file.view_as_slice_of::<ImageSectionHeader>(
                    dos.lfanew as u32 + sizeof::<ImageNtHeaderPlus>(),
                    pe.file_header.number_of_sections as u32,
                )?;
            }

            _ => return Err(invalid_data("Invalid optional header magic value")),
        };

        let section = section_from_rva(sections, com_virtual_address)?;
        let cli = file.view_as::<ImageCorHeader>(offset_from_rva(section, com_virtual_address))?;

        if cli.cb != sizeof::<ImageCorHeader>() {
            return Err(invalid_data("Invalid CLI header"));
        }

        let section = section_from_rva(sections, cli.meta_data.virtual_address)?;
        let offset = offset_from_rva(section, cli.meta_data.virtual_address);

        if *file.view_as::<u32>(offset)? != STORAGE_MAGIC_SIG {
            return Err(invalid_data("CLI metadata magic signature not found"));
        }

        let version_length = *file.view_as::<u32>(offset + 12)?;
        let mut slice = file.view_offset(offset + version_length + 20)?;
        let mut strings: (u32, u32) = (0, 0);
        let mut blobs: (u32, u32) = (0, 0);
        let mut guids: (u32, u32) = (0, 0);
        let mut tables: Option<&[u8]> = None;

        for _ in 0..*file.view_as::<u16>(offset + version_length + 18)? {
            let stream_offset = *slice.view_as::<u32>(0)?;
            let stream_size = *slice.view_as::<u32>(4)?;
            let stream_name = slice.view_as_str(8)?;

            match stream_name {
                b"#Strings" => strings = (offset + stream_offset, stream_size),
                b"#Blob" => blobs = (offset + stream_offset, stream_size),
                b"#GUID" => guids = (offset + stream_offset, stream_size),
                b"#~" => tables = Some(file.view_subslice(offset + stream_offset, stream_size)?),
                b"#US" => {}
                _ => return Err(invalid_data("Unknown metadata stream")),
            }

            let mut padding = 4 - stream_name.len() % 4;

            if padding == 0 {
                padding = 4;
            }

            slice = &slice[8 + stream_name.len() + padding..];
        }

        let tables = match tables {
            Some(value) => value,
            None => return Err(invalid_data("Tables stream not found")),
        };

        let heap_sizes = *tables.view_as::<u8>(6)?;
        let string_index_size = if (heap_sizes & 1) == 1 { 4 } else { 2 };
        let guid_index_size = if (heap_sizes >> 1 & 1) == 1 { 4 } else { 2 };
        let blob_index_size = if (heap_sizes >> 2 & 1) == 1 { 4 } else { 2 };

        let valid_bits = *tables.view_as::<u64>(8)?;
        slice = tables.view_offset(24)?;
        let mut tables = Tables::default();

        for i in 0..64 {
            if (valid_bits >> i & 1) == 0 {
                continue;
            }

            let row_count = *slice.view_as::<u32>(0)?;
            slice = slice.view_offset(4)?;

            match i {
                0x00 => tables.module.row_count = row_count,
                0x01 => tables.type_ref.row_count = row_count,
                0x02 => tables.type_def.row_count = row_count,
                0x04 => tables.field.row_count = row_count,
                0x06 => tables.method_def.row_count = row_count,
                0x08 => tables.param.row_count = row_count,
                0x09 => tables.interface_impl.row_count = row_count,
                0x0a => tables.member_ref.row_count = row_count,
                0x0b => tables.constant.row_count = row_count,
                0x0c => tables.custom_attribute.row_count = row_count,
                0x0d => tables.field_marshal.row_count = row_count,
                0x0e => tables.decl_security.row_count = row_count,
                0x0f => tables.class_layout.row_count = row_count,
                0x10 => tables.field_layout.row_count = row_count,
                0x11 => tables.standalone_sig.row_count = row_count,
                0x12 => tables.event_map.row_count = row_count,
                0x14 => tables.event.row_count = row_count,
                0x15 => tables.property_map.row_count = row_count,
                0x17 => tables.property.row_count = row_count,
                0x18 => tables.method_semantics.row_count = row_count,
                0x19 => tables.method_impl.row_count = row_count,
                0x1a => tables.module_ref.row_count = row_count,
                0x1b => tables.type_spec.row_count = row_count,
                0x1c => tables.impl_map.row_count = row_count,
                0x1d => tables.field_rva.row_count = row_count,
                0x20 => tables.assembly.row_count = row_count,
                0x21 => tables.assembly_processor.row_count = row_count,
                0x22 => tables.assembly_os.row_count = row_count,
                0x23 => tables.assembly_ref.row_count = row_count,
                0x24 => tables.assembly_ref_processor.row_count = row_count,
                0x25 => tables.assembly_ref_os.row_count = row_count,
                0x26 => tables.file.row_count = row_count,
                0x27 => tables.exported_type.row_count = row_count,
                0x28 => tables.manifest_resource.row_count = row_count,
                0x29 => tables.nested_class.row_count = row_count,
                0x2a => tables.generic_param.row_count = row_count,
                0x2b => tables.method_spec.row_count = row_count,
                0x2c => tables.generic_param_constraint.row_count = row_count,
                _ => return Err(invalid_data("Unknown metadata table")),
            };
        }

        let empty_table = Table::default();
        let has_constant = composite_index_size(&[&tables.field, &tables.param, &tables.property]);
        let type_def_or_ref =
            composite_index_size(&[&tables.type_def, &tables.type_ref, &tables.type_spec]);
        let has_custom_attribute = composite_index_size(&[
            &tables.method_def,
            &tables.field,
            &tables.type_ref,
            &tables.type_def,
            &tables.param,
            &tables.interface_impl,
            &tables.member_ref,
            &tables.module,
            &tables.property,
            &tables.event,
            &tables.standalone_sig,
            &tables.module_ref,
            &tables.type_spec,
            &tables.assembly,
            &tables.assembly_ref,
            &tables.file,
            &tables.exported_type,
            &tables.manifest_resource,
            &tables.generic_param,
            &tables.generic_param_constraint,
            &tables.method_spec,
        ]);
        let has_field_marshal = composite_index_size(&[&tables.field, &tables.param]);
        let has_decl_security =
            composite_index_size(&[&tables.type_def, &tables.method_def, &tables.assembly]);
        let member_ref_parent = composite_index_size(&[
            &tables.type_def,
            &tables.type_ref,
            &tables.module_ref,
            &tables.method_def,
            &tables.type_spec,
        ]);
        let has_semantics = composite_index_size(&[&tables.event, &tables.property]);
        let method_def_or_ref = composite_index_size(&[&tables.method_def, &tables.member_ref]);
        let member_forwarded = composite_index_size(&[&tables.field, &tables.method_def]);
        let implementation =
            composite_index_size(&[&tables.file, &tables.assembly_ref, &tables.exported_type]);
        let custom_attribute_type = composite_index_size(&[
            &tables.method_def,
            &tables.member_ref,
            &empty_table,
            &empty_table,
            &empty_table,
        ]);
        let resolution_scope = composite_index_size(&[
            &tables.module,
            &tables.module_ref,
            &tables.assembly_ref,
            &tables.type_ref,
        ]);
        let type_or_method_def = composite_index_size(&[&tables.type_def, &tables.method_def]);

        Ok(Database {
            file: file,
            strings: strings,
            blobs: blobs,
            guids: guids,
            tables: tables,
        })
    }
}

fn invalid_data(message: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, message)
}

fn unexpected_eof() -> std::io::Error {
    std::io::Error::from(std::io::ErrorKind::UnexpectedEof)
}

fn section_from_rva(
    sections: &[ImageSectionHeader],
    rva: u32,
) -> std::io::Result<&ImageSectionHeader> {
    match sections.iter().find(|&s| {
        rva >= s.virtual_address && rva < s.virtual_address + s.physical_address_or_virtual_size
    }) {
        Some(section) => Ok(section),
        None => Err(invalid_data("Section containing RVA not found")),
    }
}

fn offset_from_rva(section: &ImageSectionHeader, rva: u32) -> u32 {
    (rva - section.virtual_address + section.pointer_to_raw_data)
}

fn sizeof<T>() -> u32 {
    std::mem::size_of::<T>() as u32
}

fn composite_index_size(tables: &[&Table]) -> u8 {
    let small = |row_count: u32, bits: u8| (row_count as u64) < (1u64 << (16 - bits));

    let bits_needed = |value| {
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
    };

    let bits_needed = bits_needed(tables.len());

    if tables
        .iter()
        .all(|table| small(table.row_count, bits_needed))
    {
        2
    } else {
        4
    }
}

trait View {
    fn view_as<T>(&self, offset: u32) -> std::io::Result<&T>;
    fn view_as_slice_of<T>(&self, offset: u32, len: u32) -> std::io::Result<&[T]>;
    fn view_as_str(&self, offset: u32) -> std::io::Result<&[u8]>;
    fn view_offset(&self, offset: u32) -> std::io::Result<&[u8]>;
    fn view_subslice(&self, offset: u32, size: u32) -> std::io::Result<&[u8]>;
}

impl View for [u8] {
    fn view_as<T>(&self, offset: u32) -> std::io::Result<&T> {
        if offset + sizeof::<T>() > self.len() as u32 {
            return Err(unexpected_eof());
        }

        unsafe { Ok(&*(&self[offset as usize] as *const u8 as *const T)) }
    }

    fn view_as_slice_of<T>(&self, offset: u32, len: u32) -> std::io::Result<&[T]> {
        if offset + sizeof::<T>() * len > self.len() as u32 {
            return Err(unexpected_eof());
        }

        unsafe {
            Ok(std::slice::from_raw_parts(
                &self[offset as usize] as *const u8 as *const T,
                len as usize,
            ))
        }
    }

    fn view_as_str(&self, offset: u32) -> std::io::Result<&[u8]> {
        match self.view_offset(offset)?.iter().position(|c| *c == b'\0') {
            Some(index) => Ok(&self[offset as usize..offset as usize + index]),
            None => Err(unexpected_eof()),
        }
    }

    fn view_offset(&self, offset: u32) -> std::io::Result<&[u8]> {
        match self.get(offset as usize..) {
            Some(slice) => Ok(slice),
            None => Err(unexpected_eof()),
        }
    }

    fn view_subslice(&self, offset: u32, size: u32) -> std::io::Result<&[u8]> {
        match self.get(offset as usize..(offset + size) as usize) {
            Some(slice) => Ok(slice),
            None => Err(unexpected_eof()),
        }
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

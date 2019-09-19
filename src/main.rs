fn invalid_data(message: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, message)
}

fn unexpected_eof() -> std::io::Error {
    std::io::Error::from(std::io::ErrorKind::UnexpectedEof)
}

fn section_from_rva(
    sections: &[image_section_header],
    rva: u32,
) -> std::io::Result<&image_section_header> {
    match sections.iter().find(|&s| {
        rva >= s.virtual_address && rva < s.virtual_address + s.physical_address_or_virtual_size
    }) {
        Some(section) => Ok(section),
        None => Err(invalid_data("Section containing RVA not found")),
    }
}

fn offset_from_rva(section: &image_section_header, rva: u32) -> usize {
    (rva - section.virtual_address + section.pointer_to_raw_data) as usize
}

trait View {
    fn view_as<T>(&self, pos: usize) -> std::io::Result<&T>;
    fn view_as_slice_of<T>(&self, pos: usize, len: usize) -> std::io::Result<&[T]>;
    fn view_as_str(&self, pos: usize) -> std::io::Result<&[u8]>;
    fn offset(&self, pos: usize) -> std::io::Result<&[u8]>;
}

impl View for [u8] {
    fn view_as<T>(&self, pos: usize) -> std::io::Result<&T> {
        if pos + std::mem::size_of::<T>() > self.len() {
            return Err(unexpected_eof());
        }

        unsafe { Ok(&*(&self[pos] as *const u8 as *const T)) }
    }

    fn view_as_slice_of<T>(&self, pos: usize, len: usize) -> std::io::Result<&[T]> {
        if pos + std::mem::size_of::<T>() * len > self.len() {
            return Err(unexpected_eof());
        }

        unsafe {
            Ok(std::slice::from_raw_parts(
                &self[pos] as *const u8 as *const T,
                len,
            ))
        }
    }

    fn view_as_str(&self, pos: usize) -> std::io::Result<&[u8]> {
        match self.offset(pos)?.iter().position(|c| *c == b'\0') {
            Some(index) => Ok(&self[pos..pos + index]),
            None => Err(unexpected_eof()),
        }
    }

    fn offset(&self, pos: usize) -> std::io::Result<&[u8]> {
        match self.get(pos..) {
            Some(slice) => Ok(slice),
            None => Err(unexpected_eof()),
        }
    }
}

struct Database {
    buffer: std::vec::Vec<u8>,
}

impl Database {
    pub fn new<P: AsRef<std::path::Path>>(filename: P) -> std::io::Result<Database> {
        let buffer = std::fs::read(filename)?;
        let file = buffer.as_slice();
        let dos = file.view_as::<image_dos_header>(0)?;

        if dos.e_signature != IMAGE_DOS_SIGNATURE {
            return Err(invalid_data("Invalid DOS signature"));
        }

        let pe = file.view_as::<image_nt_headers32>(dos.e_lfanew as usize)?;
        let com_virtual_address: u32;
        let sections: &[image_section_header];

        match pe.optional_header.magic {
            MAGIC_PE32 => {
                com_virtual_address = pe.optional_header.data_directory
                    [IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR]
                    .virtual_address;
                sections = file.view_as_slice_of::<image_section_header>(
                    dos.e_lfanew as usize + std::mem::size_of::<image_nt_headers32>(),
                    pe.file_header.number_of_sections as usize,
                )?;
            }

            MAGIC_PE32PLUS => {
                com_virtual_address = file
                    .view_as::<image_nt_headers32plus>(dos.e_lfanew as usize)?
                    .optional_header
                    .data_directory[IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR]
                    .virtual_address;
                sections = file.view_as_slice_of::<image_section_header>(
                    dos.e_lfanew as usize + std::mem::size_of::<image_nt_headers32plus>(),
                    pe.file_header.number_of_sections as usize,
                )?;
            }

            _ => return Err(invalid_data("Invalid optional header magic value")),
        };

        let section = section_from_rva(sections, com_virtual_address)?;
        let cli =
            file.view_as::<image_cor20_header>(offset_from_rva(section, com_virtual_address))?;

        if cli.cb as usize != std::mem::size_of::<image_cor20_header>() {
            return Err(invalid_data("Invalid CLI header"));
        }

        let section = section_from_rva(sections, cli.meta_data.virtual_address)?;
        let offset = offset_from_rva(section, cli.meta_data.virtual_address);

        if *file.view_as::<u32>(offset)? != STORAGE_MAGIC_SIG {
            return Err(invalid_data("CLI metadata magic signature not found"));
        }

        let version_length = *file.view_as::<u32>(offset + 12)? as usize;
        let mut slice = file.offset(offset + version_length + 20)?;
        let tables: &[u8];

        for _ in 0..*file.view_as::<u16>(offset + version_length + 18)? {
            let stream_offset = *slice.view_as::<u32>(0)?;
            let stream_size = *slice.view_as::<u32>(4)?;
            let stream_name = slice.view_as_str(8)?;

            match stream_name {
                b"#Strings" => println!("strings"),
                b"#Blob" => println!("blob"),
                b"#GUID" => println!("guid"),
                b"#~" => println!("tables"),
                b"#US" => println!("us"),
                _ => return Err(invalid_data("Unknown metadata stream")),
            }

            let mut padding = 4 - stream_name.len() % 4;

            if padding == 0 {
                padding = 4;
            }

            slice = &slice[8 + stream_name.len() + padding..];
        }

        Ok(Database { buffer: buffer })
    }
}

fn main() {
    for entry in std::fs::read_dir(r#"c:\windows\system32\winmetadata"#).unwrap() {
        println!("{}", entry.unwrap().path().display());
    }

    let db = Database::new(r#"c:\windows\system32\winmetadata\Windows.Foundation.winmd"#).unwrap();
}

const IMAGE_DOS_SIGNATURE: u16 = 0x5A4D;
const MAGIC_PE32: u16 = 0x10B;
const MAGIC_PE32PLUS: u16 = 0x20B;
const IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR: usize = 14;
const STORAGE_MAGIC_SIG: u32 = 0x424A5342;

#[repr(C)]
struct image_dos_header {
    pub e_signature: u16,
    pub e_cblp: u16,
    pub e_cp: u16,
    pub e_crlc: u16,
    pub e_cparhdr: u16,
    pub e_minalloc: u16,
    pub e_maxalloc: u16,
    pub e_ss: u16,
    pub e_sp: u16,
    pub e_csum: u16,
    pub e_ip: u16,
    pub e_cs: u16,
    pub e_lfarlc: u16,
    pub e_ovno: u16,
    pub e_res: [u16; 4],
    pub e_oemid: u16,
    pub e_oeminfo: u16,
    pub e_res2: [u16; 10],
    pub e_lfanew: i32,
}

#[repr(C)]
struct image_file_header {
    pub machine: u16,
    pub number_of_sections: u16,
    pub time_date_stamp: u32,
    pub pointer_to_symbol_table: u32,
    pub number_of_symbols: u32,
    pub size_of_optional_header: u16,
    pub characteristics: u16,
}

#[repr(C)]
struct image_data_directory {
    pub virtual_address: u32,
    pub size: u32,
}

#[repr(C)]
struct image_optional_header32 {
    pub magic: u16,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: u32,
    pub size_of_initialized_data: u32,
    pub size_of_uninitialized_data: u32,
    pub address_of_entry_point: u32,
    pub base_of_code: u32,
    pub base_of_data: u32,
    pub image_base: u32,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub major_operating_system_version: u16,
    pub minor_operating_system_version: u16,
    pub major_image_version: u16,
    pub minor_image_version: u16,
    pub major_subsystem_version: u16,
    pub minor_subsystem_version: u16,
    pub win32_version_value: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub check_sum: u32,
    pub subsystem: u16,
    pub dll_characteristics: u16,
    pub size_of_stack_reserve: u32,
    pub size_of_stack_commit: u32,
    pub size_of_heap_reserve: u32,
    pub size_of_heap_commit: u32,
    pub loader_flags: u32,
    pub number_of_rva_and_sizes: u32,
    pub data_directory: [image_data_directory; 16],
}

#[repr(C)]
struct image_nt_headers32 {
    pub signature: u32,
    pub file_header: image_file_header,
    pub optional_header: image_optional_header32,
}

#[repr(C)]
struct image_optional_header32plus {
    pub magic: u16,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: u32,
    pub size_of_initialized_data: u32,
    pub size_of_uninitialized_data: u32,
    pub address_of_entry_point: u32,
    pub base_of_code: u32,
    pub image_base: u64,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub major_operating_system_version: u16,
    pub minor_operating_system_version: u16,
    pub major_image_version: u16,
    pub minor_image_version: u16,
    pub major_subsystem_version: u16,
    pub minor_subsystem_version: u16,
    pub win32_version_value: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub check_sum: u32,
    pub subsystem: u16,
    pub dll_characteristics: u16,
    pub size_of_stack_reserve: u64,
    pub size_of_stack_commit: u64,
    pub size_of_heap_reserve: u64,
    pub size_of_heap_commit: u64,
    pub loader_flags: u32,
    pub number_of_rva_and_sizes: u32,
    pub data_directory: [image_data_directory; 16],
}

#[repr(C)]
struct image_nt_headers32plus {
    signature: u32,
    file_header: image_file_header,
    optional_header: image_optional_header32plus,
}

#[repr(C)]
struct image_section_header {
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
struct image_cor20_header {
    cb: u32,
    major_runtime_version: u16,
    minor_runtime_version: u16,
    meta_data: image_data_directory,
    flags: u32,
    entry_point_token_or_entry_point_rva: u32,
    resources: image_data_directory,
    strong_name_signature: image_data_directory,
    code_manager_table: image_data_directory,
    vtable_fixups: image_data_directory,
    export_address_table_jumps: image_data_directory,
    managed_native_header: image_data_directory,
}

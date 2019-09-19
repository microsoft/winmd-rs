use std::io::{Error, ErrorKind};
use std::mem::size_of;

trait Slice {
    fn as_struct_ptr<T>(&self) -> std::io::Result<*const T>;
}

fn invalid_data(message: &str) -> std::io::Error {
    Error::new(ErrorKind::InvalidData, message)
}

impl Slice for &[u8] {
    fn as_struct_ptr<T>(&self) -> std::io::Result<*const T> {
        if self.len() < size_of::<T>() {
            return Err(invalid_data("Buffer too small"));
        }

        Ok(self.as_ptr() as *const T)
    }
}

fn run() -> std::io::Result<()> {
    let file = std::fs::read(r#"c:\windows\system32\winmetadata\Windows.Foundation.winmd"#)?;
    let slice = file.as_slice();
    let dos = slice.as_struct_ptr::<image_dos_header>()?;

    if unsafe { (*dos).e_signature != 0x5A4D } {
        return Err(invalid_data("Invalid DOS signature"));
    }

    Ok(())
}

fn main() {
    for entry in std::fs::read_dir(r#"c:\windows\system32\winmetadata"#).unwrap() {
        println!("{}", entry.unwrap().path().display());
    }

    run().unwrap();
}

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

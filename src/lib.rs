//! A Windows Metadata (winmd) parser
mod file;
pub mod parsed;
mod traits;
mod type_reader;

pub use file::{File, TableIndex};
pub use type_reader::TypeReader;

use std::io;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    InvalidData(&'static str),
}

pub fn unexpected_eof() -> Error {
    Error::Io(io::Error::from(io::ErrorKind::UnexpectedEof))
}

pub fn unsupported_blob() -> Error {
    Error::InvalidData("Unsupported blob")
}

impl std::convert::From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

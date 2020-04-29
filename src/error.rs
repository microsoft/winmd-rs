use std::{error, fmt, io};

#[derive(Debug)]
pub enum ParseError {
    Io(io::Error),
    MissingType(String),
    MissingAttribute,
    InvalidFile,
    InvalidTypeName,
    InvalidBlob,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => fmt::Display::fmt(err, f),
            Self::MissingType(ty) => write!(f, "Missing type: {}", ty),
            Self::MissingAttribute => write!(f, "Missing attribute"),
            Self::InvalidFile => write!(f, "Invalid file"),
            Self::InvalidTypeName => write!(f, "Invalid type name"),
            Self::InvalidBlob => write!(f, "Invalid blob"),
        }
    }
}

impl error::Error for ParseError {}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    ParseError(ParseError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => fmt::Display::fmt(err, f),
            Self::ParseError(err) => fmt::Display::fmt(err, f),
        }
    }
}

impl error::Error for Error {}

pub type ParseResult<T> = Result<T, ParseError>;

impl std::convert::From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl std::convert::From<ParseError> for Error {
    fn from(error: ParseError) -> Self {
        Error::ParseError(error)
    }
}

impl std::convert::From<io::Error> for ParseError {
    fn from(error: io::Error) -> Self {
        ParseError::Io(error)
    }
}

impl std::convert::From<ParseError> for std::fmt::Error {
    fn from(_: ParseError) -> Self {
        std::fmt::Error {}
    }
}

use std::io;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    ParseError(ParseError),
}

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

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    Io(io::Error),
    InvalidData(&'static str),
}

pub fn unexpected_eof() -> ParseError {
    ParseError::Io(io::Error::from(io::ErrorKind::UnexpectedEof))
}

pub fn unsupported_blob() -> ParseError {
    ParseError::InvalidData("Unsupported blob")
}

impl std::convert::From<io::Error> for ParseError {
    fn from(error: io::Error) -> Self {
        ParseError::Io(error)
    }
}

use std::io;
use std::error::Error;
use std::fmt;
use hex::FromHexError;
use std::num::ParseIntError;

#[derive(Debug, Clone, PartialEq)]
pub enum MSError {
    IOError(String),
    HexDecodingError(String),
    HexEncodingError(String),
    ParseIntError(String),
}

impl fmt::Display for MSError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Magic Switcheroo Error")?;
        match self {
            MSError::IOError(msg) => write!(f, "IOError: {msg}"),
            MSError::HexDecodingError(msg) => write!(f, "HexDecodingError: {msg}"),
            MSError::HexEncodingError(msg) => write!(f, "HexEncodingError: {msg}"),
            MSError::ParseIntError(msg) => write!(f, "ParseIntError: {msg}"),
        }
    }
}

impl Error for MSError {}

impl From<FromHexError> for MSError {
    fn from(error: FromHexError) -> Self {
        MSError::HexDecodingError(format!("HexDecodingError: {}", if let Some(source)= error.source() {source} else { &error } ))
    }
}

impl From<ParseIntError> for MSError {
    fn from(error: ParseIntError) -> Self {
        MSError::ParseIntError(format!("ParseInterror: {}", error))
    }
}

impl From<io::Error> for MSError {
    fn from(error: io::Error) -> Self {
        MSError::IOError(format!("{}", error))
    }
}

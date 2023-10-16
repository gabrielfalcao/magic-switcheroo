use std::io;
use std::error::Error;
use std::fmt;
use hex::FromHexError;
use std::num::ParseIntError;
use crate::ram::VecsException;


#[derive(Debug, Clone, PartialEq)]
pub enum MSError {
    IOError(String),
    HexDecodingError(String),
    HexEncodingError(String),
    ParseIntError(String),
    VecsError(VecsException),
}

impl fmt::Display for MSError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Magic Switcheroo Error")?;
        match self {
            MSError::IOError(msg) => write!(f, "IOError: {msg}"),
            MSError::HexDecodingError(msg) => write!(f, "HexDecodingError: {msg}"),
            MSError::HexEncodingError(msg) => write!(f, "HexEncodingError: {msg}"),
            MSError::VecsError(e) => write!(f, "{}", match e {
                VecsException::PatternNotFound(pattern) => format!("pattern not found: {}", hex::encode(pattern)),
                VecsException::NotAllOccurrencesReplaced((pattern, occrsf, occrse)) => format!("not enought ocurrences found for {} ({}/{})", hex::encode(pattern), occrsf, occrse),
            }),
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

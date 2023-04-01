use std::{fmt, io};

#[derive(Debug)]
pub enum VmError {
    ReadingError(io::Error),
    ParsingError(fmt::Error),
    TableInitializationError,
    InvalidStateError,
    WritingError(io::Error),
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            VmError::ReadingError(ref err) => write!(f, "Cannot read the file: {}", err),
            VmError::ParsingError(ref err) => write!(f, "Cannot parse current line: {}", err),
            VmError::WritingError(ref err) => write!(f, "Cannot write into file: {}", err),
            VmError::InvalidStateError => write!(f, "Translator is in invalid state"),
            VmError::TableInitializationError => write!(f, "Table cannot be initialized"),
        }
    }
}

impl From<io::Error> for VmError {
    fn from(err: io::Error) -> Self {
        VmError::ReadingError(err) 
    }
}


impl From<fmt::Error> for VmError {
    fn from(err: fmt::Error) -> Self {
        VmError::ParsingError(err)
    }
}

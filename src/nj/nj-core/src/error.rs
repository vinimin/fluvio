
use std::fmt;
use std::io::Error as IoError;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum NjError {
    IoError(IoError),
    InvalidType,
    Utf8Error(FromUtf8Error)
}

impl From<IoError> for NjError {
    fn from(error: IoError) -> Self {
        Self::IoError(error)
    }
}

impl From<FromUtf8Error> for NjError {
    fn from(error: FromUtf8Error) -> Self {
        Self::Utf8Error(error)
    }
}


impl fmt::Display for NjError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IoError(err) => write!(f, "{}", err),
            Self::InvalidType => write!(f,"invalid type"),
            Self::Utf8Error(err) => write!(f,"ut8 error: {}",err)
        }
    }
}

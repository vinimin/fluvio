
use std::fmt;
use std::io::Error as IoError;


#[derive(Debug)]
pub enum NjError {
    IoError(IoError),
    NotDoubleType
}

impl From<IoError> for NjError {
    fn from(error: IoError) -> Self {
        Self::IoError(error)
    }
}


impl fmt::Display for NjError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IoError(err) => write!(f, "{}", err),
            Self::NotDoubleType => write!(f,"invalid double data type"),
        }
    }
}

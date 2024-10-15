use std::io;
use std::fmt;

#[derive(Debug)]
pub enum ExtransError {
    SetupError(String),
    EncodeError(String),
    PropertyNotFound(String),
    ParseError(String)
}

impl fmt::Display for ExtransError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExtransError::SetupError(msg) => write!(f, "setup Error: {}", msg),
            ExtransError::EncodeError(msg) => write!(f, "encode Error: {}", msg),
            ExtransError::PropertyNotFound(msg) => write!(f, "property not found: {}", msg),
            ExtransError::ParseError(msg) => write!(f, "failed to parse property: {}", msg),
        }
    }
}

impl From<io::Error> for ExtransError {
    fn from(err: io::Error) -> ExtransError {
        ExtransError::SetupError(err.to_string())
    }
}
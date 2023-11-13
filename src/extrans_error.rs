use std::io;
use std::fmt;

pub enum ExtransError {
    SetupError(String),
    EncodeError(String)
}

impl fmt::Display for ExtransError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExtransError::SetupError(msg) => write!(f, "Setup Error: {}", msg),
            ExtransError::EncodeError(msg) => write!(f, "Encode Error: {}", msg),
        }
    }
}

impl From<io::Error> for ExtransError {
    fn from(err: io::Error) -> ExtransError {
        ExtransError::SetupError(err.to_string())
    }
}

use std::error::Error;
use std::fmt::{ Display, Result, Formatter };

#[derive(Debug, PartialEq)]
pub enum EnigmaError {
    InvalidLength
}

impl Display for EnigmaError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            EnigmaError::InvalidLength => write!(f, "Invalid length."),
        }
    }
}

impl Error for EnigmaError {

    fn description(&self) -> &str {
        match *self {
            EnigmaError::InvalidLength => "Invalid length",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
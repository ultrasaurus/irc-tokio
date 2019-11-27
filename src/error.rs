use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    MessageMissingCommand,
    MessageStringTooLong,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Error::MessageMissingCommand => write!(f, "Message Missing Command"),
            Error::MessageStringTooLong => write!(f, "Message String To Long"),
        }
    }
}

impl std::error::Error for Error {}

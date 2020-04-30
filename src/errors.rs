//! The `errors` module defines the common error types.

use std::error;
use std::fmt;
use std::io;

use super::Result;

/// `Error` provides an enumeration of all possible errors reported by Sonata.
#[derive(Debug)]
pub enum Error {
    /// The stream contained malformed data and could not be parsed.
    ParseError(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::ParseError(ref msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::ParseError(_) => None,
        }
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::ParseError("unable to parse color hex")
    }
}

/// function to create a decode error.
pub fn parse_error<T>(desc: &'static str) -> Result<T> {
    Err(Error::ParseError(desc))
}

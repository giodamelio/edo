//! Errors

use std::fmt;
use std::error::Error;

/// The error our format function might through
#[derive(Debug, PartialEq)]
pub enum EdoError {
    /// An error occured while parsing the template
    ParsingError,
}

impl fmt::Display for EdoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EdoError::ParsingError => write!(f, "Parsing error"),
        }
    }
}

impl Error for EdoError {
    fn description(&self) -> &str {
        match *self {
            EdoError::ParsingError => "Parsing error",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            // Our custom error doesn't have an underlying cause,
            // but we could modify it so that it does.
            EdoError::ParsingError => None,
        }
    }
}

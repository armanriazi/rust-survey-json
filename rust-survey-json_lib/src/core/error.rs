#![allow(dead_code, unused_variables)]
use std::fmt::{self, Formatter};

#[derive(Debug)]
pub enum StringError {
    StringParse(std::string::ParseError),
    InvalidOption(String),
    Other,
}

#[derive(Debug)]
pub enum SurveyValidationError {
    MismatchedIndex,
    InvalidFormat,
    InvalidInput,
}

#[derive(Debug)]
pub enum CustomError {
    String(StringError),
    SerdeJson(serde_json::Error),
    IO(std::io::Error),
    SurveyValidation(SurveyValidationError),
    InvalidOption(String),
    Other,
}

impl fmt::Display for SurveyValidationError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            SurveyValidationError::InvalidFormat => {
                write!(f, "SurveyValidation:InvalidFormat Error")
            }
            SurveyValidationError::InvalidInput => write!(f, "SurveyValidation:InvalidInput Error"),
            SurveyValidationError::MismatchedIndex => {
                write!(f, "SurveyValidation:MismatchedIndex Error")
            }
        }
    }
}
// Allow the use of "{}" format specifier
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::String(_) => write!(f, "\nString Error"),
            CustomError::SerdeJson(ref cause) => write!(f, "\nSerdeJson Error: {}", cause),
            CustomError::IO(ref cause) => write!(f, "\nIO Error: {}", cause),
            CustomError::SurveyValidation(ref cause) => {
                write!(f, "\nSurveyValidation Error: {}", cause)
            }
            CustomError::Other => write!(f, "\nUnknown error!"),
            CustomError::InvalidOption(_) => write!(f, "\nInvalid Option!"),
        }
    }
}
impl std::error::Error for CustomError {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            CustomError::String(_) => None,
            CustomError::SerdeJson(ref cause) => Some(cause),
            CustomError::IO(ref cause) => Some(cause),
            CustomError::SurveyValidation(_) => None,
            CustomError::Other => None,
            CustomError::InvalidOption(_) => None,
        }
    }
}

impl From<std::string::ParseError> for StringError {
    fn from(cause: std::string::ParseError) -> StringError {
        StringError::InvalidOption(cause.to_string())
    }
}

impl From<serde_json::Error> for CustomError {
    fn from(cause: serde_json::Error) -> CustomError {
        CustomError::SerdeJson(cause)
    }
}
impl From<std::io::Error> for CustomError {
    fn from(cause: std::io::Error) -> CustomError {
        CustomError::IO(cause)
    }
}
impl From<SurveyValidationError> for CustomError {
    fn from(cause: SurveyValidationError) -> CustomError {
        CustomError::SurveyValidation(cause)
    }
}

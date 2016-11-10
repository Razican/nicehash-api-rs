//! Errors for nicehash.com API.

use std::result::Result as StdResult;
use std::error::Error as StdError;
use std::fmt;
use std::num::{ParseFloatError, ParseIntError};

use hyper::error::Error as HyperError;
use serde_json::error::Error as JsonError;
use semver::SemVerError;

/// Result type for nicehash.com API.
pub type Result<T> = StdResult<T, Error>;

/// Main error enum for nicehash.com API.
#[derive(Debug)]
pub enum Error {
    Hyper(HyperError),
    Json(JsonError),
    SemVer(SemVerError),
    Api(String),
    Result(String),
    ParseAlgorithm(String),
    ParseOrderType(String),
    ParseFloat(ParseFloatError),
    ParseInt(ParseIntError),
}

impl From<HyperError> for Error {
    fn from(err: HyperError) -> Error {
        Error::Hyper(err)
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Error {
        Error::Json(err)
    }
}

impl From<SemVerError> for Error {
    fn from(err: SemVerError) -> Error {
        Error::SemVer(err)
    }
}

impl From<ParseFloatError> for Error {
    fn from(err: ParseFloatError) -> Error {
        Error::ParseFloat(err)
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Error {
        Error::ParseInt(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "an error occurred: {}", self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Hyper(ref e) => e.description(),
            Error::Json(ref e) => e.description(),
            Error::SemVer(ref e) => e.description(),
            Error::ParseFloat(ref e) => e.description(),
            Error::ParseInt(ref e) => e.description(),
            Error::Api(ref d) |
            Error::ParseAlgorithm(ref d) |
            Error::ParseOrderType(ref d) |
            Error::Result(ref d) => d,
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Hyper(ref e) => Some(e),
            Error::Json(ref e) => Some(e),
            Error::SemVer(ref e) => Some(e),
            Error::ParseFloat(ref e) => Some(e),
            Error::ParseInt(ref e) => Some(e),
            _ => None,
        }
    }
}

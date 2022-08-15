use core::fmt;
use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    BadUrl(String),
    ResponseError(String),
    ExpandError(String)
}


impl std::error::Error for Error {
    fn description(&self) -> &str {
        use self::Error::*;
        match &*self {
            BadUrl(_s) => "Url {_s} is not valid",
            ResponseError(_s) => "Got response error: {_s}",
            ExpandError(_s) => "Got expand error: {_s}",
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        use self::Error::*;

        match &*self {
            BadUrl(_s) => write!(f, "Url {_s} is not valid"),
            ResponseError(_s) => write!(f, "Got response error: {_s}"),
            ExpandError(_s) => write!(f, "Got expand error: {_s}"),
        }
    }
}
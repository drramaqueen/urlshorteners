use core::fmt;
use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    BadUrl(String),
    ResponseError(String),
    ShortenError(String),
    ExpandError(String),
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        use self::Error::*;
        match self {
            BadUrl(_) => "Url is not valid",
            ResponseError(_) => "Got response error",
            ShortenError(_) => "Got shorten error",
            ExpandError(_) => "Got expand error",
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        use self::Error::*;

        match self {
            BadUrl(s) => write!(f, "Url {} is not valid", s),
            ResponseError(s) => write!(f, "Got response error: {}", s),
            ShortenError(s) => write!(f, "Got shorten error: {}", s),
            ExpandError(s) => write!(f, "Got expand error: {}", s),
        }
    }
}

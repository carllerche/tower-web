use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    NotFound,
    Internal,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        use self::Error::*;

        match *self {
            NotFound => "not found",
            Internal => "internal error",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        error::Error::description(self).fmt(fmt)
    }
}

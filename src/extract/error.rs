use self::Kind::*;

use error::ErrorKind;

#[derive(Debug)]
pub struct Error {
    kind: Kind,
}

#[derive(Debug)]
enum Kind {
    Missing,
    Invalid(String),
    Web(::Error),
}

impl Error {
    pub fn missing_argument() -> Error {
        Error { kind: Missing }
    }

    pub fn is_missing_argument(&self) -> bool {
        match self.kind {
            Missing => true,
            _ => false,
        }
    }

    pub fn invalid_argument<T: ToString>(reason: &T) -> Error {
        Error { kind: Invalid(reason.to_string()) }
    }

    pub fn is_invalid_argument(&self) -> bool {
        match self.kind {
            Invalid(_) => true,
            _ => false,
        }
    }

    pub fn web(err: ::Error) -> Error {
        Error { kind: Web(err) }
    }

    pub(crate) fn internal_error() -> Error {
        Error::web(ErrorKind::internal().into())
    }
}

impl From<Error> for ::Error {
    fn from(err: Error) -> ::Error {
        match err.kind {
            Missing | Invalid(_) => ErrorKind::bad_request().into(),
            Web(err) => err,
        }
    }
}

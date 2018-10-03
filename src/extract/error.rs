use self::Kind::*;

use error::ErrorKind;

/// Errors that can happen while extracting data from an HTTP request.
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
    /// The data is missing from the HTTP request.
    pub fn missing_argument() -> Error {
        Error { kind: Missing }
    }

    /// Returns `true` when the error represents missing data from the HTTP
    /// request.
    pub fn is_missing_argument(&self) -> bool {
        match self.kind {
            Missing => true,
            _ => false,
        }
    }

    /// The data is in an invalid format and cannot be extracted.
    pub fn invalid_argument<T: ToString>(reason: &T) -> Error {
        Error { kind: Invalid(reason.to_string()) }
    }

    /// Returns `true` when the data is in an invalid format and cannot be
    /// extracted.
    pub fn is_invalid_argument(&self) -> bool {
        match self.kind {
            Invalid(_) => true,
            _ => false,
        }
    }

    pub(crate) fn web(err: ::Error) -> Error {
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

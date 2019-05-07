use self::Kind::*;

use http::status::StatusCode;

/// Errors that can happen while extracting data from an HTTP request.
#[derive(Debug)]
pub struct Error {
    kind: Kind,
}

#[derive(Debug)]
enum Kind {
    Missing,
    Invalid(String),
    Web(crate::Error),
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

    pub(crate) fn internal_error() -> Error {
        crate::Error::from(StatusCode::BAD_REQUEST).into()
    }
}

impl From<Error> for crate::Error {
    fn from(err: Error) -> Self {
        match err.kind {
            Missing | Invalid(_) => crate::Error::from(StatusCode::BAD_REQUEST),
            Web(err) => err,
        }
    }
}

impl From<crate::Error> for Error {
    fn from(err: crate::Error) -> Self {
        Error { kind: Web(err) }
    }
}

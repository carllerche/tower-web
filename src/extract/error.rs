use self::Kind::*;

use http::status::StatusCode;

/// Errors that can happen while extracting data from an HTTP request.
#[derive(Debug)]
pub struct Error {
    kind: Kind,
    inner: crate::Error,
}

#[derive(Debug)]
enum Kind {
    Missing,
    Invalid,
    Web,
}

impl Error {
    /// The data is missing from the HTTP request.
    pub fn missing_argument() -> Error {
        Self::missing(crate::Error::from(StatusCode::BAD_REQUEST))
    }

    /// The data is missing from the HTTP request.
    pub fn missing(inner: crate::Error) -> Error {
        Error {
            kind: Missing,
            inner,
        }
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
        let mut inner = crate::Error::from(StatusCode::BAD_REQUEST);
        inner.set_detail(&reason.to_string());
        Self::invalid(inner)
    }

    /// The data is in an invalid format and cannot be extracted.
    pub fn invalid(inner: crate::Error) -> Error {
        Error {
            kind: Invalid,
            inner,
        }
    }

    /// Returns `true` when the data is in an invalid format and cannot be
    /// extracted.
    pub fn is_invalid_argument(&self) -> bool {
        match self.kind {
            Invalid => true,
            _ => false,
        }
    }

    pub(crate) fn internal_error() -> Error {
        crate::Error::from(StatusCode::BAD_REQUEST).into()
    }
}

impl From<Error> for crate::Error {
    fn from(err: Error) -> Self {
        err.inner
    }
}

impl From<crate::Error> for Error {
    fn from(inner: crate::Error) -> Self {
        Error {
            kind: Web,
            inner,
        }
    }
}

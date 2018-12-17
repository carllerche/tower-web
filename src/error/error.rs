use self::KindPriv::*;

use std::error;
use std::fmt;
use http::status::StatusCode;

/// Errors that can happen inside Tower Web.
pub struct Error {
    kind: ErrorKind,
}

/// A list specifying the general categories of Tower Web errors.
pub struct ErrorKind {
    kind: KindPriv,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum KindPriv {
    BadRequest,
    Unauthorized,
    Fordidden,
    NotFound,
    UnprocessableEntity,
    Internal,
}

// ===== impl Error =====

impl Error {
    /// Returns the corresponding `ErrorKind` for this error.
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    /// Returns a status code for this error.
    pub fn status_code(&self) -> StatusCode {
        match self.kind.kind {
            BadRequest => StatusCode::BAD_REQUEST,
            Unauthorized => StatusCode::UNAUTHORIZED,
            Fordidden => StatusCode::FORBIDDEN,
            NotFound => StatusCode::NOT_FOUND,
            UnprocessableEntity => StatusCode::UNPROCESSABLE_ENTITY,
            Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.status_code().canonical_reason().unwrap_or("Unknown status code")
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            kind,
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Error")
            .field("kind", &self.kind)
            .finish()
    }
}

impl fmt::Display for Error {
    #[allow(deprecated)] // .cause() is deprecated on nightly
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use std::error::Error;

        if let Some(cause) = self.cause() {
            write!(fmt, "{}: {}", self.description(), cause)
        } else {
            fmt.write_str(self.description())
        }
    }
}

// ===== impl ErrorKind =====

impl ErrorKind {
    /// Returns a new `ErrorKind` value representing a 400 -- bad request error.
    pub fn bad_request() -> ErrorKind {
        ErrorKind { kind: BadRequest }
    }

    /// Returns `true` if `self` represents a 400 -- bad request error
    pub fn is_bad_request(&self) -> bool {
        self.kind == BadRequest
    }

    /// Returns a new `ErrorKind` value representing a 401 -- unauthorized error.
    pub fn unauthorized() -> ErrorKind {
        ErrorKind { kind: Unauthorized }
    }

    /// Returns a new `ErrorKind` value representing a 403 -- forbidden error.
    pub fn forbidden() -> ErrorKind {
        ErrorKind { kind: Fordidden }
    }

    /// Returns a new `ErrorKind` value representing a 404 -- not found error
    pub fn not_found() -> ErrorKind {
        ErrorKind { kind: NotFound }
    }

    /// Returns `true` if `self` represents a 404 -- not found error
    pub fn is_not_found(&self) -> bool {
        self.kind == NotFound
    }

    /// Returns a new `ErrorKind` value representing a 422 -- unprocessable entity error
    pub fn unprocessable_entity() -> ErrorKind {
        ErrorKind { kind: UnprocessableEntity }
    }

    /// Returns a new `ErrorKind` value representing 500 -- internal server
    /// error.
    pub fn internal() -> ErrorKind {
        ErrorKind { kind: Internal }
    }

    /// Returns `true` if `self` represents a 500 -- internal server error.
    pub fn is_internal(&self) -> bool {
        self.kind == Internal
    }
}

impl fmt::Debug for ErrorKind {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            BadRequest => "ErrorKind::BadRequest",
            Unauthorized => "ErrorKind::Unauthorized",
            Fordidden => "ErrorKind::Forbidden",
            NotFound => "ErrorKind::NotFound",
            UnprocessableEntity => "ErrorKind::UnprocessableEntity",
            Internal => "ErrorKind::Internal",
        }.fmt(fmt)
    }
}

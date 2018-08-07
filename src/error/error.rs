use self::KindPriv::*;

use std::error;
use std::fmt;

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
    NotFound,
    Internal,
}

// ===== impl Error =====

impl Error {
    /// Returns the corresponding `ErrorKind` for this error.
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self.kind.kind {
            BadRequest => "Bad request",
            NotFound => "Not found",
            Internal => "Internal error",
        }
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
    pub fn bad_request() -> ErrorKind {
        ErrorKind { kind: BadRequest }
    }

    pub fn is_bad_request(&self) -> bool {
        self.kind == BadRequest
    }

    pub fn not_found() -> ErrorKind {
        ErrorKind { kind: NotFound }
    }

    pub fn is_not_found(&self) -> bool {
        self.kind == NotFound
    }

    pub fn internal() -> ErrorKind {
        ErrorKind { kind: Internal }
    }

    pub fn is_internal(&self) -> bool {
        self.kind == Internal
    }
}

impl fmt::Debug for ErrorKind {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            BadRequest => "ErrorKind::BadRequest",
            NotFound => "ErrorKind::NotFound",
            Internal => "ErrorKind::Internal",
        }.fmt(fmt)
    }
}

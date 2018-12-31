use self::KindPriv::*;

use std::error;
use std::fmt;
use http::StatusCode;

/// Errors that can happen inside Tower Web.
/// The object of this type is serializable into "Problem Detail" as defined in RFC7807.
#[derive(Serialize)]
pub struct Error {
    #[serde(rename = "type")]
    kind: String,
    title: String,
    // NOTE: type of this property might be changed in the future.
    // Nevertheless, String type should be accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
    #[serde(skip)]
    status: StatusCode,

    // TODO: this property isn't used and should be removed
    #[serde(skip)]
    error_kind: ErrorKind,
}

// ===== impl Error =====

impl Error {
    // TODO: this function should return '&self.kind' property
    // and its return value should be changed to '&str'
    /// Returns the corresponding `ErrorKind` for this error.
    #[deprecated(note="return value of the function will be changed to &str")]
    pub fn kind(&self) -> &ErrorKind {
        &self.error_kind
    }

    /// Creates an error object.
    pub fn new(kind: &str, title: &str, status: StatusCode) -> Self {
        Self {
            kind: kind.to_owned(),
            title: title.to_owned(),
            detail: None,
            status,

            // TODO: this property isn't used and should be removed
            error_kind: ErrorKind::new(&status),
        }
    }

    // NOTE: type of this property might be changed in the future.
    // Nevertheless, String type should be accepted.
    /// Provides detailed information about the error.
    pub fn detail(self, detail: &str) -> Self {
        Self {
            kind: self.kind,
            title: self.title,
            detail: Some(detail.to_owned()),
            status: self.status,

            // TODO: this property isn't used and should be removed
            error_kind: self.error_kind,
        }
    }

    /// Returns a status code for this error.
    pub fn status_code(&self) -> StatusCode {
        self.status
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.status_code().canonical_reason().unwrap_or("Unknown status code")
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Error")
            .field("kind", &self.kind)
            .field("title", &self.title)
            .field("detail", &self.detail)
            .field("status", &self.status)
            .finish()
    }
}

impl fmt::Display for Error {
    #[allow(deprecated)] // .cause() is deprecated on nightly
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "[{}] {}",
            self.kind, self.title,
        )?;

        if let Some(ref detail) = self.detail {
            write!(fmt, ": {}", detail)?;
        }

        use std::error::Error;
        if let Some(ref cause) = self.cause() {
            write!(fmt, ": {}", cause)?;
        }

        Ok(())
    }
}

impl From<StatusCode> for Error {
    fn from(status: StatusCode) -> Self {
        let title = status.canonical_reason().unwrap_or("Unknown status code");
        Self {
            kind: String::from("about:blank"),
            title: title.to_owned(),
            detail: None,
            status,

            // TODO: this property isn't used and should be removed
            error_kind: ErrorKind::new(&status),
        }
    }
}

// Obsolete

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

impl From<ErrorKind> for Error {
    fn from(error_kind: ErrorKind) -> Error {
        error_kind.status_code().into()
    }
}

// ===== impl ErrorKind =====

impl ErrorKind {
    /// Returns a new `ErrorKind` value representing a 400 -- bad request error.
    #[deprecated(note="please use 'Error::from(http::StatusCode::BAD_REQUEST)' instead")]
    pub fn bad_request() -> ErrorKind {
        ErrorKind { kind: BadRequest }
    }

    /// Returns `true` if `self` represents a 400 -- bad request error
    #[deprecated(note="please use 'kind() == http::StatusCode::BAD_REQUEST' instead")]
    pub fn is_bad_request(&self) -> bool {
        self.kind == BadRequest
    }

    /// Returns a new `ErrorKind` value representing a 401 -- unauthorized error.
    #[deprecated(note="please use 'Error::from(http::StatusCode::UNAUTHORIZED)' instead")]
    pub fn unauthorized() -> ErrorKind {
        ErrorKind { kind: Unauthorized }
    }

    /// Returns a new `ErrorKind` value representing a 403 -- forbidden error.
    #[deprecated(note="please use 'Error::from(http::StatusCode::FORBIDDEN)' instead")]
    pub fn forbidden() -> ErrorKind {
        ErrorKind { kind: Fordidden }
    }

    /// Returns a new `ErrorKind` value representing a 404 -- not found error
    #[deprecated(note="please use 'Error::from(http::StatusCode::NOT_FOUND)' instead")]
    pub fn not_found() -> ErrorKind {
        ErrorKind { kind: NotFound }
    }

    /// Returns `true` if `self` represents a 404 -- not found error
    #[deprecated(note="please use 'kind() == http::StatusCode::NOT_FOUND' instead")]
    pub fn is_not_found(&self) -> bool {
        self.kind == NotFound
    }

    /// Returns a new `ErrorKind` value representing a 422 -- unprocessable entity error
    #[deprecated(note="please use 'Error::from(http::StatusCode::UNPROCESSABLE_ENTITY)' instead")]
    pub fn unprocessable_entity() -> ErrorKind {
        ErrorKind { kind: UnprocessableEntity }
    }

    /// Returns a new `ErrorKind` value representing 500 -- internal server
    /// error.
    #[deprecated(note="please use 'Error::from(http::StatusCode::INTERNAL_SERVER_ERROR)' instead")]
    pub fn internal() -> ErrorKind {
        ErrorKind { kind: Internal }
    }

    /// Returns `true` if `self` represents a 500 -- internal server error.
    #[deprecated(note="please use 'kind() == http::StatusCode::INTERNAL_SERVER_ERROR' instead")]
    pub fn is_internal(&self) -> bool {
        self.kind == Internal
    }

    fn status_code(&self) -> StatusCode {
        match self.kind {
            BadRequest => StatusCode::BAD_REQUEST,
            Unauthorized => StatusCode::UNAUTHORIZED,
            Fordidden => StatusCode::FORBIDDEN,
            NotFound => StatusCode::NOT_FOUND,
            UnprocessableEntity => StatusCode::UNPROCESSABLE_ENTITY,
            Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn new(status: &StatusCode) -> Self {
        match status {
            &StatusCode::BAD_REQUEST => ErrorKind {
                kind: BadRequest
            },
            &StatusCode::UNAUTHORIZED => ErrorKind {
                kind: Unauthorized
            },
            &StatusCode::FORBIDDEN => ErrorKind {
                kind: Fordidden
            },
            &StatusCode::NOT_FOUND => ErrorKind {
                kind: NotFound
            },
            &StatusCode::UNPROCESSABLE_ENTITY => ErrorKind {
                kind: UnprocessableEntity
            },
            _ => ErrorKind {
                kind: Internal
            },
        }
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

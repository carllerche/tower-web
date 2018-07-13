use self::Kind::*;

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
    pub fn missing_param() -> Error {
        Error { kind: Missing }
    }

    pub fn is_missing_param(&self) -> bool {
        match self.kind {
            Missing => true,
            _ => false,
        }
    }

    pub fn invalid_param<T: ToString>(reason: &T) -> Error {
        Error { kind: Invalid(reason.to_string()) }
    }

    pub fn web(err: ::Error) -> Error {
        Error { kind: Web(err) }
    }
}

impl From<Error> for ::Error {
    fn from(err: Error) -> ::Error {
        match err.kind {
            Missing | Invalid(_) => ::ErrorKind::bad_request().into(),
            Web(err) => err,
        }
    }
}

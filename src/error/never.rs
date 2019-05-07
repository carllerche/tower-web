use crate::error::Error;

use std::error;
use std::fmt;

// A crate-private type until we can use !.
//
// Being crate-private, we should be able to swap the type out in a
// backwards compatible way.
pub enum Never {}

impl From<Never> for Error {
    fn from(never: Never) -> Error {
        match never {}
    }
}

impl fmt::Debug for Never {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}

impl fmt::Display for Never {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}

impl error::Error for Never {
    fn description(&self) -> &str {
        match *self {}
    }
}

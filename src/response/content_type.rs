use bytes::Bytes;
use serde;

/// Serializes a data type according to a content type
pub struct ContentType {
    serializer: (),
}

impl ContentType {
    pub fn new<T>() -> ContentType
    where T: serde::Serializer,
          T::Ok: Into<Bytes>,
    {
        unimplemented!();
    }

    /*
    pub fn serialize<T: Serialize>(&self) -> Result<Bytes, ::Error> {
        unimplemented!();
    }
    */
}

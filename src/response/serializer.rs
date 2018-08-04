use response::ContentType;

use bytes::Bytes;
use serde::Serialize;

/// Serialize a response payload
pub trait Serializer: Clone + Send + Sync + 'static + ::util::Sealed {
    type Format: Clone + Send + Sync + 'static;

    fn lookup(&self, name: &str) -> ContentType<Self::Format>;

    fn serialize<T>(&self, value: &T, format: &Self::Format) -> Result<Bytes, ::Error>
    where
        T: Serialize;
}

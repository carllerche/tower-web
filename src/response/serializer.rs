use response::ContentType;

use bytes::Bytes;
use serde::Serialize;

/// Serialize an HTTP response body
///
/// `Serializer` values use one or more [Serde serializers][serde] to perform
/// the actual serialization.
///
/// The `Serializer` values are also responsible for mapping content-type values
/// to Serde serializers.
///
/// [serde]: https://docs.rs/serde/1.0.71/serde/trait.Serializer.html
pub trait Serializer: Clone + Send + Sync + 'static + ::util::Sealed {
    /// A token used by `Serializer` implementations to identify the specific
    /// serialization format to use when encoding a value.
    type Format: Clone + Send + Sync + 'static;

    /// Lookup a serializer and `HeaderValue` for the given `Content-Type`
    /// string.
    fn lookup(&self, name: &str) -> ContentType<Self::Format>;

    /// Serialize the value using the specified format.
    fn serialize<T>(&self, value: &T, format: &Self::Format) -> Result<Bytes, ::Error>
    where
        T: Serialize;
}

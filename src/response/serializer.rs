use crate::response::{ContentType, SerializerContext};
use crate::util::tuple::Either2;

use bytes::Bytes;
use serde::Serialize;
use void::Void;

/// Serialize an HTTP response body
///
/// `Serializer` values use one or more [Serde serializers][serde] to perform
/// the actual serialization.
///
/// The `Serializer` values are also responsible for mapping content-type values
/// to Serde serializers.
///
/// [serde]: https://docs.rs/serde/1.0.71/serde/trait.Serializer.html
pub trait Serializer: Clone + Send + Sync + 'static + crate::util::Sealed {
    /// A token used by `Serializer` implementations to identify the specific
    /// serialization format to use when encoding a value.
    type Format: Clone + Send + Sync + 'static;

    /// Lookup a serializer and `HeaderValue` for the given `Content-Type`
    /// string.
    fn lookup(&self, name: &str) -> Option<ContentType<Self::Format>>;

    /// Serialize the value using the specified format.
    fn serialize<T>(&self, value: &T, format: &Self::Format, context: &SerializerContext)
        -> Result<Bytes, crate::Error>
    where
        T: Serialize;
}

impl Serializer for () {
    type Format = Void;

    fn lookup(&self, _: &str) -> Option<ContentType<Self::Format>> {
        None
    }

    fn serialize<T>(&self, _: &T, _: &Self::Format, _: &SerializerContext)
        -> Result<Bytes, crate::Error>
    where
        T: Serialize
    {
        unreachable!();
    }
}

impl<T, U> Serializer for (T, U)
where
    T: Serializer,
    U: Serializer,
{
    type Format = Either2<T::Format, U::Format>;

    fn lookup(&self, name: &str) -> Option<ContentType<Self::Format>> {
        if let Some(content_type) = self.0.lookup(name) {
            return Some(content_type.map(Either2::A));
        }

        self.1.lookup(name)
            .map(|content_type| content_type.map(Either2::B))
    }

    fn serialize<V>(&self, value: &V, format: &Self::Format, context: &SerializerContext)
        -> Result<Bytes, crate::Error>
    where
        V: Serialize
    {
        match *format {
            Either2::A(ref format) => self.0.serialize(value, format, context),
            Either2::B(ref format) => self.1.serialize(value, format, context),
        }
    }
}

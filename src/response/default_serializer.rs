use crate::response::{ContentType, Serializer, SerializerContext};
use crate::util::Chain;
use crate::util::tuple::Either2;

use bytes::Bytes;
use http::header::HeaderValue;
use serde::Serialize;

/// Default serializer
///
/// Serializes responses into one of a number of common HTTP formats.
#[derive(Debug, Clone)]
pub struct DefaultSerializer<T = ()> {
    custom: T,
    plain: HeaderValue,
    json: HeaderValue,
}

/// Response type
#[derive(Debug, Clone)]
pub struct Format {
    kind: Kind,
}

#[derive(Debug, Clone)]
enum Kind {
    Json,
    Plain,
}

const TEXT_PLAIN: &str = "text/plain";
const APPLICATION_JSON: &str = "application/json";

impl DefaultSerializer {
    /// Return a new `DefaultSerializer` value.
    pub fn new() -> DefaultSerializer {
        DefaultSerializer {
            custom: (),
            plain: HeaderValue::from_static(TEXT_PLAIN),
            json: HeaderValue::from_static(APPLICATION_JSON),
        }
    }
}

impl<T> Serializer for DefaultSerializer<T>
where T: Serializer,
{
    type Format = Either2<T::Format, Format>;

    fn lookup(&self, name: &str) -> Option<ContentType<Self::Format>> {
        if let Some(content_type) = self.custom.lookup(name) {
            return Some(content_type.map(Either2::A));
        }

        match name {
            "json" | APPLICATION_JSON => {
                let format = Either2::B(Format::json());
                Some(ContentType::new(self.json.clone(), format))
            }
            "plain" | TEXT_PLAIN => {
                let format = Either2::B(Format::plain());
                Some(ContentType::new(self.plain.clone(), format))
            }
            _ => {
                None
            }
        }
    }

    fn serialize<V>(&self, value: &V, format: &Self::Format, ctx: &SerializerContext)
        -> Result<Bytes, crate::Error>
    where
        V: Serialize,
    {
        match format {
            Either2::A(ref format) => self.custom.serialize(value, format, ctx),
            Either2::B(ref format) => {
                match format.kind {
                    Kind::Json => {
                        let body = ::serde_json::to_vec(&value).unwrap();
                        Ok(body.into())
                    }
                    Kind::Plain => {
                        let body = ::serde_plain::to_string(&value).unwrap();
                        Ok(body.into())
                    }
                }
            }
        }
    }
}

impl<T, U> Chain<U> for DefaultSerializer<T> {
    type Output = DefaultSerializer<(T, U)>;

    fn chain(self, other: U) -> Self::Output {
        DefaultSerializer {
            custom: (self.custom, other),
            plain: self.plain,
            json: self.json,
        }
    }
}

impl<T> crate::util::Sealed for DefaultSerializer<T> {}

impl Format {
    /// Json
    fn json() -> Format {
        Format { kind: Kind::Json }
    }

    /// Plain
    fn plain() -> Format {
        Format { kind: Kind::Plain }
    }
}

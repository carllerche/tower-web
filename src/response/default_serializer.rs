use response::{ContentType, Serializer};

use bytes::Bytes;
use http::header::HeaderValue;
use serde::Serialize;

/// Default serializer
///
/// Serializes responses into one of a number of common HTTP formats.
#[derive(Debug, Clone)]
pub struct DefaultSerializer {
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

impl ::util::Sealed for DefaultSerializer {}

impl DefaultSerializer {
    pub fn new() -> DefaultSerializer {
        DefaultSerializer {
            plain: HeaderValue::from_static(TEXT_PLAIN),
            json: HeaderValue::from_static(APPLICATION_JSON),
        }
    }
}

impl Serializer for DefaultSerializer {
    type Format = Format;

    fn lookup(&self, name: &str) -> ContentType<Self::Format> {
        match name {
            "json" | APPLICATION_JSON => {
                let format = Format::json();
                ContentType::new(self.json.clone(), Some(format))
            }
            "plain" | TEXT_PLAIN => {
                let format = Format::plain();
                ContentType::new(self.plain.clone(), Some(format))
            }
            _ => {
                let header = HeaderValue::from_str(name)
                    .unwrap();

                ContentType::new(header, None)
            }
        }
    }

    fn serialize<T>(&self, value: &T, format: &Self::Format) -> Result<Bytes, ::Error>
    where
        T: Serialize,
    {
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

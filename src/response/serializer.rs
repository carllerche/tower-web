use bytes::Bytes;
use http::header::HeaderValue;
use serde::Serialize;

/// Serialize a response payload
pub trait Serializer: Clone + Send + Sync + 'static + ::util::Sealed {
    type ContentType: Clone + Send + Sync + 'static;

    fn lookup(&self, name: &str) -> Option<Self::ContentType>;

    fn content_type(&self, content_type: &Self::ContentType) -> HeaderValue;

    fn serialize<T>(&self, content_type: &Self::ContentType, value: &T) -> Result<Bytes, ::Error>
    where
        T: Serialize;
}

/// Default serialization
#[derive(Debug, Clone)]
pub struct DefaultSerializer {
    plain: HeaderValue,
    json: HeaderValue,
}

impl ::util::Sealed for DefaultSerializer {}

/// Response type
#[derive(Debug, Clone)]
pub struct ContentType {
    kind: Kind,
}

#[derive(Debug, Clone)]
enum Kind {
    Json,
    Plain,
}

const CT_TEXT_PLAIN: &str = "text/plain";
const CT_APPLICATION_JSON: &str = "application/json";

impl DefaultSerializer {
    pub fn new() -> DefaultSerializer {
        DefaultSerializer {
            plain: HeaderValue::from_static(CT_TEXT_PLAIN),
            json: HeaderValue::from_static(CT_APPLICATION_JSON),
        }
    }
}

impl Serializer for DefaultSerializer {
    type ContentType = ContentType;

    fn lookup(&self, name: &str) -> Option<Self::ContentType> {
        match name {
            "json" | CT_APPLICATION_JSON => Some(ContentType::json()),
            "plain" | CT_TEXT_PLAIN => Some(ContentType::plain()),
            _ => None,
        }
    }

    fn content_type(&self, content_type: &Self::ContentType) -> HeaderValue {
        match content_type.kind {
            Kind::Json => self.json.clone(),
            Kind::Plain => self.plain.clone(),
        }
    }

    fn serialize<T>(&self, content_type: &Self::ContentType, value: &T) -> Result<Bytes, ::Error>
    where
        T: Serialize,
    {
        match content_type.kind {
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

impl ContentType {
    /// Json
    fn json() -> ContentType {
        ContentType { kind: Kind::Json }
    }

    /// Plain
    fn plain() -> ContentType {
        ContentType { kind: Kind::Plain }
    }
}

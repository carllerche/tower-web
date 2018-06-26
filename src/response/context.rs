use response::Serializer;

use bytes::Bytes;
use http::header::HeaderValue;
use serde::Serialize;

pub struct Context<'a, S: Serializer + 'a> {
    serializer: &'a S,
    content_type: &'a S::ContentType,
}

impl<'a, S> Context<'a, S>
where S: Serializer,
{
    pub(crate) fn new(
        serializer: &'a S,
        content_type: &'a S::ContentType) -> Context<'a, S>
    {
        Context {
            serializer,
            content_type,
        }
    }

    /// Serialize a value
    pub fn serialize<T>(&self, value: &T) -> Result<Bytes, ::Error>
    where T: Serialize,
    {
        self.serializer.serialize(self.content_type, value)
    }

    pub fn content_type(&self) -> HeaderValue {
        self.serializer.content_type(self.content_type)
    }
}

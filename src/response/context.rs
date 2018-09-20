use response::{Serializer, ContentType, SerializerContext};

use bytes::Bytes;
use http;
use http::header::HeaderValue;
use serde::Serialize;

/// Context available when serializing the response.
#[derive(Debug)]
pub struct Context<'a, S: Serializer + 'a> {
    serializer: &'a S,
    default_content_type: Option<&'a ContentType<S::Format>>,
    request: &'a http::Request<()>,
    resource_mod: Option<&'a str>,
    resource_name: Option<&'a str>,
    handler_name: Option<&'a str>,
}

impl<'a, S> Context<'a, S>
where
    S: Serializer,
{
    /// Create a new response context.
    pub fn new(serializer: &'a S,
               default_content_type: Option<&'a ContentType<S::Format>>,
               request: &'a http::Request<()>,
               ) -> Context<'a, S>
    {
        Context {
            serializer,
            default_content_type,
            request,
            resource_mod: None,
            resource_name: None,
            handler_name: None,
        }
    }

    /// Returns a reference to the request
    pub fn request(&self) -> &http::Request<()> {
        self.request
    }

    #[doc(hidden)]
    pub fn set_resource_mod(&mut self, value: &'a str) {
        self.resource_mod = Some(value);
    }

    #[doc(hidden)]
    pub fn set_resource_name(&mut self, value: &'a str) {
        self.resource_name = Some(value);
    }

    #[doc(hidden)]
    pub fn set_handler_name(&mut self, value: &'a str) {
        self.handler_name = Some(value);
    }

    #[doc(hidden)]
    pub fn serializer_context(&self) -> SerializerContext {
        let mut ret = SerializerContext::new(self.request);
        ret.set_resource_mod(self.resource_mod);
        ret.set_resource_name(self.resource_name);
        ret.set_handler_name(self.handler_name);
        ret
    }

    /// Serialize a value.
    ///
    /// This uses the default content type for the action.
    ///
    /// # Panics
    ///
    /// Calling this function *requires* a default content type. If set to
    /// `None`, this function will panic.
    pub fn serialize<T>(&self, value: &T, context: &SerializerContext)
        -> Result<Bytes, ::Error>
    where
        T: Serialize,
    {
        let content_type = self.default_content_type
            .expect("no default content type associated with action");

        match content_type.format() {
            Some(format) => self.serializer.serialize(value, format, context),
            None => panic!("no serializer associated with content type `{:?}`", content_type.header()),
        }
    }

    /// Serialize a value as the specified content type.
    pub fn serialize_as<T>(&self, _value: &T, _content_type: &str)
        -> Result<Bytes, ::Error>
    where
        T: Serialize,
    {
        unimplemented!();
    }

    /// Returns a `HeaderValue` representation of the default content type.
    pub fn content_type_header(&self) -> Option<&HeaderValue> {
        self.default_content_type
            .map(|content_type| content_type.header())
    }
}

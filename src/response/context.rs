use crate::response::{Serializer, SerializerContext};

use bytes::Bytes;
use http;
use http::header::HeaderValue;
use http::status::StatusCode;
use serde::Serialize;

/// Context available when serializing the response.
#[derive(Debug)]
pub struct Context<'a, S: Serializer + 'a> {
    request: &'a http::Request<()>,
    serializer: &'a S,
    default_format: Option<&'a S::Format>,
    content_type: Option<&'a HeaderValue>,
    resource_mod: Option<&'a str>,
    resource_name: Option<&'a str>,
    handler_name: Option<&'a str>,
    template: Option<&'a str>,
}

impl<'a, S> Context<'a, S>
where
    S: Serializer,
{
    /// Create a new response context.
    pub fn new(request: &'a http::Request<()>, serializer: &'a S) -> Context<'a, S>
    {
        Context {
            request,
            serializer,
            default_format: None,
            content_type: None,
            resource_mod: None,
            resource_name: None,
            handler_name: None,
            template: None,
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

        if let Some(template) = self.template {
            ret.set_template(template);
        }

        ret
    }

    #[doc(hidden)]
    pub fn set_default_format(&mut self, value: &'a S::Format) {
        self.default_format = Some(value);
    }

    #[doc(hidden)]
    pub fn set_content_type(&mut self, value: &'a HeaderValue) {
        self.content_type = Some(value);
    }

    /// Returns the `template` value set for the response.
    pub fn template(&self) -> Option<&str> {
        self.template
    }

    #[doc(hidden)]
    pub fn set_template(&mut self, value: &'a str) {
        self.template = Some(value);
    }

    /// Serialize a value.
    ///
    /// This uses the default content type for the action.
    ///
    /// Returns an error when a default content type is not set.
    pub fn serialize<T>(&self, value: &T, context: &SerializerContext)
        -> Result<Bytes, crate::Error>
    where
        T: Serialize,
    {
        let format = match self.default_format {
            Some(format) => format,
            None => {
                warn!("no default serialization format associated with action");
                return Err(crate::Error::from(StatusCode::INTERNAL_SERVER_ERROR));
            }
        };

        self.serializer.serialize(value, format, context)
    }

    /// Serialize a value as the specified content type.
    pub fn serialize_as<T>(&self, _value: &T, _content_type: &str)
        -> Result<Bytes, crate::Error>
    where
        T: Serialize,
    {
        unimplemented!();
    }

    /// Returns a `HeaderValue` representation of the default content type.
    pub fn content_type_header(&self) -> Option<&HeaderValue> {
        self.content_type
    }
}

use http;

/// Context passed to `Serializer::serialize`
///
/// `SerializerContext` contains context obtained when deriving `Response`.
#[derive(Debug)]
pub struct SerializerContext<'a> {
    request: &'a http::Request<()>,
    resource_mod: Option<&'a str>,
    template: Option<&'a str>,
}

impl<'a> SerializerContext<'a> {
    #[doc(hidden)]
    pub fn new(request: &'a http::Request<()>) -> SerializerContext<'a> {
        SerializerContext {
            request,
            resource_mod: None,
            template: None,
        }
    }

    /// Returns a reference to the original request
    pub fn request(&self) -> &http::Request<()> {
        self.request
    }

    /// Returns the module in which the `impl_web!` was
    pub fn resource_mod(&self) -> Option<&str> {
        self.resource_mod
    }

    #[doc(hidden)]
    pub fn set_resource_mod(&mut self, value: &'a str) {
        self.resource_mod = Some(value);
    }

    /// Returns the `template` value set for the response.
    pub fn template(&self) -> Option<&str> {
        self.template
    }

    #[doc(hidden)]
    pub fn set_template(&mut self, value: &'a str) {
        self.template = Some(value);
    }
}

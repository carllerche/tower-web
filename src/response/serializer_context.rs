use http;

/// Context passed to `Serializer::serialize`
///
/// `SerializerContext` contains context obtained when deriving `Response`.
#[derive(Debug)]
pub struct SerializerContext<'a> {
    request: &'a http::Request<()>,
    resource_mod: Option<&'a str>,
    resource_name: Option<&'a str>,
    handler_name: Option<&'a str>,
    template: Option<&'a str>,
}

impl<'a> SerializerContext<'a> {
    pub(crate) fn new(request: &'a http::Request<()>) -> SerializerContext<'a> {
        SerializerContext {
            request,
            resource_mod: None,
            resource_name: None,
            handler_name: None,
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

    pub(crate) fn set_resource_mod(&mut self, value: Option<&'a str>) {
        self.resource_mod = value;
    }

    /// Returns the name of the resource handling the request.
    pub fn resource_name(&self) -> Option<&str> {
        self.resource_name
    }

    pub(crate) fn set_resource_name(&mut self, value: Option<&'a str>) {
        self.resource_name = value;
    }

    /// Returns the name of the function handling the request.
    pub fn handler_name(&self) -> Option<&str> {
        self.handler_name
    }

    pub(crate) fn set_handler_name(&mut self, value: Option<&'a str>) {
        self.handler_name = value;
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

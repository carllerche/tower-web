use http::header::HeaderValue;

/// Content type of a response
pub struct ContentType<T> {
    /// The HTTP header representing the content-type
    header: HeaderValue,

    /// Used by `Serializer` to match the content type with a specific
    /// serializer.
    format: Option<T>,
}

impl<T> ContentType<T> {
    pub(crate) fn new(header: HeaderValue, format: Option<T>) -> Self {
        ContentType {
            header,
            format,
        }
    }

    pub(crate) fn header(&self) -> &HeaderValue {
        &self.header
    }

    pub(crate) fn format(&self) -> Option<&T> {
        self.format.as_ref()
    }
}

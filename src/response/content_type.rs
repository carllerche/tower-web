use http::header::HeaderValue;

/// Content type of a response
///
/// Instances of `ContentType` are returned by [`Serializer::lookup`]. This type
/// is not intended to be used by the end user besides using it as an argument
/// to [`Context::new`].
///
/// [`Serializer::lookup`]: trait.Serializer.html#method.lookup
/// [`Context::new`]: struct.Context.html
#[derive(Debug)]
pub struct ContentType<T> {
    /// The HTTP header representing the content-type
    header: HeaderValue,

    /// Used by `Serializer` to match the content type with a specific
    /// serializer.
    format: T,
}

impl<T> ContentType<T> {
    pub(crate) fn new(header: HeaderValue, format: T) -> Self {
        ContentType {
            header,
            format,
        }
    }

    #[doc(hidden)]
    pub fn header(&self) -> &HeaderValue {
        &self.header
    }

    #[doc(hidden)]
    pub fn format(&self) -> &T {
        &self.format
    }

    pub(crate) fn map<F, U>(self, f: F) -> ContentType<U>
    where F: FnOnce(T) -> U
    {
        ContentType {
            header: self.header,
            format: f(self.format),
        }
    }
}

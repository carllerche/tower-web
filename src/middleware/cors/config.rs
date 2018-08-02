use http;

#[derive(Debug, Clone)]
pub(super) struct Config {
    /*
    /// The origins which are allowed to access this resource
    allowed_origins: AllowedOrigins,
    */

    /// The methods allowed to perform on this resource
    pub allowed_methods: Vec<http::Method>,

    /*
    /// The headers allowed to send to this resource
    allowed_headers: Vec<UniCase<String>>,

    /// The headers allowed to read from the response from this resource
    exposed_headers: Vec<UniCase<String>>,

    /// Whether to allow clients to send cookies to this resource or not
    allow_credentials: bool,

    /// Defines the max cache lifetime for operations allowed on this
    /// resource
    max_age_seconds: u32,

    /// If set, wildcard ('*') will be used as value
    /// for AccessControlAllowOrigin if possible. If not set,
    /// echoing the incoming origin will be preferred.
    /// If credentials are allowed, echoing will always be used.
    prefer_wildcard: bool,
    */
}

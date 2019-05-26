use bytes::BytesMut;
use http::header::HeaderValue;
use mime_guess;
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref BY_EXTENSION: HashMap<&'static str, HeaderValue> = {
        let mut map = HashMap::new();
        if let Some(extensions) = mime_guess::get_extensions("*", "*") {
            for extension in extensions {
                map.insert(*extension, HeaderValue::from_shared(BytesMut::from(format!("{}", mime_guess::get_mime_type(extension)).into_bytes()).freeze()).unwrap());
            }
        }
        map
    };
}

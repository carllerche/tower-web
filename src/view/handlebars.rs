use response::{Serializer, SerializerContext, ContentType};

use bytes::Bytes;
use handlebars::Handlebars as Registery;
use http::header::HeaderValue;
use serde::Serialize;

use std::env;
use std::path::Path;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Handlebars {
    registery: Arc<Registery>,
    html: HeaderValue,
}

const TEXT_HTML: &str = "text/html";

impl Handlebars {
    /// TODO: Dox
    pub fn new() -> Handlebars {
        let mut registery = Registery::new();

        if let Ok(value) = env::var("CARGO_MANIFEST_DIR") {
            let dir = Path::new(&value).join("templates");

            if dir.exists() {
                registery.register_templates_directory(".hbs", dir).unwrap();
            }
        }

        Handlebars::new_with_registery(registery)
    }

    /// TODO: Dox
    pub fn new_with_registery(registery: Registery) -> Handlebars {
        Handlebars {
            registery: Arc::new(registery),
            html: HeaderValue::from_static(TEXT_HTML),
        }
    }
}

impl Serializer for Handlebars {
    type Format = ();

    fn lookup(&self, name: &str) -> Option<ContentType<Self::Format>> {
        match name {
            "html" | TEXT_HTML => {
                Some(ContentType::new(self.html.clone(), ()))
            }
            _ => None,
        }
    }

    fn serialize<T>(&self, value: &T, _: &Self::Format, context: &SerializerContext)
        -> Result<Bytes, ::Error>
    where
        T: Serialize
    {
        if let Some(template) = context.template() {
            match self.registery.render(template, value) {
                Ok(rendered) => {
                    return Ok(rendered.into());
                }
                Err(err) => {
                    error!("error rendering template; err={:?}", err);
                    return Err(::error::ErrorKind::internal().into())
                }
            }
        }

        unimplemented!("no template specified");
    }
}

impl ::util::Sealed for Handlebars {}

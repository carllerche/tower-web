use response::{Serializer, SerializerContext, ContentType};

use bytes::Bytes;
use handlebars::Handlebars as Registery;
use http::header::HeaderValue;
use serde::Serialize;

use std::env;
use std::path::Path;
use std::sync::Arc;

/// Serialize response values using Handlebars templates
///
/// This serializer is able to render handlebar templates using structs with
/// `#[derive(Response)]` and a template name, set with the `#[web(template =
/// "<template name>")]` annotation.
#[derive(Clone, Debug)]
pub struct Handlebars {
    registery: Arc<Registery>,
    html: HeaderValue,
}

const TEXT_HTML: &str = "text/html";

impl Handlebars {
    /// Create a new handlebars serializer.
    ///
    /// The serializer renders handlebar templates using the response value to
    /// populate template variables. The response value must have
    /// `#[derive(Response)]` and a template name specified using the
    /// `#[web(template = "<template name>")]`.
    ///
    /// Templates are loaded from the `templates` directory in the crate root
    /// and have the `.hbs` file extension.
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

    /// Create a new handlebars serializer.
    ///
    /// Similar to `new`, but uses the provided registery. This allows
    /// customizing how templates are rendered.
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

        // TODO: Add conventional template lookup
        error!("no template specified; {}::{}::{}",
               context.resource_mod().unwrap_or("???"),
               context.resource_name().unwrap_or("???"),
               context.handler_name().unwrap_or("???"));
        Err(::error::ErrorKind::internal().into())
    }
}

impl ::util::Sealed for Handlebars {}

use crate::response::{Serializer, SerializerContext, ContentType};

use bytes::Bytes;
use handlebars::Handlebars as Registry;
use http::header::HeaderValue;
use http::status::StatusCode;
use serde::Serialize;

use std::env;
use std::path::{Path, MAIN_SEPARATOR};
use std::sync::Arc;

/// Serialize response values using Handlebars templates
///
/// This serializer is able to render handlebar templates using structs with
/// `#[derive(Response)]` and a template name, set with the `#[web(template =
/// "<template name>")]` annotation.
#[derive(Clone, Debug)]
pub struct Handlebars {
    registry: Arc<Registry>,
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
    /// Templates are loaded from one of the following locations, checked in order:
    ///
    /// 1. A `templates` directory under the `TOWER_WEB_TEMPLATE_DIR` environment variable
    /// 2. A `templates` directory under the crate root (`CARGO_MANIFEST_DIR` environment variable)
    /// 3. A `templates` directory under the current working directory.
    ///
    /// Templates have the `.hbs` file extension.
    ///
    /// For more control over how templates are loaded, use
    /// [`new_with_registry`](struct.Handlebars.html#method.new_with_registry).
    pub fn new() -> Handlebars {
        let mut registry = Registry::new();

        let mut registered = false;

        // 1. $TOWER_WEB_TEMPLATE_DIR/templates
        if let Ok(value) = env::var("TOWER_WEB_TEMPLATE_DIR") {
            let base_dir = Path::new(&value);

            if !base_dir.exists() {
                panic!("TOWER_WEB_TEMPLATE_DIR was set but {:?} does not exist", base_dir);
            }

            let template_dir = base_dir.join("templates");

            if !template_dir.exists() {
                panic!("TOWER_WEB_TEMPLATE_DIR was set but the template directory {:?} does not exist", template_dir);
            }
            registry.register_templates_directory(".hbs", template_dir).unwrap();
            registered = true;
        }
        if !registered {
            // 2. A 'templates' folder under the crate root
            if let Ok(value) = env::var("CARGO_MANIFEST_DIR") {
                let dir = Path::new(&value).join("templates");

                if dir.exists() {
                    registry.register_templates_directory(".hbs", dir).unwrap();
                    registered = true;
                }
            }
        }
        if !registered {
            // 3. A 'templates' folder under the current working directory
            let dir = Path::new("templates");
            if dir.exists() {
                registry.register_templates_directory(".hbs", dir).unwrap();
                registered = true;
            }
        }

        if !registered {
            let pwd = Path::new(&env::current_dir().unwrap()).join("templates");
            panic!("A templates directory was not found. Registering Handlebars failed. Checked at $TOWER_WEB_TEMPLATE_DIR{}templates, $CARGO_MANIFEST_DIR{}templates (crate root), and {:?} (under the current working directory).", MAIN_SEPARATOR, MAIN_SEPARATOR, pwd);
        }

        Handlebars::new_with_registry(registry)
    }

    /// Create a new handlebars serializer.
    ///
    /// Similar to `new`, but uses the provided registry. This allows
    /// customizing how templates are rendered.
    pub fn new_with_registry(registry: Registry) -> Handlebars {
        Handlebars {
            registry: Arc::new(registry),
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
        -> Result<Bytes, crate::Error>
    where
        T: Serialize
    {
        if let Some(template) = context.template() {
            match self.registry.render(template, value) {
                Ok(rendered) => {
                    return Ok(rendered.into());
                }
                Err(err) => {
                    error!("error rendering template; err={:?}", err);
                    return Err(crate::Error::from(StatusCode::INTERNAL_SERVER_ERROR))
                }
            }
        }

        // TODO: Use a convention to pick a template name if none is
        // specified. Probably "<module>/<handler>.hbs"
        error!("no template specified; {}::{}::{}",
               context.resource_mod().unwrap_or("???"),
               context.resource_name().unwrap_or("???"),
               context.handler_name().unwrap_or("???"));
        Err(crate::error::Error::from(StatusCode::INTERNAL_SERVER_ERROR))
    }
}

impl crate::util::Sealed for Handlebars {}

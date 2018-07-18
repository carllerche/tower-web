use syn;
use proc_macro2::TokenStream;

#[derive(Debug)]
pub(crate) struct Arg {
    pub index: usize,

    /// Argument identifier, i.e., the variable name.
    pub ident: Option<String>,

    /// The index of the path binding the identifier matches.
    pub param: Option<usize>,

    // The argument type
    pub ty: syn::Type,
}

impl Arg {
    /// Create a new, regular, argument.
    pub fn new(index: usize, ident: String, param: Option<usize>, ty: syn::Type) -> Arg {
        Arg {
            index,
            ident: Some(ident),
            param,
            ty,
        }
    }

    /// The argument is formatted in a way that cannot be interpretted.
    pub fn ty_only(index: usize, ty: syn::Type) -> Arg {
        Arg {
            index,
            ty,
            ident: None,
            param: None,
        }
    }

    /// Generate a call site for the argument
    pub fn new_callsite(&self) -> TokenStream {
        if let Some(idx) = self.param {
            quote! { __tw::codegen::CallSite::new_param(#idx) }
        } else if let Some(ref ident) = self.ident {
            match &ident[..] {
                "query_string" => quote! { __tw::codegen::CallSite::new_query_string() },
                "body" => quote! { __tw::codegen::CallSite::new_body() },
                header => {
                    let header = ::header::arg_to_header_name(header);
                    let header = header.as_str();

                    quote! { __tw::codegen::CallSite::new_header(#header) }
                }
            }
        } else {
            quote! { __tw::codegen::CallSite::new_unknown() }
        }
    }
}

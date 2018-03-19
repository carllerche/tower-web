use Route;

use {syn, quote};

#[derive(Debug)]
pub struct Service {
    pub self_ty: Box<syn::Type>,
    pub routes: Vec<Route>,
}

impl Service {
    pub fn new(self_ty: Box<syn::Type>) -> Service {
        Service {
            self_ty,
            routes: vec![],
        }
    }

    pub fn destination_ty(&self) -> quote::Tokens {
        match self.routes.len() {
            0 | 1 => quote! { () },
            2 => quote! { ::tower_web::resource::tuple::Either2 },
            _ => unimplemented!(),
        }
    }
}

use Route;

use syn;

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
}

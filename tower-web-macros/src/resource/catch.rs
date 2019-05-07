use crate::resource::{Attributes, Signature};

use proc_macro2::TokenStream;

#[derive(Debug)]
pub(crate) struct Catch {
    index: usize,

    sig: Signature,

    attributes: Attributes,
}

impl Catch {
    pub fn new(index: usize, sig: Signature, attributes: Attributes) -> Catch {
        // TODO: Handle args
        assert!(sig.args().is_empty(), "catch arguments unimplemented");

        Catch {
            index,
            sig,
            attributes,
        }
    }

    /// The response future type
    pub fn future_ty(&self) -> TokenStream {
        self.sig.future_ty()
    }

    pub fn dispatch(&self) -> TokenStream {
        let args = self.sig.args().iter().map(|_arg| {
            panic!("unimplemented: catch handlers cannot take arguments");
        });

        self.sig.dispatch(
            quote!(self.inner),
            args)
    }
}

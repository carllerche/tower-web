use resource::{Attributes, Signature};

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
}

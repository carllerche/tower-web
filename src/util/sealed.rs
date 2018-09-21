/// Private trait to this crate to prevent traits from being implemented in
/// downstream crates.
pub trait Sealed {}

impl Sealed for () {}
impl<T, U> Sealed for (T, U) {}

use super::Filter;

use tower_service::Service;

use std::marker::PhantomData;

/// Two filters chained together.
///
/// This type is produced by `Filter::chain`.
pub struct Chain<S, Inner, Outer>
{
    inner: Inner,
    outer: Outer,
    _p: PhantomData<S>,
}

impl<S, Inner, Outer> Chain<S, Inner, Outer> {
    /// Create a new `Chain`.
    pub(crate) fn new(inner: Inner, outer: Outer) -> Self {
        Chain {
            inner,
            outer,
            _p: PhantomData,
        }
    }
}

impl<S, Inner, Outer> Filter<S> for Chain<S, Inner, Outer>
where S: Service,
      Inner: Filter<S>,
      Outer: Filter<Inner::Service>,
{
    type Service = Outer::Service;

    fn wrap(&self, service: S) -> Self::Service {
        self.outer.wrap(
            self.inner.wrap(service))
    }
}

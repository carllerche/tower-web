use super::Middleware;

use tower_service::Service;

use std::marker::PhantomData;

/// Two middlewares chained together.
///
/// This type is produced by `Middleware::chain`.
pub struct Chain<S, Inner, Outer>
{
    inner: Inner,
    outer: Outer,
    _p: PhantomData<S>,
}

impl<S, Inner, Outer> Chain<S, Inner, Outer> {
    /// Create a new `Chain`.
    pub fn new(inner: Inner, outer: Outer) -> Self {
        Chain {
            inner,
            outer,
            _p: PhantomData,
        }
    }

    pub fn get_inner(&self) -> &Inner {
        &self.inner
    }

    pub fn get_outer(&self) -> &Outer {
        &self.outer
    }
}

impl<S, Inner, Outer> Middleware<S> for Chain<S, Inner, Outer>
where S: Service,
      Inner: Middleware<S>,
      Outer: Middleware<Inner::Service>,
{
    type Request = Outer::Request;
    type Response = Outer::Response;
    type Error = Outer::Error;
    type Service = Outer::Service;

    fn wrap(&self, service: S) -> Self::Service {
        self.outer.wrap(
            self.inner.wrap(service))
    }
}

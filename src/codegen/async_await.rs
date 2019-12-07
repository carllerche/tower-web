use futures::Future;

use std::future::Future as StdFuture;

/// Converts a `std::future::Future` to a boxed stable future.
///
/// This bridges async/await with stable futures.
pub fn async_to_box_future_send<T>(
    future: T,
) -> Box<dyn Future<Item = T::Output, Error = crate::Error> + Send>
where
    T: StdFuture + Send + 'static,
{
    use futures03::future::{TryFutureExt, FutureExt};
    Box::new(future.unit_error().map_err(|_| unreachable!()).boxed().compat())
}

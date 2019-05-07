use futures::Future;

use std::future::Future as StdFuture;

/// Converts a `std::future::Future` to a boxed stable future.
///
/// This bridges async/await with stable futures.
pub fn async_to_box_future_send<T>(
    future: T,
) -> Box<Future<Item = T::Output, Error = crate::Error> + Send>
where
    T: StdFuture + Send + 'static,
{
    use tokio_async_await::compat::backward;

    let future = backward::Compat::new(map_ok(future));
    Box::new(future)
}

async fn map_ok<T, E>(future: T) -> Result<T::Output, E>
where
    T: StdFuture,
{
    Ok(await!(future))
}

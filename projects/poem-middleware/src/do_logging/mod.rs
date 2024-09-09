use poem::{Body, Endpoint, IntoResponse, Middleware, Request, Response};


/// Middleware for [`tracing`](https://crates.io/crates/tracing).
#[derive(Default)]
pub struct RequestTracing {}

impl<E: Endpoint> Middleware<E> for RequestTracing {
    type Output = TracingEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        TracingEndpoint { inner: ep }
    }
}

/// Endpoint for the `Tracing` middleware.
pub struct TracingEndpoint<E> {
    inner: E,
}

impl<E: Endpoint> Endpoint for TracingEndpoint<E> {
    type Output = Response;
    async fn call(&self, input: Request) -> poem::Result<Self::Output> {
        tracing::trace!("{:?}", input);
        let (head, body) = input.into_parts();
        let next = match body.into_string().await {
            Ok(o) => {
                tracing::trace!("{}", o);
                Request::from_parts(head, Body::from(o))
            }
            Err(_) => {
                Request::from_parts(head, Body::from(()))
            }
        };
        Ok(self.inner.call(next).await?.into_response())
    }
}

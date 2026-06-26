use axum::{
    extract::Request,
    http::{HeaderName, HeaderValue},
    middleware::Next,
    response::Response,
};
use tracing::Instrument;
use uuid::Uuid;

use crate::presentation::middleware::request_context::RequestContext;

pub async fn request_id(mut request: Request, next: Next) -> Response {
    let request_id = Uuid::now_v7();

    let ctx = RequestContext { request_id };

    request.extensions_mut().insert(ctx.clone());

    let span = tracing::info_span!("http-request", request_id = %request_id, method = %request.method(), path = %request.uri().path());

    let mut response = next.run(request).instrument(span).await;

    response.headers_mut().insert(
        HeaderName::from_static("x-request-id"),
        HeaderValue::from_str(&ctx.request_id.to_string()).unwrap(),
    );

    response
}

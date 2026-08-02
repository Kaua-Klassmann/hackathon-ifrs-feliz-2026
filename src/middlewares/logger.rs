use axum::{body::Body, extract::MatchedPath, http::Request, middleware::Next, response::Response};
use tokio::time::Instant;

pub async fn logger(req: Request<Body>, next: Next) -> Response {
    let method = req.method().to_string();
    let route = req
        .extensions()
        .get::<MatchedPath>()
        .map(|path| path.as_str().to_string())
        .unwrap_or_else(|| req.uri().path().to_string());

    let start = Instant::now();

    let response = next.run(req).await;

    tracing::info!(
        target: "api",
        method = %method,
        route = %route,
        status = response.status().as_u16(),
        time_ms = start.elapsed().as_millis(),
    );

    response
}

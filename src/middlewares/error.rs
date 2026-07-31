use std::any::Any;

use axum::{
    Json,
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use serde_json::json;
use tower_http::catch_panic::CatchPanicLayer;

pub fn get_catch() -> CatchPanicLayer<impl Fn(Box<dyn Any + Send>) -> Response<Body> + Clone> {
    CatchPanicLayer::custom(|err| {
        println!("{:?}", err);

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Internal Server Error"})),
        )
            .into_response()
    })
}

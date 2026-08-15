use std::sync::Arc;

use crate::{jwt::JwtClaims, services::remedies::RemediesService};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

pub struct RemediesController;

impl RemediesController {
    pub async fn list(
        State(service): State<Arc<RemediesService>>,
        _: JwtClaims,
    ) -> impl IntoResponse {
        let result = service.list().await;

        match result {
            Ok(remedies) => (StatusCode::OK, Json(remedies)).into_response(),
            Err(err) => err.into_response(),
        }
    }
}

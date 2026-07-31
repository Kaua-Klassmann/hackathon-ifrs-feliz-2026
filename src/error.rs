use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use validator::ValidationErrors;

#[cfg_attr(test, derive(Debug))]
pub enum DomainError {
    InternalServerError(String),
    ProductNotFound,
    UnprocessableEntity(ValidationErrors),
}

impl IntoResponse for DomainError {
    fn into_response(self) -> axum::response::Response {
        let response = match self {
            DomainError::InternalServerError(err) => {
                println!("\nError: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Erro interno do servidor"})),
                )
            }
            DomainError::ProductNotFound => (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Produto não encontrado"})),
            ),
            DomainError::UnprocessableEntity(err) => {
                (StatusCode::UNPROCESSABLE_ENTITY, Json(json!(err)))
            }
        };

        response.into_response()
    }
}

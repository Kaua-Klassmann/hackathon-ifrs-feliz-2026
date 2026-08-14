use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use validator::ValidationErrors;

#[cfg_attr(test, derive(Debug))]
pub enum DomainError {
    InternalServerError(String),
    UnprocessableEntity(ValidationErrors),
    UserAlreadyExists,
    UserInvalidCredentials,
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
            DomainError::UserAlreadyExists => (
                StatusCode::CONFLICT,
                Json(json!({"error": "Usuário já cadastrado"})),
            ),
            DomainError::UserInvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Credenciais inválidas"})),
            ),
            DomainError::UnprocessableEntity(err) => {
                (StatusCode::UNPROCESSABLE_ENTITY, Json(json!(err)))
            }
        };

        response.into_response()
    }
}

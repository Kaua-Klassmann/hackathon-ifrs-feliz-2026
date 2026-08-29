use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use validator::ValidationErrors;

#[cfg_attr(test, derive(Debug))]
pub enum DomainError {
    MetricsNotFound,
    InternalServerError(String),
    PatientNotFound,
    RemedyNotFound,
    UnprocessableEntity(ValidationErrors),
    UserAlreadyExists,
    UserInvalidCredentials,
    UserNotAuthorized,
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
            DomainError::MetricsNotFound => (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Métricas não encontradas"})),
            ),
            DomainError::PatientNotFound => (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Paciente não encontrado"})),
            ),
            DomainError::RemedyNotFound => (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Remédio não encontrado"})),
            ),
            DomainError::UserAlreadyExists => (
                StatusCode::CONFLICT,
                Json(json!({"error": "Usuário já cadastrado"})),
            ),
            DomainError::UserInvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Credenciais inválidas"})),
            ),
            DomainError::UserNotAuthorized => (
                StatusCode::FORBIDDEN,
                Json(json!({"error": "Usuário não autorizado"})),
            ),
            DomainError::UnprocessableEntity(err) => {
                (StatusCode::UNPROCESSABLE_ENTITY, Json(json!(err)))
            }
        };

        response.into_response()
    }
}

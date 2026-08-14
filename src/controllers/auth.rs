use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use validator::{Validate, ValidationError};

use crate::{
    error::DomainError, repositories::users::RegisterUserPayload, services::auth::AuthService,
};

fn validate_password(password: &str) -> Result<(), ValidationError> {
    let mut chars = password.chars();

    if !chars.clone().any(|c| c.is_ascii_lowercase())
        || !chars.clone().any(|c| c.is_ascii_uppercase())
        || !chars.clone().any(|c| c.is_ascii_digit())
        || !chars.any(|c| !c.is_ascii_alphanumeric())
    {
        return Err(ValidationError::new("password_complexity"));
    }

    Ok(())
}

#[derive(Deserialize, Validate)]
pub struct LoginPayload {
    #[validate(email(message = "Email inválido"))]
    pub email: String,
    #[validate(length(min = 8, message = "Senha deve ter no mínimo 8 caracteres"))]
    #[validate(custom(
        function = "validate_password",
        message = "Senha deve conter pelo menos uma letra maiúscula, uma letra minúscula, um número e um caractere especial",
    ))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct RegisterPayload {
    #[validate(email(message = "Email inválido"))]
    pub email: String,
    #[validate(length(min = 3, message = "Nome não pode contar menos de 3 caracteres"))]
    pub name: String,
    #[validate(length(min = 8, message = "Senha deve ter no mínimo 8 caracteres"))]
    #[validate(custom(
        function = "validate_password",
        message = "Senha deve conter pelo menos uma letra maiúscula, uma letra minúscula, um número e um caractere especial",
    ))]
    pub password: String,
}

pub struct AuthController;

impl AuthController {
    pub async fn login(
        State(service): State<Arc<AuthService>>,
        Json(payload): Json<LoginPayload>,
    ) -> impl IntoResponse {
        if let Err(errors) = payload.validate() {
            return DomainError::UnprocessableEntity(errors).into_response();
        }

        let result = service.login(&payload.email, &payload.password).await;

        match result {
            Ok(res) => (StatusCode::OK, Json(res)).into_response(),
            Err(err) => err.into_response(),
        }
    }

    pub async fn register(
        State(service): State<Arc<AuthService>>,
        Json(payload): Json<RegisterPayload>,
    ) -> impl IntoResponse {
        if let Err(errors) = payload.validate() {
            return DomainError::UnprocessableEntity(errors).into_response();
        }

        let result = service
            .register(RegisterUserPayload {
                email: payload.email,
                name: payload.name,
                password: payload.password,
            })
            .await;

        match result {
            Ok(res) => (StatusCode::OK, Json(res)).into_response(),
            Err(err) => err.into_response(),
        }
    }
}

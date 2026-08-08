use std::sync::{Arc, LazyLock};

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use regex::Regex;
use serde::Deserialize;
use validator::Validate;

use crate::{
    error::DomainError, repositories::users::RegisterUserPayload, services::auth::AuthService,
};

static PASSWORD_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[^a-zA-Z\d]).+$").unwrap());

#[derive(Deserialize, Validate)]
pub struct LoginPayload {
    #[validate(email(message = "Email inválido"))]
    pub email: String,
    #[validate(length(min = 8, message = "Senha deve ter no mínimo 8 caracteres"))]
    #[validate(regex(
        path = *PASSWORD_REGEX,
        message = "Senha deve conter pelo menos uma letra maiúscula, uma letra minúscula, um número e um caractere especial",
    ))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct RegisterPayload {
    #[validate(email(message = "Email inválido"))]
    pub email: String,
    #[validate(length(min = 8, message = "Senha deve ter no mínimo 8 caracteres"))]
    #[validate(regex(
        path = *PASSWORD_REGEX,
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
                password: payload.password,
            })
            .await;

        match result {
            Ok(_) => (StatusCode::OK, "").into_response(),
            Err(err) => err.into_response(),
        }
    }
}

use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::prelude::Uuid;
use serde::Deserialize;
use validator::Validate;

use crate::{error::DomainError, repositories, services::product::ProductService};

#[derive(Deserialize, Validate)]
pub struct RegisterProductValidation {
    #[serde(rename = "idUser")]
    id_user: Uuid,
    #[validate(length(min = 1, message = "Nome do produto não pode ser vazio"))]
    name: String,
}

pub struct ProductController;

impl ProductController {
    pub async fn list(State(service): State<Arc<ProductService>>) -> impl IntoResponse {
        let response = service.list().await;

        match response {
            Ok(res) => (StatusCode::OK, Json(res)).into_response(),
            Err(err) => err.into_response(),
        }
    }

    pub async fn get_by_id(
        State(service): State<Arc<ProductService>>,
        Path(id): Path<Uuid>,
    ) -> impl IntoResponse {
        let response = service.get_by_id(id).await;

        match response {
            Ok(res) => (StatusCode::OK, Json(res)).into_response(),
            Err(err) => err.into_response(),
        }
    }

    pub async fn register(
        State(service): State<Arc<ProductService>>,
        Json(payload): Json<RegisterProductValidation>,
    ) -> impl IntoResponse {
        if let Err(errors) = payload.validate() {
            return DomainError::UnprocessableEntity(errors).into_response();
        }

        let response = service
            .register(repositories::product::RegisterProductPayload {
                id_user: payload.id_user,
                name: payload.name,
            })
            .await;

        match response {
            Ok(res) => (StatusCode::CREATED, Json(res)).into_response(),
            Err(err) => err.into_response(),
        }
    }
}

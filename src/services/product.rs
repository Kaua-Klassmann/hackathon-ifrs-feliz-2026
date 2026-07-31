use sea_orm::prelude::Uuid;

use crate::{
    error::DomainError,
    repositories::product::{
        GetByIdProductResponse, ListProductsResponse, ProductRepository, ProductRepositoryTrait,
        RegisterProductPayload,
    },
};

pub struct ProductService<PR: ProductRepositoryTrait = ProductRepository> {
    pub product_repository: PR,
}

impl ProductService<ProductRepository> {
    pub fn new() -> Self {
        Self {
            product_repository: ProductRepository::new(),
        }
    }
}

impl<PR: ProductRepositoryTrait> ProductService<PR> {
    pub async fn list(&self) -> Result<Vec<ListProductsResponse>, DomainError> {
        self.product_repository
            .list()
            .await
            .map_err(|err| DomainError::InternalServerError(err.to_string()))
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<GetByIdProductResponse, DomainError> {
        let product = self
            .product_repository
            .get_by_id(id)
            .await
            .map_err(|err| DomainError::InternalServerError(err.to_string()))?;

        product.ok_or(DomainError::ProductNotFound)
    }

    pub async fn register(&self, payload: RegisterProductPayload) -> Result<Uuid, DomainError> {
        self.product_repository
            .register(payload)
            .await
            .map_err(|err| DomainError::InternalServerError(err.to_string()))
    }
}

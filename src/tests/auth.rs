use argon2::{
    PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};

use crate::{repositories::users::MockUsersRepositoryTrait, services::auth::AuthService};

fn factory() -> AuthService<MockUsersRepositoryTrait> {
    AuthService {
        argon2: argon2::Argon2::default(),
        user_repository: MockUsersRepositoryTrait::new(),
    }
}

fn generate_hashed_password(password: &str) -> String {
    let argon2 = argon2::Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
        .unwrap()
        .to_string()
}

mod login {
    use sea_orm::sqlx::types::Uuid;

    use crate::{
        error::DomainError, repositories::users::GetToLoginUserResponse,
        tests::auth::generate_hashed_password,
    };

    #[tokio::test]
    async fn success() {
        let mut service = super::factory();

        service
            .user_repository
            .expect_get_to_login()
            .returning(|_| {
                Ok(Some(GetToLoginUserResponse {
                    id: Uuid::default(),
                    password: generate_hashed_password("correct_password"),
                }))
            });

        let result = service.login("test@gmail.com", "correct_password").await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn error_user_not_found() {
        let mut service = super::factory();

        service
            .user_repository
            .expect_get_to_login()
            .returning(|_| Ok(None));

        let result = service.login("test@gmail.com", "password").await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DomainError::UserInvalidCredentials
        ));
    }

    #[tokio::test]
    async fn error_invalid_password() {
        let mut service = super::factory();

        service
            .user_repository
            .expect_get_to_login()
            .returning(|_| {
                Ok(Some(GetToLoginUserResponse {
                    id: Uuid::default(),
                    password: generate_hashed_password("wrong_password"),
                }))
            });

        let result = service.login("test@gmail.com", "password").await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DomainError::UserInvalidCredentials
        ));
    }
}

mod register {
    use crate::{error::DomainError, repositories::users::RegisterUserPayload};

    #[tokio::test]
    async fn success() {
        let mut service = super::factory();

        service
            .user_repository
            .expect_exists_by_email()
            .returning(|_| Ok(false));

        service
            .user_repository
            .expect_register()
            .returning(|_| Ok(()));

        let result = service
            .register(RegisterUserPayload {
                email: "test@gmail.com".to_string(),
                password: "password".to_string(),
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn error_user_already_exists() {
        let mut service = super::factory();

        service
            .user_repository
            .expect_exists_by_email()
            .returning(|_| Ok(true));

        let result = service
            .register(RegisterUserPayload {
                email: "test@gmail.com".to_string(),
                password: "password".to_string(),
            })
            .await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DomainError::UserAlreadyExists
        ));
    }
}

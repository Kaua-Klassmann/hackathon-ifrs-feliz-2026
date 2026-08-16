use crate::{
    error::DomainError,
    jwt::JwtClaims,
    repositories::users::{RegisterUserPayload, UsersRepository, UsersRepositoryTrait},
};
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct LoginResponse {
    email: String,
    name: String,
    token: String,
}

pub struct AuthService<UR: UsersRepositoryTrait = UsersRepository> {
    pub argon2: Argon2<'static>,
    pub user_repository: UR,
}

impl AuthService<UsersRepository> {
    pub fn new() -> Self {
        Self {
            argon2: Argon2::default(),
            user_repository: UsersRepository::new(),
        }
    }
}

impl<UR: UsersRepositoryTrait> AuthService<UR> {
    pub async fn login(&self, email: &str, password: &str) -> Result<LoginResponse, DomainError> {
        let user_option = self
            .user_repository
            .get_to_login(email)
            .await
            .map_err(|err| DomainError::InternalServerError(err.to_string()))?;

        let password_verified = self.argon2.verify_password(
            password.as_bytes(),
            &PasswordHash::new(
                user_option
                    .as_ref()
                    .map(|u| &u.password)
                    .unwrap_or(&"$argon2id$v=19$m=19456,t=2,p=1$ZHVtbXktc2FsdC0xMjM0NTY$y7J8J6XKQK3m6YQx5XwJvQ8vQmYQh6jXQ5Q5XQ5Q5Q5Q".to_string()),
            )
            .map_err(|err| DomainError::InternalServerError(err.to_string()))?,
        );

        if user_option.is_none() || password_verified.is_err() {
            return Err(DomainError::UserInvalidCredentials);
        }

        let user = user_option.unwrap();
        let token = JwtClaims::new(user.id).gen_token();

        Ok(LoginResponse {
            name: user.name,
            token,
            email: email.to_string(),
        })
    }

    pub async fn register(
        &self,
        payload: RegisterUserPayload,
    ) -> Result<LoginResponse, DomainError> {
        let exists = self
            .user_repository
            .exists_by_email(&payload.email)
            .await
            .map_err(|err| DomainError::InternalServerError(err.to_string()))?;

        if exists {
            return Err(DomainError::UserAlreadyExists);
        }

        let password = self
            .argon2
            .hash_password(
                payload.password.as_bytes(),
                &SaltString::generate(&mut OsRng),
            )
            .map_err(|err| DomainError::InternalServerError(err.to_string()))?
            .to_string();

        let user = self
            .user_repository
            .register(RegisterUserPayload {
                name: payload.name.clone(),
                email: payload.email.clone(),
                password,
            })
            .await
            .map_err(|err| DomainError::InternalServerError(err.to_string()))?;

        let token = JwtClaims::new(user).gen_token();

        Ok(LoginResponse {
            token,
            email: payload.email,
            name: payload.name,
        })
    }
}

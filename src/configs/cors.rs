use std::{env, sync::OnceLock};

use axum::http::{HeaderName, HeaderValue, Method, header};

#[derive(Clone)]
pub struct CorsConfig {
    pub origins: Vec<HeaderValue>,
    pub methods: Vec<Method>,
    pub headers: Vec<HeaderName>,
}

static CORS_CONFIG: OnceLock<CorsConfig> = OnceLock::new();

pub fn get_cors_config() -> &'static CorsConfig {
    CORS_CONFIG.get_or_init(|| {
        let origins: Vec<HeaderValue> = env::var("CORS_ORIGIN")
            .expect("CORS_ORIGIN not found at .env file")
            .split(',')
            .filter_map(|origin| origin.trim().parse::<HeaderValue>().ok())
            .collect();

        let methods = vec![Method::GET, Method::POST, Method::PUT, Method::DELETE];

        let headers = vec![header::CONTENT_TYPE];

        CorsConfig {
            origins,
            methods,
            headers,
        }
    })
}

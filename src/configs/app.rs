use std::{env, sync::OnceLock};

pub struct AppConfig {
    pub port: u16,
    pub payload_max_size: usize,
}

static APP_CONFIG: OnceLock<AppConfig> = OnceLock::new();

pub fn get_app_config() -> &'static AppConfig {
    APP_CONFIG.get_or_init(|| {
        let port = env::var("APP_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000);
        let payload_max_size = env::var("APP_PAYLOAD_MAX_SIZE_IN_MB")
            .expect("APP_PAYLOAD_MAX_SIZE_IN_MB not found on env")
            .parse()
            .expect("APP_PAYLOAD_MAX_SIZE_IN_MB must be a number");

        AppConfig {
            port,
            payload_max_size,
        }
    })
}

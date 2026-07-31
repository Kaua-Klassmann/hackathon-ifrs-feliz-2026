use crate::configs::app::get_app_config;
use axum::extract::DefaultBodyLimit;

pub fn get_body_limit() -> DefaultBodyLimit {
    DefaultBodyLimit::max(get_app_config().payload_max_size * 1024 * 1024)
}

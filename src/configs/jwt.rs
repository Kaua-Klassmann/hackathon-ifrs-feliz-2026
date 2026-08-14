use std::{env, sync::OnceLock};

#[derive(Clone)]
pub struct JwtOpts {
    pub secret: String,
    pub expiration: usize,
}

static JWT_OPTS: OnceLock<JwtOpts> = OnceLock::new();

pub fn get_jwt_opts() -> &'static JwtOpts {
    JWT_OPTS.get_or_init(|| {
        let secret = env::var("JWT_SECRET").unwrap_or("hfsjhfsjkvbhdfjsghsjdfkbn".to_string());
        let expiration = env::var("JWT_EXPIRATION")
            .unwrap_or("3600".to_string())
            .parse::<usize>()
            .expect("JWT_EXPIRATION not found at .env file");

        JwtOpts { secret, expiration }
    })
}

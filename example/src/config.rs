use once_cell::sync::Lazy;
use std::env;

#[derive(Clone, Eq, PartialEq)]
pub enum RustEnv {
    Development,
    Production,
}

impl RustEnv {
    pub fn new(value: String) -> Self {
        match value.as_str() {
            "development" => Self::Development,
            "production" => Self::Production,
            value => panic!("RUST_ENV {value} invalid"),
        }
    }
}

pub struct Config {
    pub rust_env: RustEnv,
    pub port: u16,
    pub database_url: String,
}

impl Config {
    pub fn new() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        Self {
            rust_env: RustEnv::new(env::var("RUST_ENV").expect("RUST_ENV must be set")),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse::<u16>()
                .expect("PORT is not valid"),
            database_url,
        }
    }

    pub fn is_dev(&self) -> bool {
        self.rust_env == RustEnv::Development
    }

    pub fn is_production(&self) -> bool {
        self.rust_env == RustEnv::Production
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(Config::new);

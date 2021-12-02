use clickhouse_rs::Pool;
use config::ConfigError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

impl DatabaseConfig {
    pub fn create_pool(&self) -> Pool {
        let pool = Pool::new(self.url.clone());
        pool
    }
}

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    lvl: String,
}

impl LogConfig {
    pub fn init(&self) {
        tracing_subscriber::fmt()
            // .with_span_events(
            //     tracing_subscriber::fmt::format::FmtSpan::ENTER
            //         | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
            // )
            .with_env_filter(self.lvl.clone())
            .init();
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub db: DatabaseConfig,
    pub log: LogConfig,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}

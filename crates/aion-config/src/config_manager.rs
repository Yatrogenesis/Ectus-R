use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
}

pub struct ConfigManager {}

impl ConfigManager {
    pub fn load() -> Result<Config> {
        let config = Config {
            database_url: "postgresql://localhost/aion_r".to_string(),
            redis_url: "redis://localhost:6379".to_string(),
            jwt_secret: "your-secret-key".to_string(),
        };
        Ok(config)
    }
}
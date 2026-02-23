use serde::{Deserialize, Serialize};

/// Application configuration loaded from config/app.toml + env vars
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub logging: LoggingConfig,
    pub environment: EnvironmentConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoggingConfig {
    pub level: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnvironmentConfig {
    pub environment: String,
    pub log_level: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub url: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 3000,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
            },
            environment: EnvironmentConfig {
                environment: "development".to_string(),
                log_level: "info".to_string(),
            },
            database: DatabaseConfig {
                url: "sqlite://data.db?mode=rwc".to_string(),
            },
        }
    }
}

impl AppConfig {
    /// Load configuration from config/app.toml and environment variables
    pub fn load() -> Result<Self, config::ConfigError> {
        let builder = config::Config::builder()
            .add_source(config::File::with_name("config/app").required(false))
            .add_source(config::Environment::with_prefix("APP").separator("__"));

        let config = builder.build()?;
        config.try_deserialize()
    }

    pub fn is_development(&self) -> bool {
        self.environment.environment == "development"
    }

    pub fn is_production(&self) -> bool {
        self.environment.environment == "production"
    }
}

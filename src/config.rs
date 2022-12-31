use serde_derive::Deserialize;
use std::fs::read_to_string;
use std::error::Error;

fn default_address() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    8080
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_redis() -> RedisConfig {
    RedisConfig { address: "127.0.0.1".to_string(), port: 6379 }
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    // Go links server address
    #[serde(default = "default_address")]
    pub address: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    // Redis config
    #[serde(default = "default_redis")]
    pub redis: RedisConfig,
}

#[derive(Debug, Deserialize)]
pub struct RedisConfig {
    pub address: String,
    pub port: u16
}

pub fn parse_config(path: &str) -> Result<ServerConfig, Box<dyn Error>> {
    let raw_config = read_to_string(path);

    let config: ServerConfig = match raw_config {
        Ok(raw) => match toml::from_str(&raw) {
	    Ok(cfg) => cfg,
	    Err(e) => panic!("Error parsing config: {}", e),
	},
        Err(e) => panic!("Error reading file: {}", e),
    };

    Ok(config)
}

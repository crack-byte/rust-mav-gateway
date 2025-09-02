use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize,Serialize, Clone)]
pub struct AppConfig {
    pub mavlink: MavlinkConfig,
    pub api: ApiConfig,
    pub proxy: ProxyConfig,
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct MavlinkConfig {
    pub source: String,
    pub targets: Vec<Target>,
}
#[derive(Debug, Deserialize,Serialize, Clone)]
pub struct ProxyConfig {
    pub listen_port: Option<u16>,
}
#[derive(Debug, Deserialize,Serialize, Clone)]
pub struct Target {
    pub ip: String,
    pub port: u16,
}

#[derive(Debug, Deserialize,Serialize, Clone)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
}

pub fn load(path: &str) -> Result<AppConfig, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let cfg: AppConfig = serde_yaml::from_str(&content)?;
    Ok(cfg)
}

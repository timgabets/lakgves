use crate::errors::AppError;

use serde_derive::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use toml::value::Table;

#[derive(Deserialize, Debug)]
struct Listener {
    #[serde(rename(deserialize = "listen"))]
    host: String,
    n_workers: u32,
}

#[derive(Deserialize, Debug)]
enum ChannelFormat {
    DhiXml,
}

#[derive(Deserialize, Debug)]
struct Channel {
    //#[serde(rename(deserialize = "type"))]
    format: ChannelFormat,
    host: String,
    port: u16,
    keep_alive: u32,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    listener: Listener,
    channels: Table,
}

impl AppConfig {
    pub fn new(conf_file: &str) -> Result<Self, AppError> {
        let mut buf = Vec::new();
        let mut fd = File::open(conf_file)?;
        fd.read_to_end(&mut buf)?;
        let app_cfg: AppConfig = toml::from_slice(&buf)?;
        Ok(app_cfg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use toml::value::Value;

    #[test]
    fn config_parse_valid_config() {
        let app_cfg = AppConfig::new("tests/data/valid.toml").unwrap();

        assert_eq!(app_cfg.listener.host, "localhost:8080");
        assert_eq!(app_cfg.listener.n_workers, 4);

        assert!(app_cfg.channels["dhi"].is_table());
        assert_eq!(
            app_cfg.channels["dhi"]["host"],
            Value::from("host.bank.com")
        );
        assert_eq!(app_cfg.channels["dhi"]["keep_alive"], Value::from(75));
        assert_eq!(app_cfg.channels["dhi"]["port"], Value::from(10309));

        assert!(app_cfg.channels["vsms"].is_table());
        assert_eq!(
            app_cfg.channels["vsms"]["host"],
            Value::from("visa.bank.com")
        );
        assert_eq!(app_cfg.channels["vsms"]["keep_alive"], Value::from(75));
        assert_eq!(app_cfg.channels["vsms"]["port"], Value::from(10303));
    }
}

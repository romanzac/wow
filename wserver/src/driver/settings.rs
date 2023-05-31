use config::{Config, ConfigError, File};
use env_logger::Builder;
use log::LevelFilter;
use std::str::FromStr;

#[derive(Debug, Default, serde_derive::Deserialize, PartialEq, Eq)]
pub struct SeverConfig {
    pub listen: String,
    pub port: String,
    pub quotes_file: String,
    pub log_level: String,
}

pub fn parse_config(cfg_file: &str) -> Result<SeverConfig, ConfigError> {
    let config = Config::builder()
        .add_source(File::with_name(cfg_file))
        .add_source(config::Environment::with_prefix("WOW_SERVER").try_parsing(true))
        .build()
        .unwrap();

    config.try_deserialize::<SeverConfig>()
}

pub fn init_logger(log_level: &str) {
    let ll_filter = LevelFilter::from_str(log_level).unwrap();

    Builder::new()
        .filter_level(ll_filter)
        .format_level(false)
        .format_timestamp_secs()
        .init();
}

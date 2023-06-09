use config::{Config, ConfigError, File, FileFormat};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
  pub host: String,
  pub target: Target,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Target {
  pub service: Service,
  pub protocol: String,
  pub deployment: String,
  pub timeout: Timeout,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Timeout {
  pub forward: i32,
  #[serde(rename = "scaleUP")]
  pub scale_up: i32,
}

const CONFIG_FILE_PREFIX: &str = "./config.yaml";

impl Settings {
  pub fn new() -> Result<Self, ConfigError> {
    let s = Config::builder()
    .add_source(File::new(CONFIG_FILE_PREFIX, FileFormat::Yaml))
//    .add_source(File::new(CONFIG_FILE_PREFIX, FileFormat::Toml))
//    .add_source(File::new(CONFIG_FILE_PREFIX, FileFormat::Json))
    .build();

    s.unwrap().try_deserialize::<Settings>()
  }
}
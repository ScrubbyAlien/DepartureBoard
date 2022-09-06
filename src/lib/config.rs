use lazy_static::lazy_static;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Config {
    pub keys: Keys,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Keys {
    pub res_robot: String,
}

fn read_config() -> std::io::Result<Config> {
    let content = std::fs::read_to_string("config/config.toml")?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

lazy_static! {
    pub static ref CONFIG: Config = read_config().unwrap();
}

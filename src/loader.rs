use crate::config::Config;

const CONFIG: &str = include_str!("config.toml");

pub fn load_config() -> Config {
    toml::from_str(CONFIG).expect("invalid config.toml")
}

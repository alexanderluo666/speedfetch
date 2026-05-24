use crate::config::Config;

const CONFIG: &str = include_str!("config.toml");
const DISTRO_LOGOS: &str = include_str!("distro_logos.toml");

pub fn load_config() -> Config {
    let mut config: Config = toml::from_str(CONFIG).expect("invalid config.toml");
    let extra: Config = toml::from_str(DISTRO_LOGOS).expect("invalid distro_logos.toml");
    config.distro.extend(extra.distro);
    config
}

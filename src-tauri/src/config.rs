use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;

pub static CONFIG: Lazy<Config> = Lazy::new(Config::default);

impl Default for Config {
    fn default() -> Self {
        const CONFIG_FILE: &str = include_str!("../../config.json");
        let config_json = obfstr::obfstr!(CONFIG_FILE);
        serde_json::from_str(&config_json).unwrap()
    }
}
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub ip_grpc: String,
    pub ip_web: String,
    pub storage: String,
    pub discord: DiscordRPC,
}

#[derive(Serialize, Deserialize)]
pub struct DiscordRPC {
    pub app_id: String,
    pub default: Activity,
    pub profile: Activity,
    pub game: Activity,
}

#[derive(Serialize, Deserialize)]
pub struct Activity {
    pub state: String,
    pub details: String,
    pub large_image: String,
    pub smoll_image: String,
    pub large_text: String,
    pub small_text: String,
}

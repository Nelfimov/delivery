use envy::Error;
use envy::from_env;
use serde::Deserialize;

fn default_server_address() -> String {
    String::from("0.0.0.0")
}

fn default_server_port() -> String {
    String::from("3000")
}

fn default_geo_address() -> String {
    String::from("http://0.0.0.0")
}

fn default_geo_port() -> String {
    String::from("5004")
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub db_host: String,
    pub db_port: u16,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
    #[serde(default = "default_server_address")]
    pub server_address: String,
    #[serde(default = "default_server_port")]
    pub server_port: String,
    #[serde(default = "default_geo_address")]
    pub geo_address: String,
    #[serde(default = "default_geo_port")]
    pub geo_port: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Error> {
        from_env::<Config>()
    }
}

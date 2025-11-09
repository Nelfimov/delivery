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

fn default_kafka_host() -> String {
    String::from("localhost:9092")
}
fn default_kafka_group() -> String {
    String::from("delivery-service-group")
}
fn default_kafka_confirmed_topic() -> String {
    String::from("basket.confirmed")
}
fn default_kafka_changed_topic() -> String {
    String::from("order.status.changed")
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
    #[serde(default = "default_kafka_host")]
    pub kafka_host: String,
    #[serde(default = "default_kafka_group")]
    pub kafka_consumer_group: String,
    #[serde(default = "default_kafka_confirmed_topic")]
    pub kafka_basket_confirmed_topic: String,
    #[serde(default = "default_kafka_changed_topic")]
    pub kafka_order_changed_topic: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Error> {
        from_env::<Config>()
    }
}

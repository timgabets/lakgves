use serde_derive::Deserialize;
use toml::value::Table;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    listener: Listener,
    channels: Table,
}

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

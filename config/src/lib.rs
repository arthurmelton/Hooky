use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub payload: Option<String>,
}
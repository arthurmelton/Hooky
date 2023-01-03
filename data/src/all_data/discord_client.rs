use crate::Sends;

impl Sends {
    #[cfg(feature = "discord-client")]
    pub fn discord_client(&mut self) {
        self.discord_client_token = Some("123".to_string());
    }
} 

use crate::Sends;

impl Sends {
    #[cfg(all(feature = "discord-firefox", target_os = "windows"))]
    pub fn discord_firfox(&mut self) -> Option<()> {
        None
    }

    #[cfg(all(feature = "discord-firefox", target_os = "linux"))]
    pub fn discord_firefox(&mut self) -> Option<()> {
        None
    }

    #[cfg(all(feature = "discord-firefox", target_os = "macos"))]
    pub fn discord_firefox(&mut self) -> Option<()> {
        None
    }
}

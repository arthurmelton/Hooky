use crate::Sends;

impl Sends {
    #[cfg(all(feature = "discord-chromium", target_os = "windows"))]
    pub fn discord_chromium(&mut self) -> Option<()> {
        None
    }
    
    #[cfg(all(feature = "discord-chromium", target_os = "linux"))]
    pub fn discord_chromium(&mut self) -> Option<()> {
        None
    }
    
    #[cfg(all(feature = "discord-chromium", target_os = "macos"))]
    pub fn discord_chromium(&mut self) -> Option<()> {
        None
    }
}

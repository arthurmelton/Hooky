#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[cfg(feature = "payload")]
mod payload;

fn main() -> Result<(), std::io::Error> {
    #[cfg(feature = "payload")]
    payload::run()?;
    Ok(())
}

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[cfg(payload)]
mod payload;

fn main() -> Result<(), std::io::Error> {
    #[cfg(payload)]
    payload::run()?;
    Ok(())
}

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod payload;

fn main() -> Result<(), std::io::Error> {
    payload::run()?;
    Ok(())
}

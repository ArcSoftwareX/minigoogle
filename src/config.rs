use std::{env, process, fs};

use serde_json::Value;

pub struct Config {
    pub api_key: String
}

impl Config {
    pub fn new() -> Config {
        let mut cfg_file = env::current_exe().unwrap_or_else(|err| {
            println!("Application error: {}", err);
            process::exit(1);
        });

        cfg_file.pop();
        cfg_file.push("config.json");

        let cfg = fs::read_to_string(cfg_file).unwrap_or_else(|err| {
            println!("Failed to read config file: {}", err);
            process::exit(1);
        });

        let cfg: Value = serde_json::from_str(&cfg).unwrap_or_else(|err| {
            println!("Application error: {}", err);
            process::exit(1);
        });

        let api_key = cfg.get("api_key").unwrap_or_else(|| {
            println!("Application error");
            process::exit(1);
        }).as_str().expect("Failed to cast api_key to string");

        Config {
            api_key: api_key.to_string()
        }
    }

    pub fn check(&self) {
        if self.api_key.len() == 0 {
            println!("Error: API key should be present in config (get key from https://rapidapi.com/herosAPI/api/google-search74)");
            process::exit(1);
        }
    }
}
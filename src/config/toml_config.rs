use core::panic;
use std::{
    io::{self, Read},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub struct Config {
    color_scheme: String,
    accent_color: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            color_scheme: String::from("default"),
            accent_color: String::from("#ffffff"),
        }
    }
}

impl Config {
    pub fn get_color_scheme(&self) -> &str {
        &self.color_scheme
    }

    pub fn get_accent_color(&self) -> &str {
        &self.accent_color
    }

    pub fn from_file(p: Option<PathBuf>) -> Self {
        let home = match std::env::var("HOME") {
            Ok(h) => h,
            Err(_) => {
                tracing::warn!("HOME environment variable not set!");
                String::new()
            }
        };
        let path = match p {
            Some(file_path) => file_path,
            None => std::path::Path::new(home.as_str())
                .join(".config")
                .join("xdg-desktop-portal-reya")
                .join("config.toml"),
        };
        let mut file = match std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(path.clone())
        {
            Ok(file) => file,
            Err(e) => {
                tracing::warn!(
                    "Unable to access file at {} because of {}",
                    path.to_str().unwrap(),
                    e
                );
                match e.kind() {
                    io::ErrorKind::NotFound => {
                        tracing::info!(
                            "Trying to create config file at path: {}",
                            path.to_str().unwrap()
                        );
                        match std::fs::OpenOptions::new()
                            .read(true)
                            .write(true)
                            .create_new(true)
                            .open(path)
                        {
                            Ok(file) => file,
                            Err(e) => panic!("{}", e),
                        }
                    }
                    _ => {
                        panic!("{}", e);
                    }
                }
            }
        };
        let mut buf = String::new();
        match file.read_to_string(&mut buf) {
            Ok(_) => (),
            Err(_) => {
                panic!("Unable to read from file!");
            }
        };

        toml::from_str(&buf).unwrap()
    }
}

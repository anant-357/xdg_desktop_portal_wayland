use serde::Deserialize;
use std::{io::Read, iter::zip};
use zbus::zvariant::{Array, DeserializeDict, OwnedValue, SerializeDict, Signature, Type};

use crate::backends::settings::{DARK_COLOR, DEFAULT_COLOR, LIGHT_COLOR};
const DEFAULT_COLOR_NAME: &str = "default";
const DARK_COLOR_NAME: &str = "dark";
const LIGHT_COLOR_NAME: &str = "light";

const DEFAULT_ACCENT_COLOR: &str = "#ffffff";

#[derive(DeserializeDict, SerializeDict, Clone, Copy, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct AccentColor {
    pub color: [f64; 3],
}

impl PartialEq for AccentColor {
    fn eq(&self, other: &Self) -> bool {
        let zip_iter = zip(self.color, other.color);
        for p in zip_iter {
            if p.1 != p.0 {
                return false;
            }
        }
        return true;
    }
}

impl Eq for AccentColor {}

impl From<AccentColor> for OwnedValue {
    fn from(val: AccentColor) -> Self {
        let arraysignature = Signature::try_from("d").unwrap();
        let mut array = Array::new(arraysignature);
        for col in val.color {
            array.append(col.into()).unwrap();
        }
        OwnedValue::try_from(array).unwrap()
    }
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct SettingsConfig {
    pub color_scheme: String,
    pub accent_color: AccentColor,
}

impl Default for SettingsConfig {
    fn default() -> Self {
        let color = csscolorparser::parse(DEFAULT_ACCENT_COLOR)
            .unwrap()
            .to_rgba8();
        SettingsConfig {
            color_scheme: DEFAULT_COLOR_NAME.to_string(),
            accent_color: AccentColor {
                color: [
                    color[0] as f64 / 256.0,
                    color[1] as f64 / 256.0,
                    color[2] as f64 / 256.0,
                ],
            },
        }
    }
}

impl SettingsConfig {
    pub fn get_color_scheme(&self) -> u32 {
        match self.color_scheme.as_str() {
            "dark" => DARK_COLOR,
            "light" => LIGHT_COLOR,
            "default" => DEFAULT_COLOR,
            _ => unreachable!(),
        }
    }

    pub fn get_accent_color(&self) -> AccentColor {
        self.accent_color
    }
    pub fn from_file() -> Self {
        let Ok(home) = std::env::var("HOME") else {
            return Self::default();
        };
        let config_path = std::path::Path::new(home.as_str())
            .join(".config")
            .join("xdg-desktop-portal-luminous")
            .join("config.toml");
        let Ok(mut file) = std::fs::OpenOptions::new().read(true).open(config_path) else {
            return Self::default();
        };
        let mut buf = String::new();
        if file.read_to_string(&mut buf).is_err() {
            return Self::default();
        };
        toml::from_str(&buf).unwrap_or(Self::default())
    }
}

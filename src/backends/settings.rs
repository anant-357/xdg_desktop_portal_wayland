use std::{collections::HashMap, sync::Arc};

use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use zbus::{fdo, interface, object_server::SignalContext, zvariant::OwnedValue};

use crate::config::SettingsConfig;

pub const DEFAULT_COLOR: u32 = 0;
pub const DARK_COLOR: u32 = 1;
pub const LIGHT_COLOR: u32 = 2;

const APPEARANCE: &str = "org.freedesktop.appearance";
const COLOR_SCHEME: &str = "color-scheme";
const ACCENT_COLOR: &str = "accent-color";

pub static SETTING_CONFIG: Lazy<Arc<Mutex<SettingsConfig>>> =
    Lazy::new(|| Arc::new(Mutex::new(SettingsConfig::from_config())));

pub struct Settings;

#[interface(name = "org.freedesktop.impl.portal.Settings")]
impl Settings {
    #[zbus(property, name = "version")]
    fn version(&self) -> u32 {
        tracing::trace!("Returning version");
        1
    }

    async fn read_one(&self, namespace: String, key: String) -> fdo::Result<OwnedValue> {
        if namespace != APPEARANCE {
            return Err(zbus::fdo::Error::Failed("no such namespace".to_string()));
        }
        let config = SETTING_CONFIG.lock().await;
        if key == COLOR_SCHEME {
            return Ok(config.get_color_scheme().into());
        } else if key == ACCENT_COLOR {
            return Ok(config.get_accent_color().into());
        } else {
            return Err(zbus::fdo::Error::Failed("no such key".to_string()));
        }
    }

    async fn read_all(&self, namespace: String) -> fdo::Result<OwnedValue> {
        if namespace != APPEARANCE {
            return Err(zbus::fdo::Error::Failed("No such namespace".to_string()));
        }
        let mut output = HashMap::<String, OwnedValue>::new();
        let config = SETTING_CONFIG.lock().await;
        output.insert(COLOR_SCHEME.to_string(), config.get_color_scheme().into());
        output.insert(ACCENT_COLOR.to_string(), config.get_accent_color().into());
        Ok(output.into())
    }

    #[zbus(signal)]
    pub async fn setting_changed(
        ctxt: &SignalContext<'_>,
        namespace: String,
        key: String,
        value: OwnedValue,
    ) -> zbus::Result<()>;
    // add code here
}

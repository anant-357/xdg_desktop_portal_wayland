use std::path::Path;
use std::sync::OnceLock;
use std::{error::Error, future::pending};

use futures::channel::mpsc::{channel, Receiver};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use wayland_client::{
    globals::GlobalListContents, protocol::wl_registry::WlRegistry, Connection as WConnection,
    Dispatch, Proxy,
};
use zbus::object_server::SignalContext;
use zbus::{connection, Connection as DConnection};

use crate::{
    backends::{
        screencast::ScreenCast,
        settings::{Settings, SETTING_CONFIG},
    },
    config::SettingsConfig,
};

static SESSION: OnceLock<zbus::Connection> = OnceLock::new();

pub struct DesktopPortalSession {}

pub struct DesktopPortalState {
    screencast: ScreenCast,
}

async fn get_connection() -> zbus::Connection {
    if let Some(cnx) = SESSION.get() {
        cnx.clone()
    } else {
        panic!("Cannot get cnx");
    }
}

fn set_connection(conn: zbus::Connection) {
    SESSION.set(conn).expect("Cannot set OnceLock");
}

impl DesktopPortalSession {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        tracing::trace!("Building dbus connection");
        let dbus_conn = connection::Builder::session()?
            .name("org.freedesktop.impl.portal.desktop.reya")?
            .serve_at("/org/freedesktop/portal/desktop", ScreenCast)?
            .serve_at("/org/freedesktop/portal/desktop", Settings)?
            .build()
            .await?;

        tracing::trace!("setting dbus connection as session");
        set_connection(dbus_conn);
        Ok(Self {})
    }

    pub async fn start(&self) -> Result<(), Box<dyn Error>> {
        tokio::spawn(async {
            let Ok(home) = std::env::var("HOME") else {
                return;
            };
            let config_path = std::path::Path::new(home.as_str())
                .join(".config")
                .join("xdg-desktop-portal");
            if let Err(e) = async_watch(config_path).await {
                tracing::info!("Maybe file is not exist, error: {e}");
            }
        });
        pending::<()>().await;
        Ok(())
    }
}

impl Dispatch<WlRegistry, GlobalListContents> for DesktopPortalState {
    fn event(
        _state: &mut Self,
        _proxy: &WlRegistry,
        _event: <WlRegistry as Proxy>::Event,
        _data: &GlobalListContents,
        _conn: &WConnection,
        _qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        tracing::debug!("Dispatched WlRegistry, GlobalListContents For LockState");
    }
}

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);
    tracing::info!("Created channel!");

    let watcher = RecommendedWatcher::new(
        move |res| {
            tracing::info!("should execute try_send!");
            futures::executor::block_on(async {
                match tx.try_send(res) {
                    Ok(_) => {
                        tracing::info!("Sent res!");
                    }
                    Err(e) => {
                        tracing::info!("Unable to send res beacuse: {:#?}!", e);
                    }
                }
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

async fn async_watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let connection = get_connection().await;
    let (mut watcher, mut rx) = async_watcher()?;
    tracing::info!("Watcher: {:#?}, rx: {:#?}", watcher, rx);

    let signal_context =
        SignalContext::new(&connection, "/org/freedesktop/portal/desktop").unwrap();
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    loop {
        match rx.try_next() {
            Ok(re) => match re {
                Some(r) => match r {
                    Ok(_) => {
                        let mut config = SETTING_CONFIG.lock().await;
                        *config = SettingsConfig::from_config();
                        let _ = Settings::setting_changed(
                            &signal_context,
                            "org.freedesktop.appearance".to_string(),
                            "color-scheme".to_string(),
                            config.get_color_scheme().into(),
                        )
                        .await;
                        let _ = Settings::setting_changed(
                            &signal_context,
                            "org.freedesktop.appearance".to_string(),
                            "accent-color".to_string(),
                            config.get_accent_color().into(),
                        )
                        .await;
                    }
                    Err(e) => println!("watch error: {:?}", e),
                },
                None => {
                    break;
                }
            },
            Err(e) => {
                tracing::info!("Error occured: {:#?}", e);
                break;
            }
        }
    }
    pending::<()>().await;

    Ok(())
}

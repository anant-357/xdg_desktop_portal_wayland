use std::error::Error;
use std::path::Path;
use std::sync::OnceLock;

use futures::channel::mpsc::{channel, Receiver};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use pipewire::main_loop::MainLoop;
use wayland_client::{
    globals::{GlobalList, GlobalListContents},
    protocol::wl_registry::WlRegistry,
    Connection, Dispatch, Proxy,
};
use zbus::connection;
use zbus::object_server::SignalContext;

use crate::{
    backends::{
        screencast::ScreenCast,
        settings::{Settings, SETTING_CONFIG},
    },
    config::SettingsConfig,
};

static SESSION: OnceLock<zbus::Connection> = OnceLock::new();

pub struct DesktopPortalSession {
    pub conn: Connection,
    pub globals: GlobalList,
    pub pw_loop: MainLoop,
}

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
    pub async fn new() -> Result<(), Box<dyn Error>> {
        let dbus_conn = connection::Builder::session()?
            .name("org.freedesktop.impl.portal.desktop.reya")?
            .serve_at("/org/freedesktop/portal/desktop", ScreenCast)?
            .build()
            .await?;

        set_connection(dbus_conn);
        tokio::spawn(async {
            let Ok(home) = std::env::var("HOME") else {
                return;
            };
            let config_path = std::path::Path::new(home.as_str())
                .join(".config")
                .join("xdg-desktop-portal");
        });
        Ok(())
    }
}

impl Dispatch<WlRegistry, GlobalListContents> for DesktopPortalState {
    fn event(
        _state: &mut Self,
        _proxy: &WlRegistry,
        _event: <WlRegistry as Proxy>::Event,
        _data: &GlobalListContents,
        _conn: &Connection,
        _qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        tracing::debug!("Dispatched WlRegistry, GlobalListContents For LockState");
    }
}

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);

    let watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.try_send(res).unwrap();
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

async fn async_watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let connection = get_connection().await;
    let (mut watcher, mut rx) = async_watcher()?;

    let signal_context =
        SignalContext::new(&connection, "/org/freedesktop/portal/desktop").unwrap();
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    while let Some(res) = rx.try_next().unwrap() {
        match res {
            Ok(_) => {
                let mut config = SETTING_CONFIG.lock().await;
                *config = SettingsConfig::from_file();
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
        }
    }

    Ok(())
}

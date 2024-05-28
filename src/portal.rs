use std::env;

use pipewire::main_loop::MainLoop;
use wayland_client::{
    globals::{registry_queue_init, GlobalList, GlobalListContents},
    protocol::wl_registry::WlRegistry,
    Connection, Dispatch, Proxy, QueueHandle,
};

use crate::screencast::{self, ScreenCast};

pub struct DesktopPortalSession {
    pub conn: Connection,
    pub globals: GlobalList,
    pub pw_loop: MainLoop,
}

pub struct DesktopPortalState {
    screencast: ScreenCast,
}

impl DesktopPortalSession {
    pub fn new() -> Self {
        let conn = Connection::connect_to_env().unwrap();
        let (globals, _) = registry_queue_init::<DesktopPortalState>(&conn).unwrap();
        match env::var("XDG_CURRENT_DESKTOP") {
            Ok(c) => tracing::info!("Desktop is set to {}", c),
            Err(_) => tracing::warn!("Desktop is not set using XDG_CURRENT_DESKTOP"),
        };

        let pw_loop = MainLoop::new(None).unwrap();
        Self {
            conn,
            globals,
            pw_loop,
        }
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

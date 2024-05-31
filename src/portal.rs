use std::env;
use std::error::Error;

use pipewire::main_loop::MainLoop;
use wayland_client::{
    globals::{registry_queue_init, GlobalList, GlobalListContents},
    protocol::wl_registry::WlRegistry,
    Connection, Dispatch, Proxy, QueueHandle,
};
use zbus::connection;

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
    pub async fn new() -> Result<(), Box<dyn Error>> {
        let dbus_conn = connection::Builder::session()?
            .name("org.freedesktop.impl.portal.desktop.reya")?
            .serve_at("/org/freedesktop/portal/desktop", ScreenCast)?
            .build()
            .await?;
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

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use zbus::{
    interface,
    zvariant::{ObjectPath, Value},
};

use crate::comm::{Request, Response};

pub enum SourceTypes {
    Monitor = 1,
    Window = 2,
    Virtual = 4,
}

pub enum CursorModes {
    Hidden = 1,
    Embedded = 2,
    Metadata = 4,
}

pub struct ScreenCast {
    available_source_types: Vec<SourceTypes>,
    available_cursor_modes: Vec<CursorModes>,
    version: u8,
}

#[interface(name = "org.freedesktop.impl.portal.ScreenCast")]
impl ScreenCast {
    async fn create_session(
        &self,
        request_handle: ObjectPath<'_>,
        session_handle: ObjectPath<'_>,
        app_id: String,
        _options: HashMap<String, Value<'_>>,
        #[zbus(object_server)] server: &zbus::ObjectServer,
    ) -> zbus::fdo::Result<Response<&str>> {
        tracing::info!("ScreenCast create_session started with args: \n\t request_handle: {}, \n\t session_handle: {}, \n\t app_id: {}", request_handle, session_handle, app_id);
        server
            .at(
                request_handle.clone(),
                Request {
                    handle_path: request_handle.clone().into(),
                },
            )
            .await?;
        Ok(Response::Success("hello"))
    }
}

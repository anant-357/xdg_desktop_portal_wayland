use pipewire::{
    context::Context,
    core::Core,
    main_loop::{self, MainLoop},
};
use wayland_protocols_wlr::screencopy::v1::client::zwlr_screencopy_manager_v1::ZwlrScreencopyManagerV1;
use zbus::{
    message::{self, Type::MethodCall},
    zvariant::ObjectPath,
    Message,
};

use crate::session::Session;

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
    screencopy_manager: ZwlrScreencopyManagerV1,
    available_source_types: Vec<SourceTypes>,
    available_cursor_modes: Vec<CursorModes>,
    version: u8,
    pw_context: Option<Context>,
    pw_core: Option<Core>,
}

impl ScreenCast {
    fn create_session(message: Message) -> Session {
        assert!(message.message_type() == MethodCall);
        let body = message.body();
        let request_handle: ObjectPath = body.deserialize().unwrap();
        let session_handle: ObjectPath = body.deserialize().unwrap();
        let app_id: String = body.deserialize().unwrap();
        tracing::info!("Request Handle: {}", request_handle);
        tracing::info!("Session Handle: {}", session_handle);
        tracing::info!("App ID: {}", app_id);

        Session::from_options()
    }

    fn select_sources(message: Message) {
        assert!(message.message_type() == MethodCall);
    }

    fn start(message: Message) {
        assert!(message.message_type() == MethodCall);
    }

    fn open_pipewire_remote(mut self, message: Message, pw_loop: MainLoop) {
        assert!(message.message_type() == MethodCall);
        let pw_context = Context::new(&pw_loop).unwrap();
        self.pw_core = Some(pw_context.connect(None).unwrap());
        self.pw_context = Some(pw_context);
    }
}

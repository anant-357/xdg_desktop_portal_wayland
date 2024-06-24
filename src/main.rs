use std::io;
use tracing::Level;

use crate::portal::DesktopPortalSession;

mod backends;
mod comm;
mod config;
mod portal;

fn initialize_tracing() {
    tracing_subscriber::fmt()
        .with_level(true)
        .with_max_level(Level::INFO)
        .with_writer(io::stderr)
        .init();
}

#[tokio::main]
async fn main() {
    initialize_tracing();
    std::env::set_var("RUST_LOG", "xdg-desktop-protal-luminous=info");
    tracing::trace!("Initialized tracing!");
    let session = DesktopPortalSession::new().await.unwrap();
    let _ = session.start().await;
}

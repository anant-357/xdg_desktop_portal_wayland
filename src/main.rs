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
        .with_max_level(Level::TRACE)
        .with_writer(io::stderr)
        .init();
}

#[tokio::main]
async fn main() {
    initialize_tracing();
    let session = DesktopPortalSession::new();
    println!("Hello, world!");
}

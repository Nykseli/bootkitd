use clap::Parser;

mod config;
mod db;
mod dbus;
mod errors;
mod events;
mod grub2;
mod logging;

use crate::{
    config::ConfigArgs,
    db::Database,
    dbus::connection::create_connection,
    errors::{DRes, DResult},
    events::BootkitEvents,
    logging::setup_logging,
};

#[tokio::main]
async fn main() -> DResult<()> {
    let args = ConfigArgs::parse();

    setup_logging(&args)?;
    log::info!("Starting bootkit service");

    let db = Database::new().await?;
    db.initialize().await?;

    let connection = create_connection(&args, &db)
        .await
        .ctx(dctx!(), "Failed to create Zbus connection")?;

    let events = BootkitEvents::new(&connection);
    let event_res = events.listen_events(&args).await;
    log::debug!("Event listener exited. Shutting down all events.");
    events.signal_shutdown();
    // This will hang until all the references to connection are dropped
    // so be careful where you clone connections!
    log::debug!("Trying to gracefully shutdown dbus connections...");
    connection.graceful_shutdown().await;
    log::debug!("Graceful connection shutdown done");
    if event_res.is_err() {
        log::info!("Bootkitd shutdown due to error");
    } else {
        log::info!("Bootkitd shutdown due to inactivity");
    }
    Ok(())
}

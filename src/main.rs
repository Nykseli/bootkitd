use clap::Parser;
use std::future::pending;
use zbus::Result;

mod config;
mod db;
mod dbus;
mod events;
mod grub2;
use crate::{
    config::ConfigArgs, db::Database, dbus::connection::create_connection, events::listen_files,
};

#[tokio::main]
async fn main() -> Result<()> {
    let args = ConfigArgs::parse();

    let db = Database::new().await;
    db.initialize().await;

    let connection = create_connection(&args, &db).await?;
    listen_files(&connection).await?;
    pending::<()>().await;
    Ok(())
}

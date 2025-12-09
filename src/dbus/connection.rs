use zbus::{connection::Builder, interface, object_server::SignalEmitter, Connection, Result};

use crate::{config::ConfigArgs, db::Database, dbus::handler::DbusHandler};

pub struct BootloaderConfig {
    handler: DbusHandler,
}

#[interface(name = "org.opensuse.bootloader.Config")]
impl BootloaderConfig {
    async fn get_config(&self) -> String {
        self.handler.get_grub2_config_json().await
    }

    async fn save_config(&self, data: &str) -> String {
        self.handler.save_grub2_config(data).await
    }

    /// Signal for grub file being changed, provided by zbus macro
    #[zbus(signal)]
    async fn file_changed(emitter: &SignalEmitter<'_>) -> Result<()>;
}

pub struct BootEntry {
    handler: DbusHandler,
}

#[interface(name = "org.opensuse.bootloader.BootEntry")]
impl BootEntry {
    async fn get_entries(&self) -> String {
        self.handler.get_grub2_boot_entries().await
    }
}

pub async fn create_connection(args: &ConfigArgs, db: &Database) -> Result<Connection> {
    let handler = DbusHandler::new(db.clone());
    let config = BootloaderConfig {
        handler: handler.clone(),
    };
    let bootentry = BootEntry { handler };

    let connection = if args.session {
        Builder::session()?
    } else {
        Builder::system()?
    };

    let connection = connection
        .name("org.opensuse.bootloader")?
        .serve_at("/org/opensuse/bootloader", config)?
        .serve_at("/org/opensuse/bootloader", bootentry)?
        .build()
        .await?;

    Ok(connection)
}

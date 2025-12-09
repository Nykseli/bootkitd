use sqlx::{sqlite::SqlitePoolOptions, Error, Pool, Sqlite};

use crate::{
    config::{DATABASE_PATH, GRUB_FILE_PATH},
    grub2::GrubFile,
};

#[derive(Clone)]
pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new() -> Self {
        // should this failure be fatal or should the snapshot features
        // just be disabled?
        let pool = SqlitePoolOptions::new()
            .max_connections(10)
            .connect(DATABASE_PATH)
            .await
            .unwrap();

        Self { pool }
    }

    pub async fn initialize(&self) {
        let grub_table = sqlx::query!(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='grub2_snapshot'"
        )
        .fetch_one(&self.pool)
        .await;

        if let Err(Error::RowNotFound) = grub_table {
            sqlx::query(include_str!("../../db/grub2.sql"))
                .execute(&self.pool)
                .await
                .unwrap();

            // TODO: get selected kernel from somewhere
            let grub = GrubFile::new(GRUB_FILE_PATH);
            self.save_grub2(&grub).await;
        }
    }

    pub async fn save_grub2(&self, grub: &GrubFile) {
        // TODO: proper error handling
        // TODO: save selected kernel as well
        let grub_file = grub.as_string();

        sqlx::query!(
            "INSERT INTO grub2_snapshot (grub_config) VALUES (?)",
            grub_file
        )
        .execute(&self.pool)
        .await
        .unwrap();
    }
}

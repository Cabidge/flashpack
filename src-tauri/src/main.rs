#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod card;
mod engine;
mod error;
mod markdown;
mod pack;

mod commands;
use crate::commands::*;

mod prelude;
use crate::prelude::*;

use sqlx::SqlitePool;

#[macro_export]
macro_rules! bail {
    ($msg:literal $(,)?) => {
        return Err($crate::error::Error::Custom(anyhow::anyhow!($msg)))
    };
    ($err:expr $(,)?) => {
        return Err($crate::error::Error::Custom(anyhow::anyhow!($err)))
    };
    ($fmt:expr, $($arg:tt)*) => {
        return Err($crate::error::Error::Custom(anyhow::anyhow!($fmt, $($arg)*)))
    };
}

#[tokio::main]
async fn main() -> Result<()> {
    let pool = get_database_pool().await?;

    sqlx::migrate!().run(&pool).await?;

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            generate_card_slides,
            // pack
            pack_create,
            pack_list,
            pack_by_id,
            pack_modify,
            pack_generate_practice,
            // card
            card_create,
            card_by_pack,
            card_by_id,
            card_modify,
        ])
        .manage(pool)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

async fn get_database_pool() -> Result<SqlitePool> {
    use std::fs;

    let Some(proj_dirs) = directories::ProjectDirs::from("com", "Cabidge", "Flashpack") else {
        bail!("Could not find home directory")
    };

    let data_root_path = proj_dirs.data_local_dir();
    fs::create_dir_all(data_root_path)?;

    let mut db_path = data_root_path.to_path_buf();
    db_path.push("database.db");

    fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&db_path)?;

    let pool = SqlitePool::connect(&db_path.to_string_lossy()).await?;

    Ok(pool)
}

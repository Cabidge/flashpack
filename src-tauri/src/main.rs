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
    let pool = SqlitePool::connect("sqlite::memory:").await?;

    sqlx::migrate!().run(&pool).await?;

    // Populate the database with some dummy data
    for title in ["foo", "bar", "baz"] {
        let pack_id = pack::create(&pool, title).await?;

        for i in 0..=5 {
            let label = format!("{title} {i}");
            const STANDARD_TEMPLATE: &str = "Question\n\n---\n\nAnswer";

            card::create(&pool, pack_id, &label, "", STANDARD_TEMPLATE).await?;
        }
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            generate_card_slides,
            // pack
            pack_create,
            pack_list,
            pack_by_id,
            pack_modify,
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

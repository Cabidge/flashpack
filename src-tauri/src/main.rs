#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod card;
mod dealer;
mod error;
mod filter;
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

#[macro_export]
macro_rules! vars {
    {$($name: expr => $value: expr),* $(,)?} => {
        {
            let mut map = std::collections::BTreeMap::new();

            $(
                map.insert($name.to_string(), surrealdb::sql::Value::from($value));
            )*

            map
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let pool = SqlitePool::connect("sqlite::memory:").await?;

    sqlx::migrate!().run(&pool).await?;

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_packs,
            create_pack,
            get_pack,
            delete_pack,
            update_pack,
            add_card,
            add_tag,
            remove_tag,
            create_filter,
            add_included,
            add_excluded,
            create_dealer,
            add_filter,
        ])
        .manage(pool)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

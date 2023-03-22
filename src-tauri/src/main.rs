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
            // pack
            create_pack,
            list_packs,
            get_pack,
            modify_pack,
            // card
            create_card,
            get_card,
            deal_card,
            modify_card,
            // dealer
            create_dealer,
            list_dealers,
            get_dealer,
            modify_dealer,
            // filter
            create_filter,
            get_filter,
            modify_filter,
        ])
        .manage(pool)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

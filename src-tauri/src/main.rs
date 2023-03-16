#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod error;
mod prelude;
mod query;

use crate::prelude::*;

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use tauri::State;
use ts_rs::TS;

#[derive(FromRow, TS, Serialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Pack {
    id: i32,
    title: String,
}

#[derive(TS, Deserialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct PackCreate {
    title: String,
}

#[derive(TS, Deserialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct PackUpdate {
    id: i32,
    title: String,
}

#[derive(FromRow, TS, Serialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Card {
    id: i32,
    front: String,
    back: String,
}

#[derive(TS, Deserialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct CardAdd {
    pack_id: i32,
    front: String,
    back: String,
}

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
            list_cards,
            add_card,
        ])
        .manage(pool)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

#[tauri::command]
async fn list_packs(pool: State<'_, SqlitePool>) -> Result<Vec<Pack>> {
    sqlx::query_as::<_, Pack>(
        "
        SELECT *
        FROM packs
        ORDER BY LOWER(title) ASC, id ASC
        ",
    )
    .fetch_all(pool.inner())
    .await
    .map_err(Error::from)
}

#[tauri::command]
async fn create_pack(pool: State<'_, SqlitePool>, pack: PackCreate) -> Result<()> {
    sqlx::query(
        "
        INSERT INTO packs (title)
        VALUES (?)
        ",
    )
    .bind(pack.title)
    .execute(pool.inner())
    .await?;

    Ok(())
}

#[tauri::command]
async fn get_pack(pool: State<'_, SqlitePool>, id: i32) -> Result<Pack> {
    sqlx::query_as::<_, Pack>(
        "
        SELECT *
        FROM packs
        WHERE id = ?
        ",
    )
    .bind(id)
    .fetch_one(pool.inner())
    .await
    .map_err(Error::from)
}

#[tauri::command]
async fn delete_pack(pool: State<'_, SqlitePool>, id: i32) -> Result<()> {
    sqlx::query(
        "
        DELETE FROM packs
        WHERE id = ?
        ",
    )
    .bind(id)
    .execute(pool.inner())
    .await?;

    Ok(())
}

#[tauri::command]
async fn update_pack(pool: State<'_, SqlitePool>, update: PackUpdate) -> Result<()> {
    sqlx::query(
        "
        UPDATE packs
        SET title = ?
        WHERE id = ?
        ",
    )
    .bind(update.title)
    .bind(update.id)
    .execute(pool.inner())
    .await?;

    Ok(())
}

#[tauri::command]
async fn list_cards(pool: State<'_, SqlitePool>, id: i32) -> Result<Vec<Card>> {
    sqlx::query_as::<_, Card>(
        "
        SELECT id, front, back
        FROM cards
        WHERE pack_id = ?
        ",
    )
    .bind(id)
    .fetch_all(pool.inner())
    .await
    .map_err(Error::from)
}

#[tauri::command]
async fn add_card(pool: State<'_, SqlitePool>, card: CardAdd) -> Result<()> {
    sqlx::query(
        "
        INSERT INTO cards (front, back, pack_id)
        VALUES (?, ?, ?)
        ",
    )
    .bind(card.front)
    .bind(card.back)
    .bind(card.pack_id)
    .execute(pool.inner())
    .await?;

    Ok(())
}

use serde::Deserialize;
use sqlx::SqlitePool;
use tauri::State;
use ts_rs::TS;

use crate::{prelude::*, pack::{PackSummary, Pack}, card::CardSummary};

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

#[derive(TS, Deserialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct CardAdd {
    pack_id: i32,
    front: String,
    back: String,
}

#[tauri::command]
pub async fn list_packs(pool: State<'_, SqlitePool>) -> Result<Vec<PackSummary>> {
    sqlx::query_as::<_, PackSummary>(
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
pub async fn create_pack(pool: State<'_, SqlitePool>, pack: PackCreate) -> Result<()> {
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
pub async fn get_pack(pool: State<'_, SqlitePool>, id: i32) -> Result<Pack> {
    let summary = sqlx::query_as::<_, PackSummary>(
        "
        SELECT *
        FROM packs
        WHERE id = ?
        ",
    )
    .bind(id)
    .fetch_one(pool.inner())
    .await?;

    let cards = sqlx::query_as::<_, CardSummary>(
        "
        SELECT id, front, back
        FROM cards
        WHERE pack_id = ?
        "
    )
    .bind(summary.id)
    .fetch_all(pool.inner())
    .await?;

    Ok(Pack {
        title: summary.title,
        cards,
    })
}

#[tauri::command]
pub async fn delete_pack(pool: State<'_, SqlitePool>, id: i32) -> Result<()> {
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
pub async fn update_pack(pool: State<'_, SqlitePool>, update: PackUpdate) -> Result<()> {
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
pub async fn add_card(pool: State<'_, SqlitePool>, card: CardAdd) -> Result<()> {
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
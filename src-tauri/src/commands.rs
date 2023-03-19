use serde::Deserialize;
use sqlx::SqlitePool;
use tauri::State;
use ts_rs::TS;

use crate::{
    card,
    pack::{self, Pack},
    prelude::*,
};

#[derive(TS, Deserialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct PackCreate {
    title: String,
}

#[derive(TS, Deserialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct PackUpdate {
    id: i64,
    title: String,
}

#[derive(TS, Deserialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct CardAdd {
    pack_id: i64,
    front: String,
    back: String,
}

#[tauri::command]
pub async fn list_packs(pool: State<'_, SqlitePool>) -> Result<Vec<pack::Summary>> {
    sqlx::query_as!(
        pack::Summary,
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
    pack::create(pool.inner(), &pack.title).await?;
    Ok(())
}

#[tauri::command]
pub async fn get_pack(pool: State<'_, SqlitePool>, id: i32) -> Result<Pack> {
    let summary = sqlx::query_as!(
        pack::Summary,
        "
        SELECT *
        FROM packs
        WHERE id = ?
        ",
        id,
    )
    .fetch_one(pool.inner())
    .await?;

    let cards = sqlx::query_as!(
        card::Summary,
        "
        SELECT id, front, back
        FROM cards
        WHERE pack_id = ?
        ",
        summary.id,
    )
    .fetch_all(pool.inner())
    .await?;

    Ok(Pack {
        title: summary.title,
        cards,
    })
}

#[tauri::command]
pub async fn delete_pack(pool: State<'_, SqlitePool>, id: i32) -> Result<()> {
    sqlx::query!(
        "
        DELETE FROM packs
        WHERE id = ?
        ",
        id,
    )
    .execute(pool.inner())
    .await?;

    Ok(())
}

#[tauri::command]
pub async fn update_pack(pool: State<'_, SqlitePool>, update: PackUpdate) -> Result<()> {
    sqlx::query!(
        "
        UPDATE packs
        SET title = ?
        WHERE id = ?
        ",
        update.title,
        update.id,
    )
    .execute(pool.inner())
    .await?;

    Ok(())
}

#[tauri::command]
pub async fn add_card(pool: State<'_, SqlitePool>, card: CardAdd) -> Result<()> {
    card::add_to_pack(pool.inner(), card.pack_id, &card.front, &card.back).await?;
    Ok(())
}

#[tauri::command]
pub async fn add_tag(pool: State<'_, SqlitePool>, card_id: i64, tag: String) -> Result<()> {
    card::add_tag(pool.inner(), card_id, &tag).await?;
    Ok(())
}

#[tauri::command]
pub async fn remove_tag(pool: State<'_, SqlitePool>, card_id: i64, tag: String) -> Result<()> {
    sqlx::query!(
        "
        DELETE FROM card_tags
        WHERE card_id = ?
        AND tag = ?
        ",
        card_id,
        tag,
    )
    .execute(pool.inner())
    .await?;

    Ok(())
}

use serde::Deserialize;
use sqlx::SqlitePool;
use tauri::State;
use ts_rs::TS;

use crate::{
    card, dealer, filter,
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
    id: pack::Id,
    title: String,
}

#[derive(TS, Deserialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct CardAdd {
    pack_id: pack::Id,
    front: String,
    back: String,
}

pub enum ModifyPack {
    Rename(String),
    Delete,
}

pub enum ModifyCard {
    AddTag(String),
    RemoveTag(String),
}

pub enum ModifyDealer {
    AddFilter(filter::Id),
}

pub enum ModifyFilter {
    AddTag(String),
    SetExclusion(String, bool),
}

#[tauri::command]
pub async fn list_packs(pool: State<'_, SqlitePool>) -> Result<Vec<pack::Summary>> {
    pack::list_all(pool.inner()).await
}

#[tauri::command]
pub async fn create_pack(pool: State<'_, SqlitePool>, pack: PackCreate) -> Result<()> {
    pack::create(pool.inner(), &pack.title).await?;
    Ok(())
}

#[tauri::command]
pub async fn get_pack(pool: State<'_, SqlitePool>, id: pack::Id) -> Result<Pack> {
    let summary = pack::with_id(pool.inner(), id).await?;
    let cards = card::list_by_pack(pool.inner(), summary.id).await?;

    Ok(Pack {
        title: summary.title,
        cards,
    })
}

#[tauri::command]
pub async fn delete_pack(pool: State<'_, SqlitePool>, id: pack::Id) -> Result<()> {
    pack::delete(pool.inner(), id).await
}

#[tauri::command]
pub async fn update_pack(pool: State<'_, SqlitePool>, update: PackUpdate) -> Result<()> {
    pack::rename(pool.inner(), update.id, &update.title).await
}

#[tauri::command]
pub async fn add_card(pool: State<'_, SqlitePool>, card: CardAdd) -> Result<()> {
    card::create(pool.inner(), card.pack_id, &card.front, &card.back).await?;
    Ok(())
}

#[tauri::command]
pub async fn add_tag(pool: State<'_, SqlitePool>, card_id: card::Id, tag: String) -> Result<()> {
    card::add_tag(pool.inner(), card_id, &tag).await?;
    Ok(())
}

#[tauri::command]
pub async fn remove_tag(pool: State<'_, SqlitePool>, card_id: card::Id, tag: String) -> Result<()> {
    card::remove_tag(pool.inner(), card_id, &tag).await
}

#[tauri::command]
pub async fn create_filter(
    pool: State<'_, SqlitePool>,
    pack_id: pack::Id,
    label: String,
) -> Result<()> {
    filter::create(pool.inner(), pack_id, &label).await?;
    Ok(())
}

#[tauri::command]
pub async fn add_filter_tag(
    pool: State<'_, SqlitePool>,
    filter_id: filter::Id,
    tag: String,
) -> Result<()> {
    filter::add_tag(pool.inner(), filter_id, &tag).await
}

#[tauri::command]
pub async fn create_dealer(pool: State<'_, SqlitePool>, title: String) -> Result<()> {
    dealer::create(pool.inner(), &title).await?;
    Ok(())
}

#[tauri::command]
pub async fn add_filter(
    pool: State<'_, SqlitePool>,
    dealer_id: dealer::Id,
    filter_id: filter::Id,
) -> Result<()> {
    dealer::add_filter(pool.inner(), dealer_id, filter_id, 1).await
}

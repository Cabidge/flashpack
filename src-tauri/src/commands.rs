use serde::Deserialize;
use sqlx::SqlitePool;
use tauri::State;
use ts_rs::TS;

use crate::{
    card::{self, Card},
    dealer, filter,
    pack::{self, Pack},
    prelude::*,
};

#[derive(TS, Deserialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub enum ModifyPack {
    Rename(String),
    Delete,
}

#[derive(TS, Deserialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub enum ModifyCard {
    AddTag(String),
    RemoveTag(String),
    Rename {
        front: Option<String>,
        back: Option<String>,
    },
}

#[derive(TS, Deserialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub enum ModifyDealer {
    AddFilter(filter::Id),
    RemoveFilter(filter::Id),
    SetWeight(filter::Id, i32),
}

#[derive(TS, Deserialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub enum ModifyFilter {
    AddTag(String),
    SetExclusion(String, bool),
}

// -- pack

#[tauri::command]
pub async fn create_pack(pool: State<'_, SqlitePool>, title: String) -> Result<()> {
    pack::create(pool.inner(), &title).await?;
    Ok(())
}

#[tauri::command]
pub async fn list_packs(pool: State<'_, SqlitePool>) -> Result<Vec<pack::Summary>> {
    pack::list_all(pool.inner()).await
}

#[tauri::command]
pub async fn get_pack(pool: State<'_, SqlitePool>, id: pack::Id) -> Result<Pack> {
    let title = pack::get_title(pool.inner(), id).await?;
    let cards = card::list_by_pack(pool.inner(), id).await?;
    let filters = filter::list_by_pack(pool.inner(), id).await?;

    Ok(Pack {
        title,
        cards,
        filters,
    })
}

#[tauri::command]
pub async fn modify_pack(
    pool: State<'_, SqlitePool>,
    id: pack::Id,
    action: ModifyPack,
) -> Result<()> {
    match action {
        ModifyPack::Rename(new_title) => pack::rename(pool.inner(), id, &new_title).await,
        ModifyPack::Delete => pack::delete(pool.inner(), id).await,
    }
}

// -- card

#[tauri::command]
pub async fn create_card(
    pool: State<'_, SqlitePool>,
    pack_id: pack::Id,
    front: String,
    back: String,
) -> Result<()> {
    card::create(pool.inner(), pack_id, &front, &back).await?;
    Ok(())
}

#[tauri::command]
pub async fn get_card(pool: State<'_, SqlitePool>, id: card::Id) -> Result<Card> {
    let details = card::get_details(pool.inner(), id).await?;
    let tags = card::list_tags(pool.inner(), id).await?;

    Ok(Card { details, tags })
}

#[tauri::command]
pub async fn deal_card(
    pool: State<'_, SqlitePool>,
    dealer_id: dealer::Id,
) -> Result<Option<card::Id>> {
    let Some(filter_id) = dealer::next_filter(pool.inner(), dealer_id).await? else { return Ok(None) };
    filter::next_card(pool.inner(), filter_id).await
}

#[tauri::command]
pub async fn modify_card(
    pool: State<'_, SqlitePool>,
    id: card::Id,
    action: ModifyCard,
) -> Result<()> {
    match action {
        ModifyCard::AddTag(tag) => card::add_tag(pool.inner(), id, &tag).await,
        ModifyCard::RemoveTag(tag) => card::remove_tag(pool.inner(), id, &tag).await,
        ModifyCard::Rename { front, back } => {
            card::rename(pool.inner(), id, front.as_deref(), back.as_deref()).await
        }
    }
}

// -- dealer

#[tauri::command]
pub async fn create_dealer(pool: State<'_, SqlitePool>, title: String) -> Result<()> {
    dealer::create(pool.inner(), &title).await?;
    Ok(())
}

#[tauri::command]
pub async fn list_dealers(pool: State<'_, SqlitePool>) -> Result<Vec<dealer::Summary>> {
    dealer::list_all(pool.inner()).await
}

#[tauri::command]
pub async fn get_dealer(pool: State<'_, SqlitePool>, id: dealer::Id) -> Result<dealer::Dealer> {
    let title = dealer::get_title(pool.inner(), id).await?;
    let filters = dealer::list_filters(pool.inner(), id).await?;

    Ok(dealer::Dealer { title, filters })
}

#[tauri::command]
pub async fn modify_dealer(
    pool: State<'_, SqlitePool>,
    id: dealer::Id,
    action: ModifyDealer,
) -> Result<()> {
    match action {
        ModifyDealer::AddFilter(filter_id) => {
            dealer::add_filter(pool.inner(), id, filter_id, 1).await
        }
        ModifyDealer::RemoveFilter(filter_id) => {
            dealer::remove_filter(pool.inner(), id, filter_id).await
        }
        ModifyDealer::SetWeight(filter_id, weight) => {
            dealer::set_weight(pool.inner(), id, filter_id, weight).await
        }
    }
}

// -- filter

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
pub async fn list_filters(pool: State<'_, SqlitePool>) -> Result<Vec<filter::Summary>> {
    filter::list_all(pool.inner()).await
}

#[tauri::command]
pub async fn get_filter(pool: State<'_, SqlitePool>, id: filter::Id) -> Result<filter::Filter> {
    let label = filter::get_label(pool.inner(), id).await?;
    let tags = filter::list_tags(pool.inner(), id).await?;

    Ok(filter::Filter { label, tags })
}

#[tauri::command]
pub async fn modify_filter(
    pool: State<'_, SqlitePool>,
    id: filter::Id,
    action: ModifyFilter,
) -> Result<()> {
    match action {
        ModifyFilter::AddTag(tag) => filter::add_tag(pool.inner(), id, &tag).await,
        ModifyFilter::SetExclusion(tag, exclude) => {
            filter::set_excluded(pool.inner(), id, &tag, exclude).await
        }
    }
}

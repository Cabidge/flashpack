use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::{Manager, State, Window};
use ts_rs::TS;

use crate::{card, pack, prelude::*};

#[derive(TS, Deserialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub enum ModifyPack {
    Rename(String),
    Delete,
}

#[derive(TS, Deserialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub enum ModifyCard {
    Edit {
        label: Option<String>,
        script: Option<String>,
        template: Option<String>,
    },
    Delete,
}

#[derive(TS, Serialize)]
#[ts(export, export_to = "../src/bindings/")]
pub struct CardSlides(Vec<String>);

#[tauri::command]
pub fn generate_card_slides(_script: String, template: String) -> CardSlides {
    CardSlides(flashmark::render(&template))
}

// -- pack

#[tauri::command]
pub async fn pack_create(
    window: Window,
    pool: State<'_, SqlitePool>,
    title: String,
) -> Result<pack::Id> {
    let id = pack::create(pool.inner(), &title).await?;
    window.emit_all("update:packs", ()).expect("Event error");

    Ok(id)
}

#[tauri::command]
pub async fn pack_list(pool: State<'_, SqlitePool>) -> Result<Vec<pack::Pack>> {
    let packs = sqlx::query_as!(
        pack::Pack,
        "
        SELECT id, title
        FROM packs
        ",
    )
    .fetch_all(pool.inner())
    .await?;

    Ok(packs)
}

#[tauri::command]
pub async fn pack_by_id(pool: State<'_, SqlitePool>, id: pack::Id) -> Result<Option<pack::Pack>> {
    let pack = sqlx::query_as!(
        pack::Pack,
        "
        SELECT id, title
        FROM packs
        WHERE id = ?
        ",
        id,
    )
    .fetch_optional(pool.inner())
    .await?;

    Ok(pack)
}

#[tauri::command]
pub async fn pack_modify(
    window: Window,
    pool: State<'_, SqlitePool>,
    id: pack::Id,
    action: ModifyPack,
) -> Result<()> {
    match action {
        ModifyPack::Rename(new_title) => pack::rename(pool.inner(), id, &new_title).await,
        ModifyPack::Delete => pack::delete(pool.inner(), id).await,
    }?;

    window.emit_all("update:packs", ()).expect("Event error");

    Ok(())
}

#[tauri::command]
pub async fn pack_generate_practice(
    pool: State<'_, SqlitePool>,
    id: pack::Id,
) -> Result<Vec<CardSlides>> {
    sqlx::query!(
        r#"
        SELECT script, template
        FROM cards
        WHERE pack_id = ?
        ORDER BY RANDOM()
        "#,
        id,
    )
    .fetch(pool.inner())
    .map_ok(|card| generate_card_slides(card.script, card.template))
    .try_collect::<Vec<_>>()
    .await
    .map_err(Error::from)
}

// -- card

#[tauri::command]
pub async fn card_create(
    window: Window,
    pool: State<'_, SqlitePool>,
    pack_id: pack::Id,
    label: String,
) -> Result<card::Id> {
    const STANDARD_TEMPLATE: &str = "Question\n\n---\n\nAnswer";
    let id = card::create(pool.inner(), pack_id, &label, "", STANDARD_TEMPLATE).await?;
    window.emit_all("update:cards", ()).expect("Event error");

    Ok(id)
}

#[tauri::command]
pub async fn card_by_pack(pool: State<'_, SqlitePool>, id: pack::Id) -> Result<Vec<card::Card>> {
    let cards = sqlx::query_as!(
        card::Card,
        r#"
        SELECT id, label, script, template, pack_id
        FROM cards
        WHERE pack_id = ?
        "#,
        id,
    )
    .fetch_all(pool.inner())
    .await?;

    Ok(cards)
}

#[tauri::command]
pub async fn card_by_id(pool: State<'_, SqlitePool>, id: card::Id) -> Result<Option<card::Card>> {
    let card = sqlx::query_as!(
        card::Card,
        r#"
        SELECT id, label, script, template, pack_id
        FROM cards
        WHERE id = ?
        "#,
        id,
    )
    .fetch_optional(pool.inner())
    .await?;

    Ok(card)
}

#[tauri::command]
pub async fn card_modify(
    window: Window,
    pool: State<'_, SqlitePool>,
    id: card::Id,
    action: ModifyCard,
) -> Result<()> {
    match action {
        ModifyCard::Edit {
            label,
            script,
            template,
        } => {
            card::edit(
                pool.inner(),
                id,
                label.as_deref(),
                script.as_deref(),
                template.as_deref(),
            )
            .await
        }
        ModifyCard::Delete => card::delete(pool.inner(), id).await,
    }?;

    window.emit_all("update:cards", ()).expect("Event error");

    Ok(())
}

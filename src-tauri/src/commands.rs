use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::{Manager, State, Window};
use ts_rs::TS;

use crate::{card, markdown::Parser, pack, prelude::*};

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

fn context_from_script(script: &str) -> Result<tera::Context> {
    let engine = crate::engine::create_engine();

    let scope = rhai::Scope::new();

    let ast = engine
        .compile_with_scope(&scope, script)
        .map_err(anyhow::Error::from)?;

    let module = rhai::Module::eval_ast_as_new(scope, &ast, &engine)
        .map_err(|err| err.to_string())
        .map_err(anyhow::Error::msg)?;

    let mut context = tera::Context::new();

    for (ident, value) in module.iter_var() {
        context.insert(ident, &value);
    }

    Ok(context)
}

fn try_generate_card_slides(script: &str, template: &str) -> Result<CardSlides> {
    let context = context_from_script(script)?;

    let rendered = tera::Tera::one_off(template, &context, true)?;

    // TODO: optimize this absolute mess
    let mut grouped_events = vec![];
    let mut current_group = vec![];
    for event in Parser::new(&rendered) {
        if matches!(event, pulldown_cmark::Event::Rule) {
            grouped_events.push(current_group);
            current_group = vec![];
        } else {
            current_group.push(event);
        }
    }

    grouped_events.push(current_group);

    let slides = grouped_events
        .into_iter()
        .map(|events| {
            let mut html = String::new();
            pulldown_cmark::html::push_html(&mut html, events.into_iter());
            html
        })
        .collect();

    Ok(CardSlides(slides))
}

#[tauri::command]
pub fn generate_card_slides(script: String, template: String) -> CardSlides {
    try_generate_card_slides(&script, &template).unwrap_or_else(|err| {
        let msg = format!("<pre><code>Error: {err}</code></pre>");
        CardSlides(vec![msg])
    })
}

// -- pack

#[tauri::command]
pub async fn pack_create(window: Window, pool: State<'_, SqlitePool>, title: String) -> Result<()> {
    pack::create(pool.inner(), &title).await?;

    window.emit_all("update:packs", ()).expect("Event error");

    Ok(())
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

// -- card

#[tauri::command]
pub async fn card_create(
    window: Window,
    pool: State<'_, SqlitePool>,
    pack_id: pack::Id,
    label: String,
) -> Result<()> {
    const STANDARD_TEMPLATE: &str = "Question\n\n---\n\nAnswer";

    card::create(pool.inner(), pack_id, &label, "", STANDARD_TEMPLATE).await?;

    window.emit_all("update:cards", ()).expect("Event error");

    Ok(())
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

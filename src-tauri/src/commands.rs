use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::{Manager, State, Window};
use ts_rs::TS;

use crate::{card, markdown::Parser, pack, prelude::*, study};

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
    SetScript(Option<String>),
}

#[derive(TS, Deserialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub enum ModifyStudy {
    Rename(String),
    SetPack(Option<pack::Id>),
    SetLimit(usize),
    AddIncluded(String),
    RemoveIncluded(String),
    AddExcluded(String),
    RemoveExcluded(String),
    Delete,
}

#[derive(TS, Serialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Prompt {
    front: String,
    back: String,
}

#[derive(TS, Serialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct FullPrompt {
    card_id: card::Id,
    #[serde(flatten)]
    prompt: Prompt,
    tags: BTreeSet<String>,
}

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

fn try_generate_prompt(script: Option<&str>, front: &str, back: &str) -> Result<Prompt> {
    let context = script
        .map(context_from_script)
        .transpose()?
        .unwrap_or_default();

    let front = tera::Tera::one_off(front, &context, true)?;
    let back = tera::Tera::one_off(back, &context, true)?;

    Ok(Prompt { front, back })
}

#[tauri::command]
pub fn render_markdown(markdown: String) -> String {
    let parser = Parser::new(&markdown);

    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);

    html
}

#[tauri::command]
pub fn generate_prompt(script: Option<String>, front: String, back: String) -> Prompt {
    try_generate_prompt(script.as_deref(), &front, &back).unwrap_or_else(|err| {
        let msg = format!("```\n{err}\n```");

        Prompt {
            front: msg.clone(),
            back: msg,
        }
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
        r#"
        SELECT id as "id: u32", title
        FROM packs
        "#,
    )
    .fetch_all(pool.inner())
    .await?;

    Ok(packs)
}

#[tauri::command]
pub async fn pack_cards(pool: State<'_, SqlitePool>, id: pack::Id) -> Result<Vec<card::Card>> {
    let cards = sqlx::query_as!(
        card::Card,
        r#"
        SELECT id as "id: u32", label, script, front, back
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
    let front = format!("## {label} Question Side");
    let back = format!("## {label} Answer Side");
    card::create(pool.inner(), pack_id, &label, &front, &back).await?;

    window.emit_all("update:cards", ()).expect("Event error");

    Ok(())
}

#[tauri::command]
pub async fn card_query(
    pool: State<'_, SqlitePool>,
    pack_id: Option<pack::Id>,
    include: BTreeSet<String>,
    exclude: BTreeSet<String>,
    limit: Option<usize>,
) -> Result<Vec<FullPrompt>> {
    let tags_match =
        |tags: &BTreeSet<String>| tags.is_superset(&include) && tags.is_disjoint(&exclude);

    let prompts = card::random_by_tags(pool.inner(), pack_id, limit, tags_match)
        .await?
        .into_iter()
        .map(|(card, tags)| {
            let prompt = generate_prompt(card.script, card.front, card.back);

            FullPrompt {
                card_id: card.id,
                prompt,
                tags,
            }
        })
        .collect();

    Ok(prompts)
}

#[tauri::command]
pub async fn card_tags(pool: State<'_, SqlitePool>, id: card::Id) -> Result<Vec<String>> {
    card::list_tags(pool.inner(), id).await
}

#[tauri::command]
pub async fn card_modify(
    window: Window,
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
        ModifyCard::SetScript(script) => {
            card::set_script(pool.inner(), id, script.as_deref()).await
        }
    }?;

    window.emit_all("update:cards", ()).expect("Event error");

    Ok(())
}

// -- study

#[tauri::command]
pub async fn study_create(
    window: Window,
    pool: State<'_, SqlitePool>,
    title: String,
) -> Result<()> {
    study::create(pool.inner(), &title, None, 0).await?;

    window.emit_all("update:studies", ()).expect("Event error");

    Ok(())
}

#[tauri::command]
pub async fn study_list(pool: State<'_, SqlitePool>) -> Result<Vec<study::Study>> {
    let studies = sqlx::query!(
        r#"
        SELECT id, title, pack_id, question_count
        FROM study_queries
        "#,
    )
    .map(|row| study::Study {
        id: row.id as u32,
        title: row.title,
        pack_id: row.pack_id.map(|id| id as u32),
        limit: row.question_count as usize,
    })
    .fetch_all(pool.inner())
    .await?;

    Ok(studies)
}

#[tauri::command]
pub async fn study_tags(pool: State<'_, SqlitePool>, id: study::Id) -> Result<study::StudyTags> {
    study::list_tags(pool.inner(), id).await
}

#[tauri::command]
pub async fn study_modify(
    window: Window,
    pool: State<'_, SqlitePool>,
    id: study::Id,
    action: ModifyStudy,
) -> Result<()> {
    use ModifyStudy::*;

    let pool = pool.inner();

    match action {
        Rename(title) => study::rename(pool, id, &title).await,
        SetPack(pack_id) => study::set_pack(pool, id, pack_id).await,
        SetLimit(limit) => study::set_limit(pool, id, limit).await,
        AddIncluded(tag) => study::add_tag(pool, id, &tag, false).await,
        RemoveIncluded(tag) => study::remove_tag(pool, id, &tag, false).await,
        AddExcluded(tag) => study::add_tag(pool, id, &tag, true).await,
        RemoveExcluded(tag) => study::remove_tag(pool, id, &tag, true).await,
        Delete => study::delete(pool, id).await,
    }?;

    window.emit_all("update:studies", ()).expect("Event error");

    Ok(())
}

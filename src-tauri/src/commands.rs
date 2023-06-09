use std::collections::{BTreeMap, BTreeSet};

use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::State;
use ts_rs::TS;

use crate::{
    card::{self, Card, CardWithId},
    markdown::Parser,
    pack::{self, Pack},
    prelude::*,
    study::{self, Study, StudyTags},
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
pub async fn pack_create(pool: State<'_, SqlitePool>, title: String) -> Result<()> {
    pack::create(pool.inner(), &title).await?;
    Ok(())
}

#[tauri::command]
pub async fn pack_list(pool: State<'_, SqlitePool>) -> Result<BTreeMap<pack::Id, Pack>> {
    let mut rows = sqlx::query!(
        "
        SELECT id, title
        FROM packs
        ",
    )
    .fetch(pool.inner());

    let mut packs = BTreeMap::new();

    while let Some(row) = rows.try_next().await? {
        let pack = Pack { title: row.title };

        packs.insert(row.id as pack::Id, pack);
    }

    Ok(packs)
}

#[tauri::command]
pub async fn pack_cards(
    pool: State<'_, SqlitePool>,
    id: pack::Id,
) -> Result<BTreeMap<card::Id, Card>> {
    let mut rows = sqlx::query!(
        "
        SELECT id, label, script, front, back
        FROM cards
        WHERE pack_id = ?
        ",
        id,
    )
    .fetch(pool.inner());

    let mut cards = BTreeMap::new();

    while let Some(row) = rows.try_next().await? {
        let card = Card {
            label: row.label,
            script: row.script,
            front: row.front,
            back: row.back,
        };

        cards.insert(row.id as card::Id, card);
    }

    Ok(cards)
}

#[tauri::command]
pub async fn pack_modify(
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
pub async fn card_create(
    pool: State<'_, SqlitePool>,
    pack_id: pack::Id,
    label: String,
) -> Result<()> {
    let front = format!("## {label} Question Side");
    let back = format!("## {label} Answer Side");
    card::create(pool.inner(), pack_id, &label, &front, &back).await?;
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

    fn prompt_from_card((card, tags): (CardWithId, BTreeSet<String>)) -> FullPrompt {
        let prompt = generate_prompt(card.script, card.front, card.back);

        FullPrompt {
            card_id: card.id,
            prompt,
            tags,
        }
    }

    let prompts = card::random_by_tags(pool.inner(), pack_id, limit, tags_match)
        .await?
        .into_iter()
        .map(prompt_from_card)
        .collect();

    Ok(prompts)
}

#[tauri::command]
pub async fn card_tags(pool: State<'_, SqlitePool>, id: card::Id) -> Result<Vec<String>> {
    card::list_tags(pool.inner(), id).await
}

#[tauri::command]
pub async fn card_modify(
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
    }
}

// -- study

#[tauri::command]
pub async fn study_create(pool: State<'_, SqlitePool>, title: String) -> Result<()> {
    study::create(pool.inner(), &title, None, 0).await?;
    Ok(())
}

#[tauri::command]
pub async fn study_list(pool: State<'_, SqlitePool>) -> Result<BTreeMap<study::Id, Study>> {
    let mut rows = sqlx::query!(
        "
        SELECT id, title, pack_id, question_count
        FROM study_queries
        ",
    )
    .fetch(pool.inner());

    let mut studies = BTreeMap::new();

    while let Some(row) = rows.try_next().await? {
        let study = Study {
            title: row.title,
            pack_id: row.pack_id.map(|id| id as pack::Id),
            limit: row.question_count as usize,
        };

        studies.insert(row.id as study::Id, study);
    }

    Ok(studies)
}

#[tauri::command]
pub async fn study_tags(pool: State<'_, SqlitePool>, id: study::Id) -> Result<StudyTags> {
    study::list_tags(pool.inner(), id).await
}

#[tauri::command]
pub async fn study_modify(
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
    }
}

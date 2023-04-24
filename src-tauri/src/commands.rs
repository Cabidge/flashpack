use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::State;
use ts_rs::TS;

use crate::{
    card::{self, Card},
    markdown::Parser,
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
    SetScript(Option<String>),
}

#[derive(TS, Serialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Prompt {
    front: String,
    back: String,
}

struct ScriptResult {
    front: Option<String>,
    back: Option<String>,
}

fn run_script(script: &str, front: String, back: String) -> Result<ScriptResult> {
    let module = {
        let engine = crate::engine::create_engine();

        let mut scope = rhai::Scope::new();

        scope
            .push_constant("FRONT", front)
            .push_constant("BACK", back);

        let ast = engine
            .compile_with_scope(&scope, script)
            .map_err(anyhow::Error::from)?;

        rhai::Module::eval_ast_as_new(scope, &ast, &engine)
            .map_err(|err| err.to_string())
            .map_err(anyhow::Error::msg)?
    };

    let front = module.get_var_value::<String>("front");
    let back = module.get_var_value::<String>("back");

    Ok(ScriptResult { front, back })
}

#[tauri::command]
pub fn render_markdown(markdown: String) -> String {
    let parser = Parser::new(&markdown);

    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);

    html
}

#[tauri::command]
pub fn generate_prompt(script: String, front: String, back: String) -> Prompt {
    let (front, back) = {
        match run_script(&script, front, back) {
            Ok(ScriptResult { front, back }) => {
                let front = front.unwrap_or_else(|| String::from("`No front defined`"));
                let back = back.unwrap_or_else(|| String::from("`No back defined`"));

                (front, back)
            }
            Err(err) => {
                let msg = format!("```\n{err}\n```");
                (msg.clone(), msg)
            }
        }
    };

    Prompt { front, back }
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

    Ok(Pack { title, cards })
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
    label: String,
) -> Result<()> {
    let front = format!("## {label} Question Side");
    let back = format!("## {label} Answer Side");
    card::create(pool.inner(), pack_id, &label, &front, &back).await?;
    Ok(())
}

#[tauri::command]
pub async fn query_cards(
    pool: State<'_, SqlitePool>,
    pack_id: pack::Id,
    include: BTreeSet<String>,
    exclude: BTreeSet<String>,
    limit: Option<usize>,
) -> Result<Vec<card::Id>> {
    if include.is_empty() && exclude.is_empty() {
        card::random(pool.inner(), pack_id, limit).await
    } else {
        card::random_by_tags(pool.inner(), pack_id, limit, |tags: &BTreeSet<String>| {
            tags.is_superset(&include) && tags.is_disjoint(&exclude)
        })
        .await
    }
}

#[tauri::command]
pub async fn get_card(pool: State<'_, SqlitePool>, id: card::Id) -> Result<Card> {
    let details = card::get_details(pool.inner(), id).await?;
    let tags = card::list_tags(pool.inner(), id).await?;

    Ok(Card { details, tags })
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
        ModifyCard::SetScript(script) => {
            card::set_script(pool.inner(), id, script.as_deref()).await
        }
    }
}

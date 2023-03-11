#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod error;
mod fleeting;
mod prelude;

use std::sync::Mutex;

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

#[derive(TS, Serialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct QuizSummary {
    id: QuizQuery,
    name: String,
}

type SessionList = Vec<QuizSummary>;

#[derive(TS, Serialize, Debug, Clone)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Quiz {
    id: String,
    name: String,
    questions: Vec<Question>,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../src/bindings/")]
pub enum QuizQuery {
    Fleeting(u32),
    Concrete(String),
}

#[derive(TS, Serialize, Debug, Clone)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Question {
    question: String,
    answer: String,
    status: Completeness,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone, Copy)]
#[ts(export, export_to = "../src/bindings/")]
pub enum Completeness {
    Correct,
    Incorrect,
    Incomplete,
}

type Sessions = Mutex<fleeting::Sessions>;

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
            list_sessions,
            begin_fleeting,
            get_session,
            mark_question,
        ])
        .manage(pool)
        .manage(Sessions::default())
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

#[tauri::command]
fn list_sessions(sessions: State<'_, Sessions>) -> SessionList {
    sessions.lock().unwrap().summarize()
}

#[tauri::command]
async fn begin_fleeting(
    pool: State<'_, SqlitePool>,
    sessions: State<'_, Sessions>,
    id: i32,
) -> Result<u32> {
    let pack = get_pack(pool.clone(), id).await?;
    let cards = list_cards(pool, id).await?;

    let mut sessions = sessions.lock().unwrap();

    Ok(sessions.create(format!("{} session", pack.title), &cards))
}

#[tauri::command]
fn get_session(sessions: State<'_, Sessions>, id: QuizQuery) -> Result<Quiz> {
    match id {
        QuizQuery::Fleeting(id) => {
            let sessions = sessions.lock().unwrap();

            sessions
                .get(id)
                .cloned()
                .ok_or_else(|| Error::from(anyhow::anyhow!("Session not found")))
        }
        QuizQuery::Concrete(_id) => todo!("Does not exist...yet"),
    }
}

#[tauri::command]
fn mark_question(
    sessions: State<'_, Sessions>,
    id: QuizQuery,
    question_index: usize,
    status: Completeness,
) -> Result<()> {
    match id {
        QuizQuery::Fleeting(id) => {
            let mut sessions = sessions.lock().unwrap();

            let mut question = sessions
                .get_mut(id)
                .ok_or_else(|| Error::from(anyhow::anyhow!("Session not found")))?
                .questions
                .get_mut(question_index)
                .ok_or_else(|| Error::from(anyhow::anyhow!("Question not found")))?;

            question.status = status;

            Ok(())
        }
        QuizQuery::Concrete(_id) => todo!("Does not exist...yet"),
    }
}

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod db;
mod error;
mod fleeting;
mod prelude;

use std::sync::Mutex;

use crate::prelude::*;

use db::{Db, ParseObject};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Object, Thing, Value};
use tauri::State;
use ts_rs::TS;

#[derive(TS, Serialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Pack {
    id: String,
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
    id: String,
    title: String,
}

#[derive(TS, Serialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Card {
    id: String,
    front: String,
    back: String,
}

#[derive(TS, Deserialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct CardAdd {
    pack_id: String,
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

#[derive(TS, Serialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Quiz {
    id: String,
    name: String,
    questions: Vec<Question>,
}

#[derive(TS, Serialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub enum QuizQuery {
    Fleeting(u32),
    Concrete(String),
}

#[derive(TS, Serialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Question {
    question: String,
    answer: String,
    status: Completeness,
}

#[derive(TS, Serialize, Debug)]
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

impl ParseObject for Pack {
    fn parse_obj(mut value: Object) -> Result<Self> {
        let title = value
            .remove("title")
            .map(Value::as_string)
            .unwrap_or_else(|| String::from("untitled"));

        let Some(Value::Thing(th)) = value.get("id") else {
            bail!("Failed to get id.");
        };

        Ok(Pack {
            id: th.id.to_string(),
            title,
        })
    }
}

impl ParseObject for Card {
    fn parse_obj(mut value: Object) -> Result<Self> {
        let front = value
            .remove("front")
            .map(Value::as_string)
            .unwrap_or_else(String::new);

        let back = value
            .remove("back")
            .map(Value::as_string)
            .unwrap_or_else(String::new);

        let Some(Value::Thing(th)) = value.get("id") else {
            bail!("Failed to get id.");
        };

        Ok(Card {
            id: th.id.to_string(),
            front,
            back,
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let db = Db::new("memory", "flashpack", "db").await?;

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
        ])
        .manage(db)
        .manage(Sessions::default())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

#[tauri::command]
async fn list_packs(db: State<'_, Db>) -> Result<Vec<Pack>> {
    db.list(
        "
        SELECT id, title, created,
            string::lowercase(title) as lower_title
        FROM pack
        ORDER BY lower_title ASC, created ASC
        ",
        None,
    )
    .await
}

#[tauri::command]
async fn create_pack(db: State<'_, Db>, pack: PackCreate) -> Result<()> {
    let vars = vars! {
        "title" => pack.title,
    };

    db.execute(
        "CREATE pack SET title = $title, created = time::now()",
        Some(vars),
    )
    .await?;

    Ok(())
}

#[tauri::command]
async fn get_pack(db: State<'_, Db>, id: String) -> Result<Pack> {
    let th = Thing {
        id: id.into(),
        tb: String::from("pack"),
    };

    let vars = vars! {
        "th" => th,
    };

    db.get("SELECT id, title FROM $th", Some(vars)).await
}

#[tauri::command]
async fn delete_pack(db: State<'_, Db>, id: String) -> Result<()> {
    let th = Thing {
        id: id.into(),
        tb: String::from("pack"),
    };

    let vars = vars! {
        "th" => th,
    };

    db.execute("DELETE $th", Some(vars)).await?;

    Ok(())
}

#[tauri::command]
async fn update_pack(db: State<'_, Db>, update: PackUpdate) -> Result<()> {
    let th = Thing {
        id: update.id.into(),
        tb: String::from("pack"),
    };

    let vars = vars! {
        "th" => th,
        "title" => update.title,
    };

    db.execute("UPDATE $th SET title = $title", Some(vars))
        .await?;

    Ok(())
}

#[tauri::command]
async fn list_cards(db: State<'_, Db>, id: String) -> Result<Vec<Card>> {
    let th = Thing {
        id: id.into(),
        tb: String::from("pack"),
    };

    let vars = vars! {
        "pack" => th,
    };

    db.list(
        "
        SELECT id, front, back, created
        FROM card
        WHERE pack = $pack
        ORDER BY created ASC
        ",
        Some(vars),
    )
    .await
}

#[tauri::command]
async fn add_card(db: State<'_, Db>, card: CardAdd) -> Result<()> {
    let pack_th = Thing {
        id: card.pack_id.into(),
        tb: String::from("pack"),
    };

    let vars = vars! {
        "front" => card.front,
        "back" => card.back,
        "pack" => pack_th,
    };

    db.execute(
        "
        CREATE card
        SET front = $front,
            back = $back,
            pack = $pack,
            created = time::now()
        ",
        Some(vars),
    )
    .await?;

    Ok(())
}

#[tauri::command]
fn list_sessions(sessions: State<'_, Sessions>) -> SessionList {
    sessions.lock().unwrap().summarize()
}

#[tauri::command]
async fn begin_fleeting(
    db: State<'_, Db>,
    sessions: State<'_, Sessions>,
    id: String,
) -> Result<u32> {
    let pack = get_pack(db.clone(), id.clone()).await?;
    let cards = list_cards(db.clone(), id.clone()).await?;

    let mut sessions = sessions.lock().unwrap();

    Ok(sessions.create(format!("{} session", pack.title), &cards))
}

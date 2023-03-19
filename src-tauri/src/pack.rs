use serde::Serialize;
use sqlx::{FromRow, SqlitePool};
use ts_rs::TS;

use crate::{card, prelude::*};

#[derive(FromRow, TS, Serialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Summary {
    pub id: Id,
    pub title: String,
}

#[derive(TS, Serialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Pack {
    pub title: String,
    pub cards: Vec<card::Summary>,
}

pub type Id = i64;

pub async fn create(pool: &SqlitePool, title: &str) -> Result<Id> {
    struct InsertResult {
        id: Id,
    }

    let row = sqlx::query_as!(
        InsertResult,
        "
        INSERT INTO packs (title)
        VALUES (?)
        RETURNING id
        ",
        title,
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}


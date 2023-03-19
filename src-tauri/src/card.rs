use serde::Serialize;
use sqlx::{FromRow, SqlitePool};
use ts_rs::TS;

use crate::prelude::*;

#[derive(FromRow, TS, Serialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Summary {
    pub id: Id,
    pub front: String,
    pub back: String,
}

type Id = i64;

pub async fn add_to_pack(
    pool: &SqlitePool,
    pack_id: crate::pack::Id,
    front: &str,
    back: &str,
) -> Result<Id> {
    #[derive(FromRow)]
    struct InsertResult {
        id: Id,
    }

    let row = sqlx::query_as!(
        InsertResult,
        "
        INSERT INTO cards (front, back, pack_id)
        VALUES (?, ?, ?)
        RETURNING id
        ",
        front,
        back,
        pack_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}

pub async fn add_tag(pool: &SqlitePool, card_id: Id, tag: &str) -> Result<()> {
    sqlx::query!(
        "
        INSERT INTO card_tags (card_id, tag)
        VALUES (?, ?)
        ",
        card_id,
        tag,
    )
    .execute(pool)
    .await?;

    Ok(())
}

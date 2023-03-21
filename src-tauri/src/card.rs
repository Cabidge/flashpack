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

pub type Id = u32;

pub async fn list_by_pack(pool: &SqlitePool, pack_id: crate::pack::Id) -> Result<Vec<Summary>> {
    sqlx::query_as!(
        Summary,
        r#"
        SELECT id as "id: Id", front, back
        FROM cards
        WHERE pack_id = ?
        "#,
        pack_id,
    )
    .fetch_all(pool)
    .await
    .map_err(Error::from)
}

pub async fn create(
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
        r#"
        INSERT INTO cards (front, back, pack_id)
        VALUES (?, ?, ?)
        RETURNING id as "id: Id"
        "#,
        front,
        back,
        pack_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}

pub async fn add_tag(pool: &SqlitePool, id: Id, tag: &str) -> Result<()> {
    sqlx::query!(
        "
        INSERT INTO card_tags (card_id, tag)
        VALUES (?, ?)
        ",
        id,
        tag,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn remove_tag(pool: &SqlitePool, id: Id, tag: &str) -> Result<()> {
    sqlx::query!(
        "
        DELETE FROM card_tags
        WHERE card_id = ?
        AND tag = ?
        ",
        id,
        tag,
    )
    .execute(pool)
    .await?;

    Ok(())
}

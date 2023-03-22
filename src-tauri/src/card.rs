use serde::Serialize;
use sqlx::SqlitePool;
use ts_rs::TS;

use crate::prelude::*;

#[derive(TS, Serialize, Debug)]
#[ts(rename = "CardSummary", export, export_to = "../src/bindings/")]
pub struct Summary {
    pub id: Id,
    pub label: String,
}

#[derive(TS, Serialize, Debug)]
pub struct Details {
    pub front: String,
    pub back: String,
}

#[derive(TS, Serialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Card {
    #[serde(flatten)]
    pub details: Details,
    pub tags: Vec<String>,
}

pub type Id = u32;

pub async fn create(
    pool: &SqlitePool,
    pack_id: crate::pack::Id,
    front: &str,
    back: &str,
) -> Result<Id> {
    let row = sqlx::query!(
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

pub async fn list_by_pack(pool: &SqlitePool, pack_id: crate::pack::Id) -> Result<Vec<Summary>> {
    sqlx::query_as!(
        Summary,
        r#"
        SELECT id as "id: Id", front as label
        FROM cards
        WHERE pack_id = ?
        "#,
        pack_id,
    )
    .fetch_all(pool)
    .await
    .map_err(Error::from)
}

pub async fn get_details(pool: &SqlitePool, id: Id) -> Result<Details> {
    sqlx::query_as!(
        Details,
        "
        SELECT front, back
        FROM cards
        WHERE id = ?
        ",
        id,
    )
    .fetch_one(pool)
    .await
    .map_err(Error::from)
}

pub async fn list_tags(pool: &SqlitePool, id: Id) -> Result<Vec<String>> {
    sqlx::query!(
        "
        SELECT tag
        FROM card_tags
        WHERE card_id = ?
        ",
        id,
    )
    .map(|row| row.tag)
    .fetch_all(pool)
    .await
    .map_err(Error::from)
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

pub async fn rename(
    pool: &SqlitePool,
    id: Id,
    front: Option<&str>,
    back: Option<&str>,
) -> Result<()> {
    sqlx::query!(
        "
        UPDATE cards
        SET front = IFNULL(?, front),
            back = IFNULL(?, back)
        WHERE id = ?
        ",
        front,
        back,
        id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

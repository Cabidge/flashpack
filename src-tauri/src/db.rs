use crate::prelude::*;

use sqlx::{SqlitePool, Row};

pub type PackId = i32;
pub type CardId = i32;
pub type TagId = i32;

pub async fn create_pack(pool: &SqlitePool, title: &str) -> Result<PackId> {
    let row = sqlx::query(
        "
        INSERT INTO packs (title)
        VALUES (?)
        RETURNING id
        "
    )
    .bind(title)
    .fetch_one(pool)
    .await?;

    let id = row.try_get::<i32, _>("id")?;

    Ok(id)
}

pub async fn add_card(pool: &SqlitePool, pack_id: PackId, front: &str, back: &str) -> Result<CardId> {
    let row = sqlx::query(
        "
        INSERT INTO cards (front, back, pack_id)
        VALUES (?, ?, ?)
        RETURNING id
        ",
    )
    .bind(front)
    .bind(back)
    .bind(pack_id)
    .fetch_one(pool)
    .await?;

    let id = row.try_get::<i32, _>("id")?;

    Ok(id)
}

pub async fn create_tag(pool: &SqlitePool, label: &str) -> Result<TagId> {
    let row = sqlx::query(
        "
        INSERT INTO tags (label)
        VALUES (?)
        RETURNING id
        ",
    )
    .bind(label)
    .fetch_one(pool)
    .await?;

    let id = row.try_get::<i32, _>("id")?;

    Ok(id)
}

pub async fn add_tag(pool: &SqlitePool, card_id: CardId, tag_id: TagId) -> Result<()> {
    sqlx::query(
        "
        INSERT INTO card_tags (card_id, tag_id)
        VALUES (?, ?)
        ",
    )
    .bind(card_id)
    .bind(tag_id)
    .execute(pool)
    .await?;

    Ok(())
}

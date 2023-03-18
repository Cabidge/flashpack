use crate::prelude::*;

use sqlx::{FromRow, SqlitePool};

pub type PackId = i64;
pub type CardId = i64;

#[derive(FromRow)]
struct Id {
    id: i64,
}

pub async fn create_pack(pool: &SqlitePool, title: &str) -> Result<PackId> {
    let row = sqlx::query_as!(
        Id,
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

pub async fn add_card(
    pool: &SqlitePool,
    pack_id: PackId,
    front: &str,
    back: &str,
) -> Result<CardId> {
    let row = sqlx::query_as!(
        Id,
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

pub async fn add_tag(pool: &SqlitePool, card_id: CardId, tag: &str) -> Result<()> {
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

use std::collections::BTreeSet;

use futures::TryStreamExt;
use serde::Serialize;
use sqlx::SqlitePool;
use ts_rs::TS;

use crate::{prelude::*, pack};

#[derive(TS, Serialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Card {
    pub label: String,
    pub script: Option<String>,
    pub front: String,
    pub back: String,
}

pub struct CardWithId {
    pub id: Id,
    pub label: String,
    pub script: Option<String>,
    pub front: String,
    pub back: String,
}

pub type Id = u32;

pub async fn create(
    pool: &SqlitePool,
    pack_id: crate::pack::Id,
    label: &str,
    front: &str,
    back: &str,
) -> Result<Id> {
    let row = sqlx::query!(
        r#"
        INSERT INTO cards (label, front, back, pack_id)
        VALUES (?, ?, ?, ?)
        RETURNING id as "id: Id"
        "#,
        label,
        front,
        back,
        pack_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}

pub async fn random_by_tags(
    pool: &SqlitePool,
    pack_id: Option<pack::Id>,
    limit: Option<usize>,
    mut predicate: impl FnMut(&BTreeSet<String>) -> bool,
) -> Result<Vec<(CardWithId, BTreeSet<String>)>> {
    let mut queried = vec![];

    let mut cards = sqlx::query_as!(
        CardWithId,
        r#"
        SELECT id as "id: Id",
            label,
            script,
            front,
            back
        FROM cards
        WHERE ?1 IS NULL
        OR pack_id = ?1
        ORDER BY RANDOM()
        "#,
        pack_id,
    )
    .fetch(pool);

    while let Some(card) = cards.try_next().await? {
        if Some(queried.len()) == limit {
            break;
        }

        let tags: BTreeSet<String> = sqlx::query!(
            "
            SELECT tag
            FROM card_tags
            WHERE card_id = ?
            ",
            card.id,
        )
        .fetch(pool)
        .map_ok(|row| row.tag)
        .try_collect()
        .await?;

        if predicate(&tags) {
            queried.push((card, tags));
        }
    }

    Ok(queried)
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

pub async fn set_script(pool: &SqlitePool, id: Id, script: Option<&str>) -> Result<()> {
    sqlx::query!(
        "
        UPDATE cards
        SET script = ?
        WHERE id = ?
        ",
        script,
        id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

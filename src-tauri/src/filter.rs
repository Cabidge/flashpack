use std::collections::BTreeSet;

use futures::{StreamExt, TryStreamExt};
use sqlx::{FromRow, SqlitePool};

use crate::{prelude::*, card};

pub struct Summary {
    id: Id,
    label: String,
}

pub struct Filter {
    label: String,
    included_tags: Vec<String>,
    excluded_tags: Vec<String>,
}

#[derive(FromRow)]
pub struct Prompt {
    pub question: String,
    pub answer: String,
}

pub type Id = i64;

pub async fn create(
    pool: &SqlitePool,
    pack_id: crate::pack::Id,
    label: &str,
) -> Result<Id> {
    #[derive(FromRow)]
    struct InsertResult {
        id: Id,
    }

    let row = sqlx::query_as!(
        InsertResult,
        "
        INSERT INTO filters (label, pack_id)
        VALUES (?, ?)
        RETURNING id
        ",
        label,
        pack_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}

pub async fn add_included(
    pool: &SqlitePool,
    filter_id: Id,
    tag: &str,
) -> Result<()> {
    sqlx::query!(
        "
        INSERT INTO included_tags (filter_id, tag)
        VALUES (?, ?)
        ",
        filter_id,
        tag,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn add_excluded(
    pool: &SqlitePool,
    filter_id: Id,
    tag: &str,
) -> Result<()> {
    sqlx::query!(
        "
        INSERT INTO excluded_tags (filter_id, tag)
        VALUES (?, ?)
        ",
        filter_id,
        tag,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn select_card(
    pool: &SqlitePool,
    filter_id: Id,
) -> Result<Option<card::Id>> {
    #[derive(FromRow)]
    struct TagQueryResult {
        tag: String,
    }

    let included: BTreeSet<String> = sqlx::query_as!(
        TagQueryResult,
        r#"
        SELECT tag
        FROM included_tags
        WHERE filter_id = ?
        "#,
        filter_id,
    )
    .fetch(pool)
    .map_ok(|row| row.tag)
    .try_collect()
    .await?;

    let excluded: BTreeSet<String> = sqlx::query_as!(
        TagQueryResult,
        r#"
        SELECT tag
        FROM excluded_tags
        WHERE filter_id = ?
        "#,
        filter_id,
    )
    .fetch(pool)
    .map_ok(|row| row.tag)
    .try_collect()
    .await?;

    #[derive(FromRow)]
    struct CardQueryResult {
        id: i64,
    }

    let mut cards = sqlx::query_as!(
        CardQueryResult,
        "
        SELECT c.id
        FROM cards c, filters f
        WHERE c.pack_id = f.pack_id
        AND f.id = ?
        ORDER BY RANDOM()
        ",
        filter_id,
    )
    .fetch(pool);

    while let Some(CardQueryResult { id }) = cards.try_next().await? {
        let tags: BTreeSet<String> = sqlx::query_as!(
            TagQueryResult,
            "
            SELECT tag
            FROM card_tags
            WHERE card_id = ?
            ",
            id,
        )
        .fetch(pool)
        .map_ok(|row| row.tag)
        .try_collect()
        .await?;

        if included.is_subset(&tags) && excluded.is_disjoint(&tags) {
            return Ok(Some(id));
        }
    }

    Ok(None)
}

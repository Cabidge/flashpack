use std::collections::BTreeSet;

use futures::TryStreamExt;
use sqlx::{FromRow, SqlitePool};

use crate::{card, prelude::*};

pub struct Summary {
    id: Id,
    label: String,
}

pub struct Filter {
    label: String,
    tags: Vec<Tag>,
}

pub struct Tag {
    tag: String,
    exclude: bool,
}

#[derive(FromRow)]
pub struct Prompt {
    pub question: String,
    pub answer: String,
}

pub type Id = u32;

pub async fn create(pool: &SqlitePool, pack_id: crate::pack::Id, label: &str) -> Result<Id> {
    #[derive(FromRow)]
    struct InsertResult {
        id: Id,
    }

    let row = sqlx::query_as!(
        InsertResult,
        r#"
        INSERT INTO filters (label, pack_id)
        VALUES (?, ?)
        RETURNING id as "id: Id"
        "#,
        label,
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
        SELECT id as "id: Id", label
        FROM filters
        WHERE pack_id = ?
        "#,
        pack_id,
    )
    .fetch_all(pool)
    .await
    .map_err(Error::from)
}

pub async fn add_tag(pool: &SqlitePool, filter_id: Id, tag: &str) -> Result<()> {
    sqlx::query!(
        "
        INSERT INTO filter_tags (filter_id, tag)
        VALUES (?, ?)
        ",
        filter_id,
        tag,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn set_excluded(
    pool: &SqlitePool,
    filter_id: Id,
    tag: &str,
    exclude: bool,
) -> Result<()> {
    sqlx::query!(
        "
        UPDATE filter_tags
        SET exclude = ?
        WHERE filter_id = ?
        AND tag = ?
        ",
        filter_id,
        tag,
        exclude,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn select_card(pool: &SqlitePool, filter_id: Id) -> Result<Option<card::Id>> {
    #[derive(FromRow)]
    struct FilterTagRow {
        tag: String,
        exclude: bool,
    }

    let mut rows = sqlx::query_as!(
        FilterTagRow,
        "
        SELECT tag, exclude
        FROM filter_tags
        WHERE filter_id = ?
        ",
        filter_id,
    )
    .fetch(pool);

    let mut included = BTreeSet::new();
    let mut excluded = BTreeSet::new();
    while let Some(row) = rows.try_next().await? {
        if row.exclude {
            excluded.insert(row.tag);
        } else {
            included.insert(row.tag);
        }
    }

    #[derive(FromRow)]
    struct CardIdRow {
        id: card::Id,
    }

    let mut cards = sqlx::query_as!(
        CardIdRow,
        r#"
        SELECT c.id as "id: card::Id"
        FROM cards c, filters f
        WHERE c.pack_id = f.pack_id
        AND f.id = ?
        ORDER BY RANDOM()
        "#,
        filter_id,
    )
    .fetch(pool);

    while let Some(CardIdRow { id }) = cards.try_next().await? {
        #[derive(FromRow)]
        struct CardTagRow {
            tag: String,
        }

        let tags: BTreeSet<String> = sqlx::query_as!(
            CardTagRow,
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

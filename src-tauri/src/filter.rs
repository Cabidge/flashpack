use sqlx::{FromRow, SqlitePool};

use crate::prelude::*;

pub struct Summary {
    id: Id,
    label: String,
}

pub struct Filter {
    label: String,
    included_tags: Vec<String>,
    excluded_tags: Vec<String>,
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

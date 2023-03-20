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
    struct QueryResult {
        id: card::Id,
    }

    let row = sqlx::query_as!(
        QueryResult,
        r#"
        SELECT c.id as "id!"
        FROM cards c, filters f
        WHERE c.pack_id = f.pack_id
        AND f.id = ?
        AND c.id IN (
            WITH included AS (
                SELECT it.tag
                FROM included_tags it
                WHERE it.filter_id = f.id
            )
            SELECT ct.card_id
            FROM card_tags ct
            WHERE ct.tag IN included
            GROUP BY ct.card_id
            HAVING COUNT(*) = (SELECT COUNT(*) FROM included)
        )
        AND c.id IN (
            SELECT ct.card_id
            FROM card_tags ct
            WHERE ct.tag IN (
                SELECT et.tag
                FROM excluded_tags et
                WHERE et.filter_id = f.id
            )
            GROUP BY ct.card_id
        )
        ORDER BY RANDOM()
        LIMIT 1
        "#,
        filter_id,
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|card| card.id))
}

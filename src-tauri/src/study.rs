use futures::TryStreamExt;
use serde::Serialize;
use sqlx::SqlitePool;
use ts_rs::TS;

use crate::{pack, prelude::*};

#[derive(TS, Serialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Study {
    pub id: Id,
    pub title: String,
    pub pack_id: Option<pack::Id>,
    pub limit: usize,
}

#[derive(TS, Serialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct StudyTags {
    included: Vec<String>,
    excluded: Vec<String>,
}

pub type Id = u32;

pub async fn create(
    pool: &SqlitePool,
    title: &str,
    pack_id: Option<pack::Id>,
    limit: usize,
) -> Result<Id> {
    let limit = limit as u32;

    let id = sqlx::query!(
        r#"
        INSERT INTO study_queries (title, pack_id, question_count)
        VALUES (?, ?, ?)
        RETURNING id as "id: Id"
        "#,
        title,
        pack_id,
        limit,
    )
    .fetch_one(pool)
    .await?
    .id;

    Ok(id)
}

pub async fn list_all(pool: &SqlitePool) -> Result<Vec<Study>> {
    sqlx::query!(
        r#"
        SELECT id,
            title,
            pack_id,
            question_count
        FROM study_queries
        ORDER BY LOWER(title) ASC, id ASC
        "#,
    )
    .map(|row| Study {
        id: row.id as u32,
        title: row.title,
        pack_id: row.pack_id.map(|id| id as pack::Id),
        limit: row.question_count as usize,
    })
    .fetch_all(pool)
    .await
    .map_err(Error::from)
}

pub async fn list_tags(pool: &SqlitePool, id: Id) -> Result<StudyTags> {
    let mut rows = sqlx::query!(
        r#"
        SELECT tag,
            exclude
        FROM study_query_tags
        WHERE study_id = ?
        "#,
        id,
    )
    .fetch(pool);

    let mut included = vec![];
    let mut excluded = vec![];

    while let Some(row) = rows.try_next().await? {
        if row.exclude {
            excluded.push(row.tag);
        } else {
            included.push(row.tag);
        }
    }

    Ok(StudyTags { included, excluded })
}

pub async fn rename(pool: &SqlitePool, id: Id, title: &str) -> Result<()> {
    sqlx::query!(
        "
        UPDATE study_queries
        SET title = ?
        WHERE id = ?
        ",
        title,
        id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn set_pack(pool: &SqlitePool, id: Id, pack_id: Option<pack::Id>) -> Result<()> {
    sqlx::query!(
        "
        UPDATE study_queries
        SET pack_id = ?
        WHERE id = ?
        ",
        pack_id,
        id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn set_limit(pool: &SqlitePool, id: Id, limit: usize) -> Result<()> {
    let limit = limit as u32;

    sqlx::query!(
        "
        UPDATE study_queries
        SET question_count = ?
        WHERE id = ?
        ",
        limit,
        id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn add_tag(pool: &SqlitePool, id: Id, tag: &str, exclude: bool) -> Result<()> {
    sqlx::query!(
        "
        INSERT INTO study_query_tags (study_id, tag, exclude)
        VALUES (?, ?, ?)
        ",
        id,
        tag,
        exclude,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn remove_tag(pool: &SqlitePool, id: Id, tag: &str, exclude: bool) -> Result<()> {
    sqlx::query!(
        "
        DELETE FROM study_query_tags
        WHERE study_id = ?
        AND tag = ?
        AND exclude = ?
        ",
        id,
        tag,
        exclude,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete(pool: &SqlitePool, id: Id) -> Result<()> {
    sqlx::query!(
        "
        DELETE FROM study_queries
        WHERE id = ?
        ",
        id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

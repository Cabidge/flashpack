use std::borrow::Cow;

use futures::{TryStreamExt, StreamExt};
use serde::Serialize;
use sqlx::SqlitePool;
use ts_rs::TS;

use crate::{card, filter, prelude::*};

#[derive(TS, Serialize, Debug)]
#[ts(rename = "PackSummary", export, export_to = "../src/bindings/")]
pub struct Summary {
    pub id: Id,
    pub title: String,
}

#[derive(TS, Serialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Pack {
    pub title: String,
    pub cards: Vec<card::Summary>,
    pub filters: Vec<filter::Summary>,
}

pub type Id = u32;

async fn find_unique_title<'a>(pool: &SqlitePool, desired_title: &'a str) -> Result<Cow<'a, str>> {
    let mut unique_title = Cow::Borrowed(desired_title);
    let mut unique_lower = desired_title.to_lowercase();

    let mut titles = sqlx::query!(
        "
        SELECT title
        FROM packs
        ORDER BY LOWER(title) ASC
        "
    )
    .map(|row| row.title.to_lowercase())
    .fetch(pool);

    let mut i = 0;
    while let Some(title) = titles.try_next().await? {
        if title > unique_lower {
            break;
        }

        if title == unique_lower {
            i += 1;
            unique_title = Cow::Owned(format!("{} ({})", desired_title, i));
            unique_lower = unique_title.to_lowercase();
        }
    }

    Ok(unique_title)
}

pub async fn create(pool: &SqlitePool, title: &str) -> Result<Id> {
    struct InsertResult {
        id: Id,
    }

    let row = sqlx::query_as!(
        InsertResult,
        r#"
        INSERT INTO packs (title)
        VALUES (?)
        RETURNING id as "id: Id"
        "#,
        title,
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}

pub async fn list_all(pool: &SqlitePool) -> Result<Vec<Summary>> {
    sqlx::query_as!(
        Summary,
        r#"
        SELECT id as "id: Id", title
        FROM packs
        ORDER BY LOWER(title) ASC, id ASC
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(Error::from)
}

pub async fn get_title(pool: &SqlitePool, id: Id) -> Result<String> {
    let row = sqlx::query!(
        r#"
        SELECT title
        FROM packs
        WHERE id = ?
        "#,
        id,
    )
    .fetch_one(pool)
    .await?;

    Ok(row.title)
}

pub async fn delete(pool: &SqlitePool, id: Id) -> Result<()> {
    sqlx::query!(
        "
        DELETE FROM packs
        WHERE id = ?
        ",
        id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn rename(pool: &SqlitePool, id: Id, new_title: &str) -> Result<()> {
    sqlx::query!(
        "
        UPDATE packs
        SET title = ?
        WHERE id = ?
        ",
        new_title,
        id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

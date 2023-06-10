use std::borrow::Cow;

use futures::TryStreamExt;
use serde::Serialize;
use sqlx::SqlitePool;
use ts_rs::TS;

use crate::prelude::*;

#[derive(TS, Serialize, Debug)]
#[ts(rename = "Pack", export, export_to = "../src/bindings/")]
pub struct Pack {
    pub id: Id,
    pub title: String,
}

pub type Id = u32;

async fn find_unique_title<'a>(
    pool: &SqlitePool,
    desired_title: &'a str,
    skip_id: Option<Id>,
) -> Result<Cow<'a, str>> {
    let mut unique_title = Cow::Borrowed(desired_title);
    let mut unique_lower = desired_title.to_lowercase();

    let skip_id = skip_id.map(i64::from).unwrap_or(-1);
    let mut titles = sqlx::query!(
        "
        SELECT title
        FROM packs
        WHERE id <> ?
        ORDER BY LOWER(title) ASC
        ",
        skip_id,
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
    let title = find_unique_title(pool, title, None).await?;

    let row = sqlx::query!(
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
    let new_title = find_unique_title(pool, new_title, Some(id)).await?;

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

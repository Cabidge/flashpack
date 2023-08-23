use serde::Serialize;
use sqlx::SqlitePool;
use ts_rs::TS;

use crate::prelude::*;

#[derive(TS, Serialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Pack {
    pub id: Id,
    pub title: String,
}

pub type Id = i64;

pub async fn create(pool: &SqlitePool, title: &str) -> Result<Id> {
    let row = sqlx::query!(
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

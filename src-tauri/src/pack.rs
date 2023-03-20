use serde::Serialize;
use sqlx::{FromRow, SqlitePool};
use ts_rs::TS;

use crate::{card, prelude::*};

#[derive(FromRow, TS, Serialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Summary {
    pub id: Id,
    pub title: String,
}

#[derive(TS, Serialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Pack {
    pub title: String,
    pub cards: Vec<card::Summary>,
}

pub type Id = i64;

pub async fn create(pool: &SqlitePool, title: &str) -> Result<Id> {
    struct InsertResult {
        id: Id,
    }

    let row = sqlx::query_as!(
        InsertResult,
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

pub async fn list_all(pool: &SqlitePool) -> Result<Vec<Summary>> {
    sqlx::query_as!(
        Summary,
        "
        SELECT *
        FROM packs
        ORDER BY LOWER(title) ASC, id ASC
        ",
    )
    .fetch_all(pool)
    .await
    .map_err(Error::from)
}

pub async fn with_id(pool: &SqlitePool, id: Id) -> Result<Summary> {
    sqlx::query_as!(
        Summary,
        "
        SELECT *
        FROM packs
        WHERE id = ?
        ",
        id,
    )
    .fetch_one(pool)
    .await
    .map_err(Error::from)
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

use serde::Serialize;
use sqlx::SqlitePool;
use ts_rs::TS;

use crate::prelude::*;

#[derive(TS, Serialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Card {
    pub id: Id,
    pub label: String,
    pub script: String,
    pub template: String,
    pub pack_id: crate::pack::Id,
}

pub type Id = i64;

pub async fn create(
    pool: &SqlitePool,
    pack_id: crate::pack::Id,
    label: &str,
    script: &str,
    template: &str,
) -> Result<Id> {
    let row = sqlx::query!(
        "
        INSERT INTO cards (label, script, template, pack_id)
        VALUES (?, ?, ?, ?)
        RETURNING id
        ",
        label,
        script,
        template,
        pack_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}

pub async fn edit(
    pool: &SqlitePool,
    id: Id,
    label: Option<&str>,
    script: Option<&str>,
    template: Option<&str>,
) -> Result<()> {
    sqlx::query!(
        "
        UPDATE cards
        SET label = IFNULL(?, label),
            script = IFNULL(?, script),
            template = IFNULL(?, template)
        WHERE id = ?
        ",
        label,
        script,
        template,
        id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete(pool: &SqlitePool, id: Id) -> Result<()> {
    sqlx::query!(
        "
        DELETE FROM cards
        WHERE id = ?
        ",
        id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

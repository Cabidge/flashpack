use rand::seq::SliceRandom;
use serde::Serialize;
use sqlx::SqlitePool;
use ts_rs::TS;

use crate::{filter, prelude::*};

#[derive(TS, Serialize, Debug)]
#[ts(rename = "DealerSummary", export, export_to = "../src/bindings/")]
pub struct Summary {
    id: Id,
    title: String,
}

#[derive(TS, Serialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Dealer {
    pub title: String,
    pub filters: Vec<DealerFilter>,
}

#[derive(TS, Serialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct DealerFilter {
    id: filter::Id,
    label: String,
    pack_title: String,
    weight: u32,
}

pub type Id = u32;

pub async fn create(pool: &SqlitePool, title: &str) -> Result<Id> {
    let row = sqlx::query!(
        r#"
        INSERT INTO dealers (title)
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
        FROM dealers
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(Error::from)
}

pub async fn get_title(pool: &SqlitePool, id: Id) -> Result<String> {
    let row = sqlx::query!(
        "
        SELECT title
        FROM dealers
        WHERE id = ?
        ",
        id,
    )
    .fetch_one(pool)
    .await?;

    Ok(row.title)
}

pub async fn list_filters(pool: &SqlitePool, id: Id) -> Result<Vec<DealerFilter>> {
    sqlx::query_as!(
        DealerFilter,
        r#"
        SELECT f.id as "id: filter::Id",
            f.label,
            p.title as pack_title,
            df.strength as "weight: u32"
        FROM dealer_filters df, filters f, packs p
        WHERE df.filter_id = f.id
        AND f.pack_id = p.id
        AND df.dealer_id = ?
        "#,
        id,
    )
    .fetch_all(pool)
    .await
    .map_err(Error::from)
}

pub async fn add_filter(
    pool: &SqlitePool,
    dealer_id: Id,
    filter_id: filter::Id,
    weight: i32,
) -> Result<()> {
    sqlx::query!(
        "
        INSERT INTO dealer_filters (dealer_id, filter_id, strength)
        VALUES (?, ?, ?)
        ",
        dealer_id,
        filter_id,
        weight,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn remove_filter(pool: &SqlitePool, dealer_id: Id, filter_id: filter::Id) -> Result<()> {
    sqlx::query!(
        "
        DELETE FROM dealer_filters
        WHERE dealer_id = ?
        AND filter_id = ?
        ",
        dealer_id,
        filter_id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn next_filter(pool: &SqlitePool, dealer_id: Id) -> Result<Option<filter::Id>> {
    let id = sqlx::query!(
        r#"
        SELECT filter_id as "id: filter::Id", strength as weight
        FROM dealer_filters
        WHERE dealer_id = ?
        "#,
        dealer_id,
    )
    .fetch_all(pool)
    .await?
    .choose_weighted(&mut rand::thread_rng(), |row| row.weight)
    .ok()
    .map(|row| row.id);

    Ok(id)
}

use rand::seq::SliceRandom;
use sqlx::{FromRow, SqlitePool};

use crate::{filter, prelude::*};

pub struct Summary {
    id: Id,
    title: String,
}

pub struct Dealer {
    title: String,
    filters: Vec<DealerFilter>,
}

pub struct DealerFilter {
    summary: filter::Summary,
    pack_title: String,
    weight: i32,
}

pub type Id = u32;

pub async fn create(pool: &SqlitePool, title: &str) -> Result<Id> {
    #[derive(FromRow)]
    struct InsertResult {
        id: Id,
    }

    let row = sqlx::query_as!(
        InsertResult,
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

pub async fn select_filter(pool: &SqlitePool, dealer_id: Id) -> Result<Option<filter::Id>> {
    #[derive(FromRow)]
    struct QueryResult {
        id: filter::Id,
        weight: i64,
    }

    let id = sqlx::query_as!(
        QueryResult,
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

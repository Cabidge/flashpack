use sqlx::{SqlitePool, FromRow};

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

pub type Id = i64;

pub async fn create(
    pool: &SqlitePool,
    title: &str,
) -> Result<Id> {
    #[derive(FromRow)]
    struct InsertResult {
        id: Id,
    }

    let row = sqlx::query_as!(
        InsertResult,
        "
        INSERT INTO dealers (title)
        VALUES (?)
        RETURNING id
        ",
        title,
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}

pub async fn add_filter(pool: &SqlitePool, dealer_id: Id, filter_id: filter::Id, weight: i32) -> Result<()> {
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

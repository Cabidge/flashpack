use rand::{distributions::WeightedIndex, prelude::Distribution};
use serde::Serialize;
use sqlx::FromRow;
use ts_rs::TS;

use crate::prelude::*;

#[derive(FromRow, TS, Serialize, Debug)]
pub struct Prompt {
    question: String,
    answer: String,
}

pub enum Versioned {
    V1(Query),
}

pub enum Query {
    Pack { id: i32, filters: Vec<CardFilter> },
    Nested(Vec<(Query, u32)>),
}

enum CardFilter {
    WithTag(i32),
    WithoutTag(i32),
}

impl From<Versioned> for Query {
    fn from(value: Versioned) -> Self {
        match value {
            Versioned::V1(query) => query,
        }
    }
}

impl From<Query> for Versioned {
    fn from(value: Query) -> Self {
        Versioned::V1(value)
    }
}

pub async fn try_query(mut query: &Query, pool: &sqlx::SqlitePool) -> Result<Option<Prompt>> {
    loop {
        match query {
            Query::Pack { id, .. } => {
                return sqlx::query_as::<_, Prompt>(
                    "
                    SELECT front as question, back as answer
                    FROM cards
                    WHERE pack_id = ?
                    ORDER BY RANDOM()
                    LIMIT 1
                    ",
                )
                .bind(*id)
                .fetch_optional(pool)
                .await
                .map_err(Error::from);
            }
            Query::Nested(weighted_queries) => {
                let weights = weighted_queries.iter().map(|(_, weight)| *weight);
                let distribution = WeightedIndex::new(weights).map_err(Error::simple)?;

                let index = distribution.sample(&mut rand::thread_rng());

                query = &weighted_queries[index].0;
            }
        }
    }
}

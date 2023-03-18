use rand::{distributions::WeightedIndex, prelude::Distribution};
use serde::Serialize;
use sqlx::FromRow;
use ts_rs::TS;

use crate::prelude::*;

#[derive(FromRow, TS, Serialize, Debug)]
pub struct Prompt {
    pub question: String,
    pub answer: String,
}

pub enum Versioned {
    V1(CardQuery),
}

pub enum CardQuery {
    Root(Filter),
    Branch(Vec<WeightedQuery>),
}

pub struct Filter {
    pub pack_id: i64,
    pub included_tags: Vec<String>,
    pub excluded_tags: Vec<String>,
}

pub struct WeightedQuery {
    query: CardQuery,
    weight: u32,
}

impl From<Versioned> for CardQuery {
    fn from(value: Versioned) -> Self {
        match value {
            Versioned::V1(query) => query,
        }
    }
}

impl From<CardQuery> for Versioned {
    fn from(value: CardQuery) -> Self {
        Versioned::V1(value)
    }
}

pub async fn try_query(mut query: &CardQuery, pool: &sqlx::SqlitePool) -> Result<Option<Prompt>> {
    loop {
        match query {
            CardQuery::Root(filter) => {
                let mut sql = sqlx::QueryBuilder::new(
                    "
                    SELECT front as question, back as answer
                    FROM cards
                    WHERE pack_id =
                    ",
                );
                sql.push_bind(filter.pack_id);

                if !filter.included_tags.is_empty() {
                    sql.push("AND id IN (");
                    push_tag_matches(&mut sql, &filter.included_tags);
                    sql.push(
                        "
                        GROUP BY card_id
                        HAVING COUNT(*) =
                        "
                    )
                    .push_bind(filter.included_tags.len() as i64)
                    .push(")");
                }

                if !filter.excluded_tags.is_empty() {
                    sql.push("AND id NOT IN (");
                    push_tag_matches(&mut sql, &filter.excluded_tags);
                    sql.push(
                        "
                        GROUP BY card_id
                        )
                        "
                    );
                }

                sql.push(
                    "
                    ORDER BY RANDOM()
                    LIMIT 1
                    "
                );

                return sql.build_query_as::<Prompt>()
                    .fetch_optional(pool).await
                    .map_err(Error::from);
            }
            CardQuery::Branch(weighted_queries) => {
                let weights = weighted_queries.iter().map(|query| query.weight);
                let distribution = WeightedIndex::new(weights).map_err(Error::simple)?;

                let index = distribution.sample(&mut rand::thread_rng());

                query = &weighted_queries[index].query;
            }
        }
    }
}

fn push_tag_matches<'a>(sql: &mut sqlx::QueryBuilder<'a, sqlx::Sqlite>, tags: &'a [String]) {
    sql.push(
        "
        SELECT card_id
        FROM card_tags
        WHERE tag IN (
        ",
    );

    let mut sql_tags = sql.separated(", ");
    for tag in tags {
        sql_tags.push_bind(tag);
    }

    sql.push(")");
}

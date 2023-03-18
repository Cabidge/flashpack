use std::collections::BTreeMap;

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

struct Filter {
    pack_id: i64,
    tags: BTreeMap<i64, Inclusion>,
}

#[derive(Clone, Copy)]
enum Inclusion {
    Include,
    Exclude,
}

struct WeightedQuery {
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

                if !filter.tags.is_empty() {
                    sql.push(
                        "
                        AND id IN (
                            SELECT card_id
                            FROM card_tags
                            WHERE
                        ",
                    );

                    let mut sql_tags = sql.separated(" OR ");

                    for (tag_id, inclusion) in filter.tags.iter() {
                        let sql_str = match inclusion {
                            Inclusion::Include => "tag_id = ",
                            Inclusion::Exclude => "tag_id <> ",
                        };

                        sql_tags.push(sql_str).push_bind_unseparated(tag_id);
                    }

                    sql.push(
                        "
                        GROUP BY card_id
                        HAVING COUNT(*) =
                        ",
                    )
                    .push_bind(filter.tags.len() as i64)
                    .push(
                        "
                        )
                        SORT BY RANDOM()
                        LIMIT 1
                        "
                    );
                }

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

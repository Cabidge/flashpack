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
                todo!()
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

use serde::Serialize;
use sqlx::FromRow;
use ts_rs::TS;

use crate::card::CardSummary;

#[derive(FromRow, TS, Serialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct PackSummary {
    pub id: i64,
    pub title: String,
}

#[derive(TS, Serialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Pack {
    pub title: String,
    pub cards: Vec<CardSummary>,
}

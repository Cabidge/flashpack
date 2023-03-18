use serde::Serialize;
use sqlx::FromRow;
use ts_rs::TS;

#[derive(FromRow, TS, Serialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct CardSummary {
    pub id: i64,
    pub front: String,
    pub back: String,
}

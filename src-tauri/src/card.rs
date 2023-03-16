use serde::Serialize;
use sqlx::FromRow;
use ts_rs::TS;

#[derive(FromRow, TS, Serialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct CardSummary {
    id: i32,
    front: String,
    back: String,
}

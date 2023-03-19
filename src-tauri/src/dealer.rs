use crate::filter;

pub struct Summary {
    id: Id,
    title: String,
}

pub struct Dealer {
    title: String,
    filters: Vec<filter::DetailedSummary>
}

pub type Id = i64;
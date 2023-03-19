use crate::filter;

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
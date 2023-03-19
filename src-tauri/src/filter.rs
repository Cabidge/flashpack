pub struct Summary {
    id: Id,
    label: String,
}

pub struct DetailedSummary {
    summary: Summary,
    pack_title: String,
    weight: i32,
}

pub type Id = i64;
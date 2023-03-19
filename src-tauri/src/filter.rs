pub struct Summary {
    id: Id,
    label: String,
}

pub struct DetailedSummary {
    summary: Summary,
    pack_title: String,
    weight: i32,
}

pub struct Filter {
    label: String,
    included_tags: Vec<String>,
    excluded_tags: Vec<String>,
}

pub type Id = i64;
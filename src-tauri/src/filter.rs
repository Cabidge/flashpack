use std::collections::{BTreeSet, BTreeMap};

use futures::TryStreamExt;
use serde::Serialize;
use sqlx::SqlitePool;
use ts_rs::TS;

use crate::{card, prelude::*};

#[derive(TS, Serialize, Debug)]
#[ts(rename = "FilterSummary", export, export_to = "../src/bindings/")]
pub struct Summary {
    pub id: Id,
    pub label: String,
}

#[derive(TS, Serialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Filter {
    pub label: String,
    pub tags: Vec<Tag>,
    pub is_valid: bool,
}

#[derive(TS, Serialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Tag {
    tag: String,
    exclude: bool,
}

#[derive(Default)]
struct TagSets {
    included: BTreeSet<String>,
    excluded: BTreeSet<String>,
}

pub type GroupedFilters = BTreeMap<String, Vec<Summary>>;

pub type Id = u32;

pub async fn create(pool: &SqlitePool, pack_id: crate::pack::Id, label: &str) -> Result<Id> {
    let row = sqlx::query!(
        r#"
        INSERT INTO filters (label, pack_id)
        VALUES (?, ?)
        RETURNING id as "id: Id"
        "#,
        label,
        pack_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}

pub async fn list_all(pool: &SqlitePool) -> Result<GroupedFilters> {
    let mut rows = sqlx::query!(
        r#"
        SELECT f.id as "id: Id",
            f.label,
            p.title as pack_title
        FROM filters f, packs p
        WHERE f.pack_id = p.id
        ORDER BY LOWER(f.label)
        "#,
    )
    .fetch(pool);

    let mut filters = GroupedFilters::new();
    while let Some(row) = rows.try_next().await? {
        let group = filters.entry(row.pack_title).or_default();

        let filter = Summary {
            id: row.id,
            label: row.label,
        };

        group.push(filter);
    }

    Ok(filters)
}

pub async fn list_by_pack(pool: &SqlitePool, pack_id: crate::pack::Id) -> Result<Vec<Summary>> {
    sqlx::query_as!(
        Summary,
        r#"
        SELECT id as "id: Id", label
        FROM filters
        WHERE pack_id = ?
        ORDER BY LOWER(label)
        "#,
        pack_id,
    )
    .fetch_all(pool)
    .await
    .map_err(Error::from)
}

pub async fn get_label(pool: &SqlitePool, id: Id) -> Result<String> {
    sqlx::query!(
        "
        SELECT label
        FROM filters
        WHERE id = ?
        ",
        id,
    )
    .map(|row| row.label)
    .fetch_one(pool)
    .await
    .map_err(Error::from)
}

pub async fn list_tags(pool: &SqlitePool, id: Id) -> Result<Vec<Tag>> {
    sqlx::query_as!(
        Tag,
        "
        SELECT tag, exclude
        FROM filter_tags
        WHERE filter_id = ?
        ORDER BY LOWER(tag)
        ",
        id,
    )
    .fetch_all(pool)
    .await
    .map_err(Error::from)
}

pub async fn next_card(pool: &SqlitePool, filter_id: Id) -> Result<Option<card::Id>> {
    let tag_sets = TagSets::new(pool, filter_id).await?;

    let mut cards = sqlx::query!(
        r#"
        SELECT c.id as "id: card::Id"
        FROM cards c, filters f
        WHERE c.pack_id = f.pack_id
        AND f.id = ?
        ORDER BY RANDOM()
        "#,
        filter_id,
    )
    .fetch(pool);

    while let Some(row) = cards.try_next().await? {
        let id = row.id;

        let tags: BTreeSet<String> = sqlx::query!(
            "
            SELECT tag
            FROM card_tags
            WHERE card_id = ?
            ",
            id,
        )
        .fetch(pool)
        .map_ok(|row| row.tag)
        .try_collect()
        .await?;

        if tag_sets.matches(&tags) {
            return Ok(Some(id));
        }
    }

    Ok(None)
}

/// Figures out whether or not the given filter can 
pub async fn check_validity(pool: &SqlitePool, id: Id) -> Result<bool> {
    next_card(pool, id).await.map(|card_id| card_id.is_some())
}

pub async fn add_tag(pool: &SqlitePool, filter_id: Id, tag: &str) -> Result<()> {
    sqlx::query!(
        "
        INSERT INTO filter_tags (filter_id, tag)
        VALUES (?, ?)
        ",
        filter_id,
        tag,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn set_excluded(
    pool: &SqlitePool,
    filter_id: Id,
    tag: &str,
    exclude: bool,
) -> Result<()> {
    sqlx::query!(
        "
        UPDATE filter_tags
        SET exclude = ?
        WHERE filter_id = ?
        AND tag = ?
        ",
        exclude,
        filter_id,
        tag,
    )
    .execute(pool)
    .await?;

    Ok(())
}

impl TagSets {
    async fn new(pool: &SqlitePool, id: Id) -> Result<Self> {
        let mut rows = sqlx::query!(
            "
            SELECT tag, exclude
            FROM filter_tags
            WHERE filter_id = ?
            ",
            id,
        )
        .fetch(pool);

        let mut tag_sets = TagSets::default();
        while let Some(row) = rows.try_next().await? {
            if row.exclude {
                tag_sets.excluded.insert(row.tag);
            } else {
                tag_sets.included.insert(row.tag);
            }
        }

        Ok(tag_sets)
    }

    fn matches(&self, tags: &BTreeSet<String>) -> bool {
        self.included.is_subset(tags) && self.excluded.is_disjoint(tags)
    }
}

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod card;
mod dealer;
mod error;
mod filter;
mod pack;

mod commands;
use crate::commands::*;

mod prelude;
use crate::prelude::*;

use rand::Rng;
use rand::seq::SliceRandom;
use sqlx::SqlitePool;

#[macro_export]
macro_rules! bail {
    ($msg:literal $(,)?) => {
        return Err($crate::error::Error::Custom(anyhow::anyhow!($msg)))
    };
    ($err:expr $(,)?) => {
        return Err($crate::error::Error::Custom(anyhow::anyhow!($err)))
    };
    ($fmt:expr, $($arg:tt)*) => {
        return Err($crate::error::Error::Custom(anyhow::anyhow!($fmt, $($arg)*)))
    };
}

#[macro_export]
macro_rules! vars {
    {$($name: expr => $value: expr),* $(,)?} => {
        {
            let mut map = std::collections::BTreeMap::new();

            $(
                map.insert($name.to_string(), surrealdb::sql::Value::from($value));
            )*

            map
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let pool = SqlitePool::connect("sqlite::memory:").await?;

    sqlx::migrate!().run(&pool).await?;

    seed_database(&pool).await?;

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            generate_prompt,
            // pack
            create_pack,
            list_packs,
            get_pack,
            modify_pack,
            // card
            create_card,
            get_card,
            deal_card,
            modify_card,
            // dealer
            create_dealer,
            list_dealers,
            get_dealer,
            modify_dealer,
            // filter
            create_filter,
            list_filters,
            get_filter,
            modify_filter,
        ])
        .manage(pool)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

async fn seed_database(pool: &SqlitePool) -> Result<()> {
    let mut rng = rand::thread_rng();

    let tags = ["math", "epic", "hard", "important"];
    let mut filters = vec![];

    for _ in 0..10 {
        let pack_id = pack::create(pool, "pack").await?;

        for _ in 0..(rng.gen_range(1..=10)) {
            let lhs = rng.gen_range(1..=10);
            let rhs = rng.gen_range(1..=10);

            let front = format!("What is {lhs} + {rhs}?");
            let back = format!("{} + {} = {}", lhs, rhs, lhs + rhs);

            let card_id = card::create(pool, pack_id, &front, &back).await?;

            let tag_count = rng.gen_range(0..=tags.len());
            let card_tags = tags.choose_multiple(&mut rng, tag_count);

            for tag in card_tags {
                card::add_tag(pool, card_id, tag).await?;
            }
        }

        for _ in 0..2 {
            let tag_count = rng.gen_range(0..=tags.len());
            let filter_tags = tags
                .choose_multiple(&mut rng, tag_count)
                .map(|&tag| {
                    let exclude = rng.gen_bool(0.2);
                    (tag, exclude)
                })
                .collect::<Vec<_>>();

            let label = if tag_count == 0 {
                String::from("all")
            } else {
                filter_tags.iter()
                    .map(|&(tag, exclude)| {
                        let c = if exclude { '-' } else { '+' };
                        format!("{c}{tag}")
                    })
                    .collect::<Vec<_>>()
                    .join(",")
            };

            let filter_id = filter::create(pool, pack_id, &label).await?;

            filters.push(filter_id);

            for (tag, exclude) in filter_tags {
                filter::add_tag(pool, filter_id, tag).await?;

                if exclude {
                    filter::set_excluded(pool, filter_id, tag, exclude).await?;
                }
            }
        }
    }

    for i in 0..10 {
        let title = format!("Study #{i}");
        let dealer_id = dealer::create(pool, &title).await?;

        let filter_count = rng.gen_range(1..=3);
        let dealer_filters = filters.choose_multiple(&mut rng, filter_count);

        for &filter_id in dealer_filters {
            let weight = if rng.gen_bool(0.1) {
                rng.gen_range(2..=5)
            } else {
                1
            };

            dealer::add_filter(pool, dealer_id, filter_id, weight).await?;
        }
    }

    Ok(())
}

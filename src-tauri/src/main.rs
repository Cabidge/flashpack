#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod card;
mod engine;
mod error;
mod markdown;
mod pack;
mod study;

mod commands;
use crate::commands::*;

mod prelude;
use crate::prelude::*;

use rand::seq::SliceRandom;
use rand::Rng;
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

#[tokio::main]
async fn main() -> Result<()> {
    let pool = SqlitePool::connect("sqlite::memory:").await?;

    sqlx::migrate!().run(&pool).await?;

    seed_database(&pool).await?;

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            render_markdown,
            generate_prompt,
            // pack
            pack_create,
            pack_list,
            pack_cards,
            pack_modify,
            // card
            card_create,
            card_query,
            card_tags,
            card_modify,
            // study
            study_create,
            study_list,
            study_tags,
            study_modify,
        ])
        .manage(pool)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

async fn seed_database(pool: &SqlitePool) -> Result<()> {
    let mut rng = rand::thread_rng();

    let tags = ["math", "epic", "hard", "important"];

    for _ in 0..10 {
        let pack_id = pack::create(pool, "pack").await?;

        for _ in 0..(rng.gen_range(1..=10)) {
            let lhs = rng.gen_range(1..=10);
            let rhs = rng.gen_range(1..=10);

            let front = format!("What is {lhs} + {rhs}?");
            let back = format!("{} + {} = {}", lhs, rhs, lhs + rhs);

            let card_id = card::create(pool, pack_id, &front, &front, &back).await?;

            let tag_count = rng.gen_range(0..=tags.len());
            let card_tags = tags.choose_multiple(&mut rng, tag_count);

            for tag in card_tags {
                card::add_tag(pool, card_id, tag).await?;
            }
        }
    }

    Ok(())
}

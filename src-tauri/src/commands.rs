use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::State;
use ts_rs::TS;

use crate::{
    card::{self, Card},
    dealer, filter,
    pack::{self, Pack},
    prelude::*,
};

#[derive(TS, Deserialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub enum ModifyPack {
    Rename(String),
    Delete,
}

#[derive(TS, Deserialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub enum ModifyCard {
    AddTag(String),
    RemoveTag(String),
    Rename {
        front: Option<String>,
        back: Option<String>,
    },
}

#[derive(TS, Deserialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub enum ModifyDealer {
    AddFilter(filter::Id),
    RemoveFilter(filter::Id),
    SetWeight(filter::Id, i32),
}

#[derive(TS, Deserialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub enum ModifyFilter {
    AddTag(String),
    SetExclusion(String, bool),
}

#[derive(TS, Serialize, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Prompt {
    question: String,
    answer: String,
}

#[tauri::command]
pub fn generate_prompt(script: Option<String>, question: String, answer: String) -> Prompt {
    fn parse_markdown(input: &str) -> String {
        let parser = pulldown_cmark::Parser::new(input);
        let mut output = String::new();
        pulldown_cmark::html::push_html(&mut output, parser);

        output
    }

    fn parse_latex(input: &str) -> String {
        use nom::branch::alt;
        use nom::bytes::complete::{tag, take_until1};
        use nom::character::complete::anychar;
        use nom::IResult;
        use nom::multi::fold_many0;
        use nom::Parser;
        use nom::sequence::delimited;

        enum ParsedLatex<'a> {
            Char(char),
            Latex(&'a str)
        }

        fn parse_latex(input: &str) -> IResult<&str, ParsedLatex<'_>> {
            let latex = delimited(tag("$"), take_until1("$"), tag("$")).map(ParsedLatex::Latex);
            let char_wrapped = anychar.map(ParsedLatex::Char);
            alt((latex, char_wrapped))(input)
        }

        let opts = katex::Opts::builder().output_type(katex::OutputType::Mathml).throw_on_error(false).build().unwrap();

        let mut parser = fold_many0(parse_latex, String::new, |mut acc, parsed| {
            match parsed {
                ParsedLatex::Char(ch) => acc.push(ch),
                ParsedLatex::Latex(latex) => {
                    let parsed = katex::render_with_opts(latex, &opts).expect("bad");
                    acc.push_str(&parsed);
                }
            }
            acc
        });

        parser(input).expect("bad nom").1
    }

    fn parse(input: &str) -> String {
        let output = parse_markdown(input);
        parse_latex(&output)
    }

    Prompt {
        question: parse(&question),
        answer: parse(&answer),
    }
}

// -- pack

#[tauri::command]
pub async fn create_pack(pool: State<'_, SqlitePool>, title: String) -> Result<()> {
    pack::create(pool.inner(), &title).await?;
    Ok(())
}

#[tauri::command]
pub async fn list_packs(pool: State<'_, SqlitePool>) -> Result<Vec<pack::Summary>> {
    pack::list_all(pool.inner()).await
}

#[tauri::command]
pub async fn get_pack(pool: State<'_, SqlitePool>, id: pack::Id) -> Result<Pack> {
    let title = pack::get_title(pool.inner(), id).await?;
    let cards = card::list_by_pack(pool.inner(), id).await?;
    let filters = filter::list_by_pack(pool.inner(), id).await?;

    let mut invalid_filters = vec![];
    for filter_id in filters.iter().map(|filter| filter.id) {
        if !filter::check_validity(pool.inner(), filter_id).await? {
            invalid_filters.push(filter_id);
        }
    }

    Ok(Pack {
        title,
        cards,
        filters,
        invalid_filters,
    })
}

#[tauri::command]
pub async fn modify_pack(
    pool: State<'_, SqlitePool>,
    id: pack::Id,
    action: ModifyPack,
) -> Result<()> {
    match action {
        ModifyPack::Rename(new_title) => pack::rename(pool.inner(), id, &new_title).await,
        ModifyPack::Delete => pack::delete(pool.inner(), id).await,
    }
}

// -- card

#[tauri::command]
pub async fn create_card(
    pool: State<'_, SqlitePool>,
    pack_id: pack::Id,
    label: String,
) -> Result<()> {
    let front = format!("## {label} Question Side");
    let back = format!("## {label} Answer Side");
    card::create(pool.inner(), pack_id, &label, &front, &back).await?;
    Ok(())
}

#[tauri::command]
pub async fn get_card(pool: State<'_, SqlitePool>, id: card::Id) -> Result<Card> {
    let details = card::get_details(pool.inner(), id).await?;
    let tags = card::list_tags(pool.inner(), id).await?;

    Ok(Card { details, tags })
}

#[tauri::command]
pub async fn deal_card(
    pool: State<'_, SqlitePool>,
    dealer_id: dealer::Id,
) -> Result<Option<card::Id>> {
    let Some(filter_id) = dealer::next_filter(pool.inner(), dealer_id).await? else { return Ok(None) };
    filter::next_card(pool.inner(), filter_id).await
}

#[tauri::command]
pub async fn modify_card(
    pool: State<'_, SqlitePool>,
    id: card::Id,
    action: ModifyCard,
) -> Result<()> {
    match action {
        ModifyCard::AddTag(tag) => card::add_tag(pool.inner(), id, &tag).await,
        ModifyCard::RemoveTag(tag) => card::remove_tag(pool.inner(), id, &tag).await,
        ModifyCard::Rename { front, back } => {
            card::rename(pool.inner(), id, front.as_deref(), back.as_deref()).await
        }
    }
}

// -- dealer

#[tauri::command]
pub async fn create_dealer(pool: State<'_, SqlitePool>, title: String) -> Result<()> {
    dealer::create(pool.inner(), &title).await?;
    Ok(())
}

#[tauri::command]
pub async fn list_dealers(pool: State<'_, SqlitePool>) -> Result<Vec<dealer::Summary>> {
    dealer::list_all(pool.inner()).await
}

#[tauri::command]
pub async fn get_dealer(pool: State<'_, SqlitePool>, id: dealer::Id) -> Result<dealer::Dealer> {
    let title = dealer::get_title(pool.inner(), id).await?;
    let filters = dealer::list_filters(pool.inner(), id).await?;

    let mut invalid_filters = vec![];
    for filter_id in filters.flatten().map(|filter| filter.summary.id) {
        if !filter::check_validity(pool.inner(), filter_id).await? {
            invalid_filters.push(filter_id);
        }
    }

    Ok(dealer::Dealer {
        title,
        filters,
        invalid_filters,
    })
}

#[tauri::command]
pub async fn modify_dealer(
    pool: State<'_, SqlitePool>,
    id: dealer::Id,
    action: ModifyDealer,
) -> Result<()> {
    match action {
        ModifyDealer::AddFilter(filter_id) => {
            dealer::add_filter(pool.inner(), id, filter_id, 1).await
        }
        ModifyDealer::RemoveFilter(filter_id) => {
            dealer::remove_filter(pool.inner(), id, filter_id).await
        }
        ModifyDealer::SetWeight(filter_id, weight) => {
            dealer::set_weight(pool.inner(), id, filter_id, weight).await
        }
    }
}

// -- filter

#[tauri::command]
pub async fn create_filter(
    pool: State<'_, SqlitePool>,
    pack_id: pack::Id,
    label: String,
) -> Result<()> {
    filter::create(pool.inner(), pack_id, &label).await?;
    Ok(())
}

#[tauri::command]
pub async fn list_filters(pool: State<'_, SqlitePool>) -> Result<filter::GroupedFilters> {
    filter::list_all(pool.inner()).await
}

#[tauri::command]
pub async fn get_filter(pool: State<'_, SqlitePool>, id: filter::Id) -> Result<filter::Filter> {
    let details = filter::get_details(pool.inner(), id).await?;
    let tags = filter::list_tags(pool.inner(), id).await?;
    let is_valid = filter::check_validity(pool.inner(), id).await?;

    Ok(filter::Filter {
        details,
        tags,
        is_valid,
    })
}

#[tauri::command]
pub async fn modify_filter(
    pool: State<'_, SqlitePool>,
    id: filter::Id,
    action: ModifyFilter,
) -> Result<()> {
    match action {
        ModifyFilter::AddTag(tag) => filter::add_tag(pool.inner(), id, &tag).await,
        ModifyFilter::SetExclusion(tag, exclude) => {
            filter::set_excluded(pool.inner(), id, &tag, exclude).await
        }
    }
}

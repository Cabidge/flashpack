// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{path::PathBuf, sync::Mutex};

use tauri::{api::dialog::blocking::FileDialogBuilder, State};

struct Collection(PathBuf);
struct Pack(PathBuf);
struct Card(PathBuf);

type CollectionState = Mutex<Option<Collection>>;

impl Collection {
    fn pack(&self, pack_name: &str) -> Pack {
        Pack(self.0.join(pack_name))
    }
}

impl Pack {
    fn open_card(self, card_name: &str) -> Card {
        let mut card_path = self.0;
        card_path.push(card_name);
        card_path.set_extension(CARD_EXTENSION);
        Card(card_path)
    }
}

impl Card {
    fn set_contents(&self, contents: &str) -> std::io::Result<()> {
        if let Some(pack_path) = self.0.parent() {
            std::fs::create_dir_all(pack_path)?;
        }
        std::fs::write(&self.0, contents)
    }
}

const CARD_EXTENSION: &str = "fmark";

#[tauri::command]
async fn open_collection(collection: State<'_, CollectionState>) -> Result<bool, ()> {
    let Some(path) = FileDialogBuilder::new().pick_folder() else {
        return Ok(false);
    };

    *collection.lock().unwrap() = Some(Collection(path));

    Ok(true)
}

#[tauri::command]
fn get_collection_name(collection: State<CollectionState>) -> Option<String> {
    collection
        .lock()
        .unwrap()
        .as_ref()
        .and_then(|Collection(path)| path.file_name())
        .map(|name| name.to_string_lossy().into_owned())
}

#[tauri::command]
fn list_packs(collection: State<CollectionState>) -> Vec<String> {
    let collection = collection.lock().unwrap();

    let Some(collection_contents) = collection
        .as_ref()
        .and_then(|collection| collection.0.read_dir().ok())
    else {
        return vec![];
    };

    collection_contents
        .filter_map(|entry| {
            let entry = entry.ok()?;
            entry
                .metadata()
                .ok()?
                // packs can only be directories
                .is_dir()
                .then(|| entry.file_name())
                // the accepted pack name should only contain valid String characters
                .and_then(|name| name.into_string().ok())
        })
        .collect()
}

#[tauri::command]
fn delete_pack(collection: State<CollectionState>, pack_name: String) {
    let collection = collection.lock().unwrap();
    let Some(collection) = collection.as_ref() else {
        return;
    };

    let pack = collection.pack(&pack_name);
    let _ = std::fs::remove_dir_all(pack.0).ok();
}

#[tauri::command]
fn list_cards(collection: State<CollectionState>, pack_name: String) -> Vec<String> {
    let collection = collection.lock().unwrap();
    let Some(cards) = collection
        .as_ref()
        .and_then(|collection| collection.pack(&pack_name).0.read_dir().ok())
    else {
        return vec![];
    };

    cards
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            (path.extension()? == CARD_EXTENSION && entry.file_type().ok()?.is_file())
                .then_some(path)
                .and_then(|path| path.file_stem()?.to_str().map(String::from))
        })
        .collect()
}

#[tauri::command]
fn add_card(
    collection: State<CollectionState>,
    pack_name: String,
    card_name: String,
    contents: String,
) {
    let mut collection = collection.lock().unwrap();
    let Some(collection) = collection.as_mut() else {
        return;
    };

    let card = collection.pack(&pack_name).open_card(&card_name);

    // TODO: error handling
    card.set_contents(&contents).unwrap();
}

#[tauri::command]
fn get_card(
    collection: State<CollectionState>,
    pack_name: String,
    card_name: String,
) -> Option<String> {
    let collection = collection.lock().unwrap();
    let card = collection.as_ref()?.pack(&pack_name).open_card(&card_name);

    std::fs::read_to_string(card.0).ok()
}

#[tauri::command]
fn delete_card(collection: State<CollectionState>, pack_name: String, card_name: String) {
    let collection = collection.lock().unwrap();
    let Some(collection) = collection.as_ref() else {
        return;
    };

    let card = collection.pack(&pack_name).open_card(&card_name);
    let _ = std::fs::remove_file(card.0).ok();
}

#[tauri::command]
fn deal_cards(collection: State<CollectionState>, pack_name: String) -> Vec<String> {
    let collection = collection.lock().unwrap();

    let Some(cards) = collection
        .as_ref()
        .map(|collection| collection.pack(&pack_name))
        .and_then(|Pack(path)| path.read_dir().ok())
    else {
        return vec![];
    };

    let card_contents = cards
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            (path.extension()? == CARD_EXTENSION && entry.file_type().ok()?.is_file())
                .then_some(path)
                .and_then(|path| std::fs::read_to_string(path).ok())
        })
        .collect();

    // TODO: shuffle cards

    card_contents
}

fn main() {
    tauri::Builder::default()
        .manage(CollectionState::default())
        .invoke_handler(tauri::generate_handler![
            open_collection,
            get_collection_name,
            list_packs,
            delete_pack,
            list_cards,
            add_card,
            get_card,
            delete_card,
            deal_cards,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

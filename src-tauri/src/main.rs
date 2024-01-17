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

const CARD_EXTENSION: &str = "flashmark";

#[tauri::command]
async fn open_collection(collection: State<'_, CollectionState>) -> Result<Option<String>, ()> {
    let Some(path) = FileDialogBuilder::new().pick_folder() else {
        return Ok(None);
    };

    let name = match path.file_name() {
        Some(name) => name.to_string_lossy().into_owned(),
        None => String::from("*Unknown*"),
    };

    *collection.lock().unwrap() = Some(Collection(path));

    Ok(Some(name))
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

fn main() {
    tauri::Builder::default()
        .manage(CollectionState::default())
        .invoke_handler(tauri::generate_handler![
            open_collection,
            list_packs,
            add_card
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

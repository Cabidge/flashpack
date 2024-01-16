// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{path::PathBuf, sync::Mutex};

use tauri::{api::dialog::blocking::FileDialogBuilder, State};

struct Collection(PathBuf);

type CollectionState = Mutex<Option<Collection>>;

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

fn main() {
    tauri::Builder::default()
        .manage(CollectionState::default())
        .invoke_handler(tauri::generate_handler![open_collection, list_packs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

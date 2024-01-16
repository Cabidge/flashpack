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

fn main() {
    tauri::Builder::default()
        .manage(CollectionState::default())
        .invoke_handler(tauri::generate_handler![open_collection])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

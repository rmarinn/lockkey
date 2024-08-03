// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use lockkey::Session;

#[tauri::command]
fn new_secret(
    kind: String,
    label: String,
    data: String,
    state: tauri::State<Arc<Mutex<Session>>>,
) -> String {
    let sess = state.lock().expect("should get session");

    match sess.insert_into_db(&kind, &label, &data) {
        Ok(()) => "Ok".to_string(),
        Err(e) => format!("Error: {e:?}"),
    }
}

#[tauri::command]
fn get_labels(state: tauri::State<Arc<Mutex<Session>>>) -> Vec<(String, String)> {
    let sess = state.lock().expect("should get session");

    match sess.retrieve_labels() {
        Ok(labels) => labels.into_iter().map(|x| (x.label, x.kind)).collect(),
        _ => vec![],
    }
}

fn main() {
    let sess = Arc::new(Mutex::new(Session::new()));
    tauri::Builder::default()
        .manage(sess)
        .invoke_handler(tauri::generate_handler![get_labels, new_secret])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
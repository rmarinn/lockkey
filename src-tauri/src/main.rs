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

    match sess.store_secret(&kind, &label, data) {
        Ok(()) => "Ok".to_string(),
        Err(e) => format!("Error: {e:?}"),
    }
}

#[tauri::command]
fn delete_secret(label: String, state: tauri::State<Arc<Mutex<Session>>>) -> String {
    let sess = state.lock().expect("should get session");

    match sess.delete_secret(&label) {
        Ok(()) => "Ok".to_string(),
        Err(e) => format!("Error: {e:?}"),
    }
}

#[derive(serde::Serialize)]
struct Label {
    label: String,
    kind: String,
}

#[tauri::command]
fn get_labels(state: tauri::State<Arc<Mutex<Session>>>) -> Vec<Label> {
    let sess = state.lock().expect("should get session");

    match sess.retrieve_labels() {
        Ok(labels) => labels
            .into_iter()
            .map(|x| Label {
                label: x.label,
                kind: x.kind,
            })
            .collect(),
        _ => vec![],
    }
}

#[tauri::command]
fn get_secret(label: String, state: tauri::State<Arc<Mutex<Session>>>) -> Option<String> {
    let sess = state.lock().expect("should get session");

    match sess.retrieve_secret(&label) {
        Ok(secret) => secret,
        _ => None,
    }
}

fn main() {
    let username = "test_user".to_string();
    let pass = "test_passwd".to_string();
    let db_path = "./test.db";

    let sess = Session::new()
        .set_credentials(&username, pass)
        .connect_to_db(&db_path);
    let sess = Arc::new(Mutex::new(sess));
    tauri::Builder::default()
        .manage(sess)
        .invoke_handler(tauri::generate_handler![
            get_labels,
            new_secret,
            get_secret,
            delete_secret
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

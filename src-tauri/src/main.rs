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
fn delete_secret(label: String, state: tauri::State<Arc<Mutex<Session>>>) -> String {
    let sess = state.lock().expect("should get session");

    match sess.delete_data(&label) {
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

fn main() {
    let sess = Session::new()
        .set_key(&String::from("test"))
        .connect_to_db("test.db");
    let sess = Arc::new(Mutex::new(sess));
    tauri::Builder::default()
        .manage(sess)
        .invoke_handler(tauri::generate_handler![
            get_labels,
            new_secret,
            delete_secret
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

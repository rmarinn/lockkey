// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use lockkey::Session;
use serde_json::{json, Value};

#[derive(serde::Serialize)]
struct Response {
    success: bool,
    body: Option<Value>,
}

impl Response {
    pub fn ok() -> Self {
        Response {
            success: true,
            body: None,
        }
    }

    pub fn err() -> Self {
        Response {
            success: false,
            body: None,
        }
    }

    pub fn body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }
}

#[derive(serde::Serialize)]
struct Label {
    label: String,
    kind: String,
}

#[tauri::command]
fn new_secret(
    kind: String,
    label: String,
    data: String,
    state: tauri::State<Arc<Mutex<Session>>>,
) -> Response {
    let sess = state.lock().expect("should get session");

    match sess.store_secret(&kind, &label, data) {
        Ok(()) => Response::ok().body(json!("secret created".to_string())),
        Err(e) => Response::err().body(json!(format!("Error creating secret: {e:?}"))),
    }
}

#[tauri::command]
fn edit_secret(
    label: String,
    new_label: String,
    new_data: String,
    state: tauri::State<Arc<Mutex<Session>>>,
) -> Response {
    let sess = state.lock().expect("should get session");

    match sess.edit_secret(&label, &new_label, new_data) {
        Ok(()) => Response::ok().body(json!("scret edited".to_string())),
        Err(e) => Response::err().body(json!(format!("Error creating secret: {e:?}"))),
    }
}

#[tauri::command]
fn delete_secret(label: String, state: tauri::State<Arc<Mutex<Session>>>) -> Response {
    let sess = state.lock().expect("should get session");

    match sess.delete_secret(&label) {
        Ok(()) => Response::ok().body(json!(format!("{:?} deleted", label))),
        Err(e) => Response::err().body(json!(format!("Error deleting secret: {e:?}"))),
    }
}

#[tauri::command]
fn get_labels(state: tauri::State<Arc<Mutex<Session>>>) -> Response {
    let sess = state.lock().expect("should get session");

    match sess.retrieve_labels() {
        Ok(labels) => {
            let labels: Vec<Label> = labels
                .into_iter()
                .map(|x| Label {
                    label: x.label,
                    kind: x.kind,
                })
                .collect();
            Response::ok().body(json!(labels))
        }
        Err(e) => Response::err().body(json!(format!("Error getting labels: {e:?}"))),
    }
}

#[tauri::command]
fn get_secret(label: String, state: tauri::State<Arc<Mutex<Session>>>) -> Response {
    let sess = state.lock().expect("should get session");

    match sess.retrieve_secret(&label) {
        Ok(result) => match result {
            Some(s) => Response::ok().body(json!(s)),
            None => Response::err().body(json!(format!("{:?} does not exist", label))),
        },
        Err(e) => Response::err().body(json!(format!("Error getting secret: {e:?}"))),
    }
}

#[tauri::command]
fn is_authenticated(state: tauri::State<Arc<Mutex<Session>>>) -> Response {
    let sess = state.lock().expect("should get session");
    Response::ok().body(json!(sess.is_authenticated()))
}

#[tauri::command]
fn new_user(usrname: String, passwd: String, state: tauri::State<Arc<Mutex<Session>>>) -> Response {
    let sess = state.lock().expect("should get session");
    match sess.create_user(&usrname, passwd) {
        Ok(()) => Response::ok().body(json!(format!("user `{:?}` created", usrname))),
        Err(e) => Response::err().body(json!(format!("Error creating a new account: {e:?}"))),
    }
}

#[tauri::command]
fn login(usrname: String, passwd: String, state: tauri::State<Arc<Mutex<Session>>>) -> Response {
    let mut sess = state.lock().expect("should get session");
    match sess.authenticate_user(&usrname, passwd) {
        Ok(()) => Response::ok().body(json!(format!("logged in as {:?}", usrname))),
        Err(e) => Response::err().body(json!(format!("Error loggin in: {e:?}"))),
    }
}

#[tauri::command]
fn logout(state: tauri::State<Arc<Mutex<Session>>>) -> Response {
    let mut sess = state.lock().expect("should get session");
    match sess.logout() {
        Ok(()) => Response::ok().body(json!("logged out")),
        Err(e) => Response::ok().body(json!(format!("Error logging out: {e:?}"))),
    }
}

fn main() {
    let db_path = "./test.db";

    let sess = Session::new().connect_to_db(&db_path);

    let sess = Arc::new(Mutex::new(sess));
    tauri::Builder::default()
        .manage(sess)
        .invoke_handler(tauri::generate_handler![
            get_labels,
            new_secret,
            get_secret,
            edit_secret,
            delete_secret,
            is_authenticated,
            login,
            logout,
            new_user,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

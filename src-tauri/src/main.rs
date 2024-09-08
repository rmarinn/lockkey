// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    fs,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use lockkey::{create_new_account, Session};
use serde_json::{json, Value};
use tauri::Manager;

/// Represents a generic response structure for Tauri commands.
#[derive(serde::Serialize)]
struct Response {
    success: bool,
    body: Option<Value>,
}

impl Response {
    /// Constructs a successful response without a body.
    pub fn ok() -> Self {
        Response {
            success: true,
            body: None,
        }
    }

    /// Constructs an error response without a body.
    pub fn err() -> Self {
        Response {
            success: false,
            body: None,
        }
    }

    /// Attaches a JSON body to the response.
    ///
    /// # Arguments
    /// * `body` - A `serde_json::Value` that will be attached as the body of the response.
    pub fn body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }
}

/// Represents a secret label with its kind.
#[derive(serde::Serialize)]
struct Label {
    label: String,
    kind: String,
}

/// Command to store a new secret.
///
/// # Arguments
/// * `kind` - The type of secret (e.g., password, text).
/// * `label` - The label for the secret.
/// * `data` - The secret data to store.
/// * `state` - A shared state containing the current session.
///
/// # Returns
/// A `Response` indicating success or failure.
#[tauri::command]
fn new_secret(
    kind: String,
    label: String,
    data: String,
    state: tauri::State<Arc<Mutex<Option<Session>>>>,
) -> Response {
    let sess_guard = state.lock().expect("should get session");

    match *sess_guard {
        Some(ref session) => match session.store_secret(&kind, &label, data) {
            Ok(()) => Response::ok().body(json!("Secret created".to_string())),
            Err(e) => Response::err().body(json!(format!("Error creating secret: {e:?}"))),
        },
        None => Response::err().body(json!("No running session")),
    }
}

/// Command to edit an existing secret.
///
/// # Arguments
/// * `label` - The label of the secret to edit.
/// * `new_label` - The new label for the secret.
/// * `new_data` - The new data for the secret.
/// * `session` - A shared state containing the current session.
///
/// # Returns
/// A `Response` indicating success or failure.
#[tauri::command]
fn edit_secret(
    label: String,
    new_label: String,
    new_data: String,
    session: tauri::State<Arc<Mutex<Option<Session>>>>,
) -> Response {
    let sess_guard = session.lock().unwrap();

    match *sess_guard {
        Some(ref session) => match session.edit_secret(&label, &new_label, new_data) {
            Ok(()) => Response::ok().body(json!("secret edited".to_string())),
            Err(e) => Response::err().body(json!(format!("Error creating secret: {e:?}"))),
        },
        None => Response::err().body(json!(format!("No running session"))),
    }
}

/// Command to delete a secret.
///
/// # Arguments
/// * `label` - The label of the secret to delete.
/// * `state` - A shared state containing the current session.
///
/// # Returns
/// A `Response` indicating success or failure.
#[tauri::command]
fn delete_secret(label: String, state: tauri::State<Arc<Mutex<Option<Session>>>>) -> Response {
    let sess_guard = state.lock().expect("should get session");

    match *sess_guard {
        Some(ref session) => match session.delete_secret(&label) {
            Ok(()) => Response::ok().body(json!(format!("{:?} deleted", label))),
            Err(e) => Response::err().body(json!(format!("Error deleting secret: {e:?}"))),
        },
        None => Response::err().body(json!(format!("No running session"))),
    }
}

/// Command to retrieve all secret labels.
///
/// # Arguments
/// * `state` - A shared state containing the current session.
///
/// # Returns
/// A `Response` with the list of secret labels, or an error.
#[tauri::command]
fn get_labels(state: tauri::State<Arc<Mutex<Option<Session>>>>) -> Response {
    let sess_guard = state.lock().expect("should get session");

    match *sess_guard {
        Some(ref session) => match session.retrieve_labels() {
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
        },
        None => Response::err().body(json!(format!("No running session"))),
    }
}

/// Command to retrieve a secret by its label.
///
/// # Arguments
/// * `label` - The label of the secret to retrieve.
/// * `state` - A shared state containing the current session.
///
/// # Returns
/// A `Response` with the secret data, or an error if the secret does not exist.
#[tauri::command]
fn get_secret(label: String, state: tauri::State<Arc<Mutex<Option<Session>>>>) -> Response {
    let sess_guard = state.lock().expect("should get session");

    match *sess_guard {
        Some(ref session) => match session.retrieve_secret(&label) {
            Ok(result) => match result {
                Some(s) => Response::ok().body(json!(s)),
                None => Response::err().body(json!(format!("{:?} does not exist", label))),
            },
            Err(e) => Response::err().body(json!(format!("Error getting secret: {e:?}"))),
        },
        None => Response::err().body(json!(format!("No running session"))),
    }
}

/// Command to check if a user is authenticated.
///
/// # Arguments
/// * `state` - A shared state containing the current session.
///
/// # Returns
/// A `Response` indicating whether the user is authenticated or not.
#[tauri::command]
fn is_authenticated(state: tauri::State<Arc<Mutex<Option<Session>>>>) -> Response {
    let sess_guard = state.lock().expect("should get managed session state");
    match *sess_guard {
        Some(_) => Response::ok().body(json!(true)),
        None => Response::ok().body(json!(false)),
    }
}

/// Command to create a new user.
///
/// # Arguments
/// * `usrname` - The username of the new user.
/// * `passwd` - The password for the new user.
/// * `db_path` - A shared state containing the database path.
///
/// # Returns
/// A `Response` indicating success or failure of user creation.
#[tauri::command]
fn new_user(
    usrname: String,
    passwd: String,
    db_path: tauri::State<Arc<Mutex<String>>>,
) -> Response {
    let db_path = db_path.lock().expect("should get db path");
    match create_new_account(&usrname, passwd, &db_path) {
        Ok(()) => Response::ok().body(json!(format!("user `{:?}` created", usrname))),
        Err(e) => Response::err().body(json!(format!("Error creating a new account: {e:?}"))),
    }
}

/// Command to log in a user.
///
/// # Arguments
/// * `usrname` - The username for login.
/// * `passwd` - The password for login.
/// * `app_handle` - A handle to the Tauri application.
/// * `session` - A shared state containing the current session.
/// * `db_path` - A shared state containing the database path.
///
/// # Returns
/// A `Response` indicating success or failure of login.
#[tauri::command]
fn login(
    usrname: String,
    passwd: String,
    app_handle: tauri::AppHandle,
    session: tauri::State<Arc<Mutex<Option<Session>>>>,
    db_path: tauri::State<Arc<Mutex<String>>>,
) -> Response {
    let db_path = db_path.lock().expect("should get db path");

    // log out existing session if there is one
    if let Some(sess) = session.lock().unwrap().as_mut() {
        _ = sess.logout();
    }

    // create a new session and update app state
    {
        let new_session = match Session::new(&usrname, passwd, &db_path) {
            Ok(sess) => sess,
            Err(e) => return Response::err().body(json!(format!("Error loggin in: {e:?}"))),
        };

        let mut sess_guard = session.lock().unwrap();
        *sess_guard = Some(new_session);
    }

    // spawn a thread to monitor session timeout every 30 secs
    let session = Arc::clone(&session);
    let session_timeout = Duration::from_secs(300);
    thread::spawn(move || {
        loop {
            let mut sess_guard = session.lock().unwrap();

            match sess_guard.as_ref() {
                Some(sess) => {
                    if sess.last_activity.elapsed() >= session_timeout {
                        // trigger session timeout action
                        app_handle
                            .emit_all("session_timeout", "Logged out due to inactivity")
                            .unwrap();
                        *sess_guard = None;
                        break;
                    }
                }
                None => {
                    *sess_guard = None;
                    break;
                }
            }

            drop(sess_guard);
            thread::sleep(Duration::from_secs(30)); // check every 30 secs
        }
    });

    Response::ok().body(json!(format!("logged in as {:?}", usrname)))
}

#[tauri::command]
fn logout(session: tauri::State<Arc<Mutex<Option<Session>>>>) -> Response {
    let mut sess_guard = session.lock().unwrap();
    match *sess_guard {
        Some(ref mut sess) => {
            return match sess.logout() {
                Ok(()) => {
                    *sess_guard = None;
                    Response::ok().body(json!("logged out"))
                }
                Err(e) => Response::ok().body(json!(format!("Error logging out: {e:?}"))),
            };
        }
        None => {
            *sess_guard = None;
            Response::ok().body(json!("already logged out"))
        }
    }
}

#[tauri::command]
fn update_last_activity(session: tauri::State<Arc<Mutex<Option<Session>>>>) -> Response {
    let mut sess_guard = session.lock().unwrap();
    match *sess_guard {
        Some(ref mut sess) => {
            sess.last_activity = Instant::now();
            Response::ok().body(json!("Session refreshed"))
        }
        None => Response::err().body(json!("No running session")),
    }
}

fn main() {
    let session_state: Arc<Mutex<Option<Session>>> = Arc::new(Mutex::new(None));
    let db_path: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));

    let db_path_clone: Arc<Mutex<String>> = Arc::clone(&db_path);
    tauri::Builder::default()
        .setup(move |app| {
            let mut db_pathbuf = app
                .path_resolver()
                .app_data_dir()
                .expect("should get app directory");
            fs::create_dir_all(&db_pathbuf).expect("should create app directory");

            db_pathbuf.push("lockkey.secrets");
            let db_path_str = db_pathbuf.to_str().unwrap().to_string();

            let mut db_path_lock = db_path_clone.lock().unwrap();
            *db_path_lock = db_path_str;

            Ok(())
        })
        .manage(db_path)
        .manage(session_state)
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
            update_last_activity
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

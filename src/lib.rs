pub mod data;
pub mod encryption;

use anyhow::Result;

use crate::data::DbConn;
use crate::encryption::*;

pub struct Session {
    key: [u8; 32],
    db_conn: DbConn,
}

impl Session {
    pub fn new(passwd: &String, db_path: &str) -> Result<Session> {
        let key = derive_key(passwd.as_bytes())?;
        let db_conn = DbConn::new(db_path).expect("should create db connection");
        Ok(Session { key, db_conn })
    }

    pub fn insert_into_db(&self, label: &str, secret: &String) -> Result<()> {
        let encrypted = encrypt(&self.key, &secret.as_bytes())?;
        self.db_conn.insert_into_table(label, &encrypted)?;
        Ok(())
    }

    pub fn retrieve_from_db(&self, label: &str) -> Result<Option<String>> {
        let data = match self.db_conn.retrieve_data(label)? {
            Some(d) => d,
            _ => return Ok(None),
        };

        let secret_bytes = decrypt(&self.key, &data)?;
        let secret = String::from_utf8(secret_bytes)?;
        Ok(Some(secret))
    }

    pub fn retrieve_labels(&self) -> Result<Vec<String>> {
        Ok(self.db_conn.retrieve_labels()?)
    }

    pub fn close(self) -> Result<()> {
        self.db_conn.close()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{
        fs,
        path::PathBuf,
        sync::atomic::{AtomicUsize, Ordering},
        thread,
        time::Duration,
    };

    static TEST_ID: AtomicUsize = AtomicUsize::new(0);

    fn get_test_db_path() -> PathBuf {
        let test_id = TEST_ID.fetch_add(1, Ordering::SeqCst);
        let mut path = std::env::temp_dir();
        path.push(format!("lib_test_db_{test_id:?}.sqlite"));
        path
    }

    fn remove_db_with_retry(path: &PathBuf) {
        let mut attempts = 0;
        while attempts < 5 {
            if fs::remove_file(path).is_ok() {
                return;
            }

            attempts += 1;
            thread::sleep(Duration::from_millis(100));
        }
        fs::remove_file(path).expect("should delete the test database file");
    }

    #[test]
    fn can_store_and_retrieve_secret() {
        let db_path = get_test_db_path();

        let master_pass = "a$$word".to_string();
        let label = "mypass";
        let secret = "mysecret".to_string();

        let sess =
            Session::new(&master_pass, db_path.to_str().unwrap()).expect("should create session");

        sess.insert_into_db(&label, &secret)
            .expect("should insert secret into db");

        let retrieved_secret = sess
            .retrieve_from_db(&label)
            .expect("should retrieve secret from db")
            .unwrap();

        assert_eq!(secret, retrieved_secret);

        // cleanup
        sess.close().expect("should close session");
        remove_db_with_retry(&db_path);
    }

    #[test]
    fn can_retrieve_labels() {
        let db_path = get_test_db_path();

        let master_pass = "a$$word".to_string();
        let label1 = "mypass";
        let label2 = "mypass2";
        let label3 = "mypass3";
        let secret = "mysecret".to_string();

        let sess =
            Session::new(&master_pass, db_path.to_str().unwrap()).expect("should create session");

        sess.insert_into_db(&label1, &secret)
            .expect("should insert secret into db");
        sess.insert_into_db(&label2, &secret)
            .expect("should insert secret into db");
        sess.insert_into_db(&label3, &secret)
            .expect("should insert secret into db");

        let labels = sess.retrieve_labels().expect("should retrieve labels");

        assert!(labels.contains(&label1.to_string()));
        assert!(labels.contains(&label2.to_string()));
        assert!(labels.contains(&label3.to_string()));

        // cleanup
        sess.close().expect("should close session");
        remove_db_with_retry(&db_path);
    }
}

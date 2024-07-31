pub mod data;
pub mod encryption;

use anyhow::{anyhow, Result};

use crate::data::DbConn;
use crate::encryption::*;

pub struct Session {
    key: Option<[u8; 32]>,
    db_conn: Option<DbConn>,
}

pub struct SessionBuilder {
    key: Option<[u8; 32]>,
    db_conn: Option<DbConn>,
}

impl SessionBuilder {
    pub fn new() -> Self {
        SessionBuilder {
            key: None,
            db_conn: None,
        }
    }

    pub fn set_key(mut self, passwd: &String) -> Self {
        self.key = Some(derive_key(passwd.as_bytes()).expect("should derive key from password"));
        self
    }

    pub fn connect_to_db(mut self, db_path: &str) -> Self {
        self.db_conn = Some(DbConn::new(db_path).expect("should connect to db"));
        self
    }

    pub fn build(self) -> Session {
        Session {
            key: self.key,
            db_conn: self.db_conn,
        }
    }
}

impl Session {
    pub fn insert_into_db(&self, label: &str, secret: &String) -> Result<()> {
        let key = self
            .key
            .expect("cannot insert into db without setting a key first");

        let db = self
            .db_conn
            .as_ref()
            .ok_or(anyhow!("cannot insert before connecting to db"))?;

        let encrypted = encrypt(&key, &secret.as_bytes())?;
        db.insert_into_table(label, &encrypted)?;
        Ok(())
    }

    pub fn retrieve_from_db(&self, label: &str) -> Result<Option<String>> {
        let key = self
            .key
            .expect("cannot insert into db without setting a key first");

        let db = self
            .db_conn
            .as_ref()
            .ok_or(anyhow!("cannot insert before connecting to db"))?;

        let data = match db.retrieve_data(label)? {
            Some(d) => d,
            None => return Ok(None),
        };

        let secret_bytes = decrypt(&key, &data)?;
        let secret = String::from_utf8(secret_bytes)?;
        Ok(Some(secret))
    }

    pub fn retrieve_labels(&self) -> Result<Vec<String>> {
        let db = self
            .db_conn
            .as_ref()
            .ok_or(anyhow!("cannot insert before connecting to db"))?;

        Ok(db.retrieve_labels()?)
    }

    pub fn close(mut self) -> Result<()> {
        match self.db_conn {
            Some(db) => {
                db.close()?;
                self.db_conn = None;
                Ok(())
            }
            None => Ok(()),
        }
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

        let sess = SessionBuilder::new()
            .set_key(&master_pass)
            .connect_to_db(db_path.to_str().unwrap())
            .build();

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

        let sess = SessionBuilder::new()
            .set_key(&master_pass)
            .connect_to_db(db_path.to_str().unwrap())
            .build();

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

    #[test]
    #[should_panic]
    fn cannot_access_db_without_key() {
        let db_path = get_test_db_path();

        let label1 = "mypass";
        let secret = "mysecret".to_string();

        let sess = SessionBuilder::new()
            .connect_to_db(db_path.to_str().unwrap())
            .build();

        sess.insert_into_db(&label1, &secret)
            .expect("should insert secret into db");

        // cleanup
        sess.close().expect("should close session");
        remove_db_with_retry(&db_path);
    }

    #[test]
    #[should_panic]
    fn cannot_access_db_without_conn() {
        let db_path = get_test_db_path();

        let master_pass = "a$$word".to_string();
        let label1 = "mypass";
        let secret = "mysecret".to_string();

        let sess = SessionBuilder::new().set_key(&master_pass).build();

        sess.insert_into_db(&label1, &secret)
            .expect("should insert secret into db");

        // cleanup
        sess.close().expect("should close session");
        remove_db_with_retry(&db_path);
    }
}

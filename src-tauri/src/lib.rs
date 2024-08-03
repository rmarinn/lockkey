pub mod data;
pub mod encryption;

use anyhow::{anyhow, Result};
use data::{Kind, RetrieveLabelsQueryResult};

use crate::data::DbConn;
use crate::encryption::*;

pub struct Session {
    key: Option<[u8; 32]>,
    db_conn: Option<DbConn>,
}

impl Session {
    pub fn new() -> Self {
        Session {
            key: None,
            db_conn: None,
        }
    }

    fn get_key(&self) -> Result<&[u8; 32]> {
        match &self.key {
            Some(key) => Ok(key),
            None => Err(anyhow!("session does not have a key yet")),
        }
    }

    fn get_db_conn(&self) -> Result<&DbConn> {
        match &self.db_conn {
            Some(conn) => Ok(conn),
            None => Err(anyhow!("connection to db is not established yet")),
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

    pub fn insert_into_db(&self, kind: &str, label: &str, data: &str) -> Result<()> {
        let key = self.get_key()?;
        let db = self.get_db_conn()?;

        let encrypted = encrypt(&key, &data.as_bytes())?;
        db.insert_into_table(Kind::from_str(kind)?, label, &encrypted)?;
        Ok(())
    }

    pub fn retrieve_from_db(&self, label: &str) -> Result<Option<String>> {
        let key = self.get_key()?;
        let db = self.get_db_conn()?;

        let data = match db.retrieve_data(label)? {
            Some(d) => d,
            None => return Ok(None),
        };

        let secret_bytes = decrypt(&key, &data)?;
        let secret = String::from_utf8(secret_bytes)?;
        Ok(Some(secret))
    }

    pub fn retrieve_labels(&self) -> Result<Vec<RetrieveLabelsQueryResult>> {
        let db = self.get_db_conn()?;
        Ok(db.retrieve_labels()?)
    }

    pub fn delete_data(&self, label: &str) -> Result<()> {
        let db = self.get_db_conn()?;
        db.delete_data(label)?;
        Ok(())
    }

    pub fn close(mut self) -> Result<()> {
        if let Some(db) = self.db_conn.take() {
            db.close()?;
        }
        Ok(())
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        if let Some(db) = self.db_conn.take() {
            db.close().expect("should close db connection");
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

    struct TestDb {
        path: PathBuf,
    }

    impl TestDb {
        fn new() -> Self {
            static TEST_ID: AtomicUsize = AtomicUsize::new(0);
            let test_id = TEST_ID.fetch_add(1, Ordering::SeqCst);
            let mut path = std::env::temp_dir();
            path.push(format!("lockkey_lib_test_db_{test_id:?}.sqlite"));
            TestDb { path }
        }

        fn get_path(&self) -> &str {
            &self.path.to_str().unwrap()
        }
    }

    impl Drop for TestDb {
        fn drop(&mut self) {
            let mut attempts = 0;
            while attempts < 5 {
                if fs::remove_file(&self.path).is_ok() {
                    return;
                }

                attempts += 1;
                thread::sleep(Duration::from_millis(100));
            }
            fs::remove_file(&self.path).expect("should delete the tst database file");
        }
    }

    #[test]
    fn can_store_and_retrieve_secret() {
        let test_db = TestDb::new();
        let db_path = test_db.get_path();

        let master_pass = String::from("a$$word");
        let label = String::from("mypass");
        let secret = String::from("mysecret");

        let sess = Session::new().set_key(&master_pass).connect_to_db(db_path);

        sess.insert_into_db("password", &label, &secret)
            .expect("should insert secret into db");

        let retrieved_secret = sess
            .retrieve_from_db(&label)
            .expect("should retrieve secret from db")
            .unwrap();

        assert_eq!(secret, retrieved_secret);
    }

    #[test]
    fn can_retrieve_labels() {
        let test_db = TestDb::new();
        let db_path = test_db.get_path();

        let master_pass = String::from("a$$word");
        let label1 = String::from("mypass");
        let label2 = String::from("mypass2");
        let label3 = String::from("mypass3");
        let secret = String::from("mysecret");

        let sess = Session::new().set_key(&master_pass).connect_to_db(db_path);

        sess.insert_into_db("text", &label1, &secret)
            .expect("should insert secret into db");
        sess.insert_into_db("password", &label2, &secret)
            .expect("should insert secret into db");
        sess.insert_into_db("text", &label3, &secret)
            .expect("should insert secret into db");

        let labels = vec![label1, label2, label3];
        let query = sess.retrieve_labels().expect("should retrieve labels");

        for result in query {
            assert!(labels.contains(&result.label));
        }
    }

    #[test]
    #[should_panic]
    fn cannot_access_db_without_key() {
        let test_db = TestDb::new();
        let db_path = test_db.get_path();

        let label1 = String::from("mypass");
        let secret = String::from("mysecret");

        let sess = Session::new().connect_to_db(db_path);

        sess.insert_into_db("password", &label1, &secret)
            .expect("should insert secret into db");
    }

    #[test]
    #[should_panic]
    fn cannot_access_db_without_conn() {
        let master_pass = String::from("a$$word");
        let label1 = String::from("mypass");
        let secret = String::from("mysecret");

        let sess = Session::new().set_key(&master_pass);

        sess.insert_into_db("password", &label1, &secret)
            .expect("should insert secret into db");
    }

    #[test]
    fn should_return_empty_on_nonexistent_labels() {
        let test_db = TestDb::new();
        let db_path = test_db.get_path();

        let master_pass = String::from("a$$word");

        let sess = Session::new().set_key(&master_pass).connect_to_db(db_path);

        let data = sess
            .retrieve_from_db(&String::from("test"))
            .expect("should retrieve from db");
        assert_eq!(data, None);
    }

    #[test]
    fn can_delete_data() {
        let test_db = TestDb::new();
        let db_path = test_db.get_path();

        let master_pass = String::from("a$$word");
        let label = String::from("mypass");
        let secret = String::from("mysecret");

        let sess = Session::new().set_key(&master_pass).connect_to_db(db_path);

        sess.insert_into_db("password", &label, &secret)
            .expect("should insert secret into db");

        // check if data is inserted successfully
        let retrieved_secret = sess
            .retrieve_from_db(&label)
            .expect("should retrieve secret from db");
        assert_eq!(retrieved_secret, Some(secret));

        // delete data
        sess.delete_data(&label).expect("should delete data");

        // check if data is deleted
        let retrieved_secret = sess
            .retrieve_from_db(&label)
            .expect("should retrieve secret from db");
        assert_eq!(retrieved_secret, None);
    }
}

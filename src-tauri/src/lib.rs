pub mod data;
pub mod encryption;

use anyhow::{anyhow, Result};
use data::{Kind, RetrieveLabelsQueryResult};

use crate::data::DbConn;
use crate::encryption::*;

pub struct Session {
    passwd: Option<String>,
    db_conn: Option<DbConn>,
}

impl Session {
    pub fn new() -> Self {
        Session {
            passwd: None,
            db_conn: None,
        }
    }

    fn get_passwd(&self) -> Result<&String> {
        match &self.passwd {
            Some(pass) => Ok(pass),
            None => Err(anyhow!("session does not have a key yet")),
        }
    }

    fn get_db_conn(&self) -> Result<&DbConn> {
        match &self.db_conn {
            Some(conn) => Ok(conn),
            None => Err(anyhow!("connection to db is not established yet")),
        }
    }

    pub fn set_passwd(mut self, passwd: &String) -> Self {
        self.passwd = Some(passwd.clone());
        self
    }

    pub fn connect_to_db(mut self, db_path: &str) -> Self {
        self.db_conn = Some(DbConn::new(db_path).expect("should connect to db"));
        self
    }

    pub fn store_secret(&self, kind: &str, label: &str, data: &str) -> Result<()> {
        let passwd = self.get_passwd()?;
        let db = self.get_db_conn()?;

        let encrypted = encrypt(&passwd, &data)?;
        db.insert_into_table(Kind::from_str(kind)?, label, &encrypted)?;
        Ok(())
    }

    pub fn retrieve_secret(&self, label: &str) -> Result<Option<String>> {
        let key = self.get_passwd()?;
        let db = self.get_db_conn()?;

        let encrypted_secret = match db.retrieve_data(label)? {
            Some(d) => d,
            None => return Ok(None),
        };

        let secret = decrypt(&key, &encrypted_secret)?;
        Ok(Some(secret))
    }

    pub fn retrieve_labels(&self) -> Result<Vec<RetrieveLabelsQueryResult>> {
        let db = self.get_db_conn()?;
        Ok(db.retrieve_labels()?)
    }

    pub fn delete_secret(&self, label: &str) -> Result<()> {
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
        sync::{
            atomic::{AtomicUsize, Ordering},
            Arc, Mutex,
        },
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

        let passwd = String::from("a$$word");
        let label = String::from("mypass");
        let secret = String::from("mysecret");

        let sess = Session::new().set_passwd(&passwd).connect_to_db(&db_path);

        sess.store_secret("password", &label, &secret)
            .expect("should insert secret into db");

        let retrieved_secret = sess
            .retrieve_secret(&label)
            .expect("should retrieve secret from db");

        assert_eq!(Some(secret), retrieved_secret);
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

        let sess = Session::new()
            .set_passwd(&master_pass)
            .connect_to_db(db_path);

        sess.store_secret("text", &label1, &secret)
            .expect("should insert secret into db");
        sess.store_secret("password", &label2, &secret)
            .expect("should insert secret into db");
        sess.store_secret("text", &label3, &secret)
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

        sess.store_secret("password", &label1, &secret)
            .expect("should insert secret into db");
    }

    #[test]
    #[should_panic]
    fn cannot_access_db_without_conn() {
        let master_pass = String::from("a$$word");
        let label1 = String::from("mypass");
        let secret = String::from("mysecret");

        let sess = Session::new().set_passwd(&master_pass);

        sess.store_secret("password", &label1, &secret)
            .expect("should insert secret into db");
    }

    #[test]
    fn should_return_empty_on_nonexistent_labels() {
        let test_db = TestDb::new();
        let db_path = test_db.get_path();

        let master_pass = String::from("a$$word");

        let sess = Session::new()
            .set_passwd(&master_pass)
            .connect_to_db(db_path);

        let data = sess
            .retrieve_secret(&String::from("test"))
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

        let sess = Session::new()
            .set_passwd(&master_pass)
            .connect_to_db(db_path);

        sess.store_secret("password", &label, &secret)
            .expect("should insert secret into db");

        // check if data is inserted successfully
        let retrieved_secret = sess
            .retrieve_secret(&label)
            .expect("should retrieve secret from db");
        assert_eq!(retrieved_secret, Some(secret));

        // delete data
        sess.delete_secret(&label).expect("should delete data");

        // check if data is deleted
        let retrieved_secret = sess
            .retrieve_secret(&label)
            .expect("should retrieve secret from db");
        assert_eq!(retrieved_secret, None);
    }

    #[test]
    fn can_store_and_retrieve_in_arcmutex() {
        let test_db = TestDb::new();
        let db_path = test_db.get_path();

        let passwd = String::from("a$$word");
        let label = String::from("mypass");
        let secret = String::from("mysecret");

        let sess = Session::new().set_passwd(&passwd).connect_to_db(db_path);
        let sess = Arc::new(Mutex::new(sess));

        // acquire lock then store
        let s = sess.lock().expect("should acquire lock");
        s.store_secret("password", &label, &secret)
            .expect("should store secret");
        drop(s);

        // acquire lock then retreive
        let s = sess.lock().expect("should acquire lock");
        let retrieved_passwd = s.retrieve_secret(&label).expect("should retrieve password");
        drop(s);

        assert_eq!(retrieved_passwd, Some(secret));
    }
}

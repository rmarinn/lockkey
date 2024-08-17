mod auth;
mod data;
mod encryption;

use anyhow::{anyhow, Result};
use auth::{hash_password, verify_passwd};
use data::{Kind, RetrieveLabelsQueryResult};
use zeroize::Zeroize;

use crate::data::DbConn;
use crate::encryption::*;

pub struct Session {
    user_id: Option<i64>,
    key: Option<[u8; 32]>,
    db_conn: Option<DbConn>,
}

#[derive(serde::Serialize, Debug, PartialEq)]
pub struct Secret {
    label: String,
    data: String,
    kind: String,
}

impl Session {
    pub fn new() -> Self {
        Session {
            user_id: None,
            key: None,
            db_conn: None,
        }
    }

    fn get_user_id(&self) -> Result<i64> {
        match &self.user_id {
            Some(id) => Ok(*id),
            None => Err(anyhow!("session does not have a user_id yet")),
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

    pub fn is_authenticated(&self) -> bool {
        self.key.is_some()
    }

    pub fn create_user(&self, username: &str, mut passwd: String) -> Result<()> {
        let db = self.get_db_conn().expect("should get db connection");
        let enc_salt = generate_salt();
        let passwd_hash = hash_password(&passwd)?;
        db.create_user(username, &passwd_hash, &enc_salt)?;

        passwd.zeroize();
        Ok(())
    }

    pub fn delete_user(&self, mut passwd: String) -> Result<()> {
        let db = self.get_db_conn().expect("should get db connection");

        let usrname = match db.get_username(&self.get_user_id()?)? {
            Some(usrname) => usrname,
            None => return Err(anyhow!("user does not exist")),
        };

        let stored_hash = match db.get_user_passwd_hash(&usrname)? {
            Some(hash) => hash,
            None => return Err(anyhow!("user does not have a stored hash")),
        };

        if !verify_passwd(&passwd, &stored_hash)? {
            return Err(anyhow!("invalid password"));
        }

        db.delete_user(&usrname)?;

        passwd.zeroize();

        Ok(())
    }

    pub fn authenticate_user(&mut self, usrname: &str, mut passwd: String) -> Result<()> {
        let db = self.get_db_conn().expect("should get db connection");

        let passwd_hash = match db.get_user_passwd_hash(&usrname)? {
            Some(hash) => hash,
            None => return Err(anyhow!("invalid username or password")),
        };

        if !verify_passwd(&passwd, &passwd_hash)? {
            return Err(anyhow!("invalid username or password"));
        }

        let enc_key = match db.get_user_enc_salt(usrname)? {
            Some(salt) => Some(derive_encryption_key(&passwd, &salt)?),
            None => return Err(anyhow!("user has missing data")),
        };

        let user_id = match db.get_user_id(&usrname)? {
            Some(id) => Some(id),
            None => return Err(anyhow!("user has missing data")),
        };

        self.key = enc_key;
        self.user_id = user_id;

        passwd.zeroize();

        Ok(())
    }

    pub fn connect_to_db(mut self, db_path: &str) -> Self {
        self.db_conn = Some(DbConn::new(db_path).expect("should connect to db"));
        self
    }

    pub fn store_secret(&self, kind: &str, label: &str, data: String) -> Result<()> {
        let key = self.get_key()?;
        let db = self.get_db_conn()?;

        let encrypted = encrypt_using_key(key, &data)?;
        db.store_secret(self.get_user_id()?, Kind::from_str(kind)?, label, encrypted)?;
        Ok(())
    }

    pub fn edit_secret(&self, label: &str, data: String) -> Result<()> {
        let key = self.get_key()?;
        let db = self.get_db_conn()?;

        let encrypted = encrypt_using_key(key, &data)?;
        db.edit_secret(self.get_user_id()?, label, encrypted)?;
        Ok(())
    }

    pub fn retrieve_secret(&self, label: &str) -> Result<Option<Secret>> {
        let key = self.get_key()?;
        let db = self.get_db_conn()?;

        let (kind, encrypted_data) = match db.get_secret(self.get_user_id()?, label)? {
            Some(d) => d,
            None => return Ok(None),
        };

        let decrypted_data = decrypt_using_key(key, encrypted_data)?;
        let secret = Secret {
            label: label.into(),
            kind,
            data: decrypted_data,
        };
        Ok(Some(secret))
    }

    pub fn retrieve_labels(&self) -> Result<Vec<RetrieveLabelsQueryResult>> {
        let db = self.get_db_conn()?;
        Ok(db.get_labels(self.get_user_id()?)?)
    }

    pub fn delete_secret(&self, label: &str) -> Result<()> {
        let db = self.get_db_conn()?;
        db.delete_secret(&self.get_user_id()?, label)?;
        Ok(())
    }

    pub fn logout(&mut self) -> Result<()> {
        self.user_id = None;
        self.key.zeroize();
        self.key = None;
        Ok(())
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        if let Some(db) = self.db_conn.take() {
            db.close().expect("should close db connection");
        }
        self.key.zeroize();
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

        let username = String::from("test_user");
        let passwd = String::from("test_pass");
        let label = String::from("mypass");
        let secret = String::from("mysecret");

        let mut sess = Session::new().connect_to_db(&db_path);
        sess.create_user(&username, passwd.clone()).unwrap();
        sess.authenticate_user(&username, passwd.clone()).unwrap();

        sess.store_secret("password", &label, secret.to_string())
            .expect("should insert secret into db");

        let retrieved_secret = sess.retrieve_secret(&label).unwrap();

        assert_eq!(
            Some(Secret {
                label,
                kind: "password".to_string(),
                data: secret
            }),
            retrieved_secret
        );
    }

    #[test]
    fn can_retrieve_labels() {
        let test_db = TestDb::new();
        let db_path = test_db.get_path();

        let username = String::from("test_user");
        let passwd = String::from("test_pass");
        let label1 = String::from("mypass");
        let label2 = String::from("mypass2");
        let label3 = String::from("mypass3");
        let secret = String::from("mysecret");

        let mut sess = Session::new().connect_to_db(db_path);
        sess.create_user(&username, passwd.clone()).unwrap();
        sess.authenticate_user(&username.to_string(), passwd)
            .unwrap();

        sess.store_secret("text", &label1, secret.to_string())
            .expect("should insert secret into db");
        sess.store_secret("password", &label2, secret.to_string())
            .expect("should insert secret into db");
        sess.store_secret("text", &label3, secret.to_string())
            .expect("should insert secret into db");

        let labels = vec![label1.to_string(), label2.to_string(), label3.to_string()];
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

        let label1 = "mypass";
        let secret = "mysecret";

        let sess = Session::new().connect_to_db(db_path);

        sess.store_secret("password", &label1, secret.to_string())
            .expect("should insert secret into db");
    }

    #[test]
    #[should_panic]
    fn cannot_access_db_without_conn() {
        let label1 = String::from("mypass");
        let secret = String::from("mysecret");
        let username = String::from("test_user");
        let passwd = String::from("test_pass");

        let mut sess = Session::new();
        sess.create_user(&username, passwd.clone()).unwrap();
        sess.authenticate_user(&username.to_string(), passwd)
            .unwrap();

        sess.store_secret("password", &label1, secret.to_string())
            .unwrap();
    }

    #[test]
    fn should_return_empty_on_nonexistent_labels() {
        let test_db = TestDb::new();
        let db_path = test_db.get_path();
        let username = String::from("test_user");
        let passwd = String::from("test_pass");

        let mut sess = Session::new().connect_to_db(db_path);
        sess.create_user(&username, passwd.clone()).unwrap();
        sess.authenticate_user(&username.to_string(), passwd)
            .unwrap();

        let data = sess.retrieve_secret("test").unwrap();
        assert_eq!(data, None);
    }

    #[test]
    fn can_delete_data() {
        let test_db = TestDb::new();
        let db_path = test_db.get_path();

        let username = String::from("test_user");
        let passwd = String::from("test_pass");
        let label = String::from("mypass");
        let secret = String::from("mysecret");

        let mut sess = Session::new().connect_to_db(db_path);
        sess.create_user(&username, passwd.clone()).unwrap();
        sess.authenticate_user(&username.to_string(), passwd)
            .unwrap();

        sess.store_secret("password", &label, secret.to_string())
            .expect("should insert secret into db");

        // check if data is inserted successfully
        let retrieved_secret = sess
            .retrieve_secret(&label)
            .expect("should retrieve secret from db");

        assert_eq!(
            retrieved_secret,
            Some(Secret {
                label: label.clone(),
                kind: "password".to_string(),
                data: secret
            })
        );

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

        let username = String::from("user");
        let passwd = String::from("test_pass");
        let label = String::from("mypass");
        let secret = String::from("mysecret");

        let mut sess = Session::new().connect_to_db(db_path);
        sess.create_user(&username, passwd.clone()).unwrap();
        sess.authenticate_user(&username.to_string(), passwd)
            .unwrap();

        let sess = Arc::new(Mutex::new(sess));

        // acquire lock then store
        let s = sess.lock().expect("should acquire lock");
        s.store_secret("password", &label, secret.to_string())
            .expect("should store secret");
        drop(s);

        // acquire lock then retreive
        let s = sess.lock().expect("should acquire lock");
        let retrieved_secret = s.retrieve_secret(&label).expect("should retrieve password");
        drop(s);

        assert_eq!(
            retrieved_secret,
            Some(Secret {
                label: label,
                kind: "password".to_string(),
                data: secret
            })
        );
    }
}

mod auth;
mod data;
mod encryption;

use std::time::Instant;

use anyhow::{anyhow, Result};
use auth::{hash_password, verify_passwd};
use data::{Kind, RetrieveLabelsQueryResult};
use zeroize::Zeroize;

use crate::data::DbConn;
use crate::encryption::*;

pub struct Session {
    user_id: i64,
    key: [u8; 32],
    db_conn: DbConn,
    pub last_activity: Instant,
}

#[derive(serde::Serialize, Debug, PartialEq)]
pub struct Secret {
    label: String,
    data: String,
    kind: String,
}

pub fn create_new_account(username: &str, mut passwd: String, db_path: &str) -> Result<()> {
    let db_conn = DbConn::new(db_path)?;
    let enc_salt = generate_salt();
    let passwd_hash = hash_password(&passwd)?;
    db_conn.create_user(username, &passwd_hash, &enc_salt)?;

    passwd.zeroize();
    Ok(())
}

impl Session {
    pub fn new(usrname: &str, mut passwd: String, db_path: &str) -> Result<Session> {
        let db_conn = DbConn::new(db_path)?;

        let passwd_hash = match db_conn.get_user_passwd_hash(&usrname)? {
            Some(hash) => hash,
            None => return Err(anyhow!("invalid username or password")),
        };

        if !verify_passwd(&passwd, &passwd_hash)? {
            return Err(anyhow!("invalid username or password"));
        }

        let key = match db_conn.get_user_enc_salt(usrname)? {
            Some(salt) => derive_encryption_key(&passwd, &salt)?,
            None => return Err(anyhow!("user has missing data")),
        };

        let user_id = match db_conn.get_user_id(&usrname)? {
            Some(id) => id,
            None => return Err(anyhow!("user has missing data")),
        };

        passwd.zeroize();

        Ok(Session {
            user_id,
            key,
            db_conn,
            last_activity: Instant::now(),
        })
    }

    pub fn delete_user(&self, mut passwd: String) -> Result<()> {
        let usrname = match self.db_conn.get_username(&self.user_id)? {
            Some(usrname) => usrname,
            None => return Err(anyhow!("user does not exist")),
        };

        let stored_hash = match self.db_conn.get_user_passwd_hash(&usrname)? {
            Some(hash) => hash,
            None => return Err(anyhow!("user does not have a stored hash")),
        };

        if !verify_passwd(&passwd, &stored_hash)? {
            return Err(anyhow!("invalid password"));
        }

        self.db_conn.delete_user(&usrname)?;

        passwd.zeroize();

        Ok(())
    }

    pub fn store_secret(&self, kind: &str, label: &str, mut data: String) -> Result<()> {
        let encrypted = encrypt_using_key(&self.key, &data)?;
        self.db_conn
            .store_secret(self.user_id, Kind::from_str(kind)?, label, encrypted)?;
        data.zeroize();
        Ok(())
    }

    pub fn edit_secret(&self, label: &str, new_label: &str, new_data: String) -> Result<()> {
        let encrypted = encrypt_using_key(&self.key, &new_data)?;
        self.db_conn
            .edit_secret(self.user_id, label, new_label, encrypted)?;
        Ok(())
    }

    pub fn retrieve_secret(&self, label: &str) -> Result<Option<Secret>> {
        let (kind, encrypted_data) = match self.db_conn.get_secret(self.user_id, label)? {
            Some(d) => d,
            None => return Ok(None),
        };

        let decrypted_data = decrypt_using_key(&self.key, encrypted_data)?;
        let secret = Secret {
            label: label.into(),
            kind,
            data: decrypted_data,
        };
        Ok(Some(secret))
    }

    pub fn retrieve_labels(&self) -> Result<Vec<RetrieveLabelsQueryResult>> {
        Ok(self.db_conn.get_labels(self.user_id)?)
    }

    pub fn delete_secret(&self, label: &str) -> Result<()> {
        self.db_conn.delete_secret(self.user_id, label)?;
        Ok(())
    }

    pub fn logout(&mut self) -> Result<()> {
        self.key.zeroize();
        Ok(())
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        self.db_conn.close().expect("should close db connection");
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

        create_new_account(&username, passwd.clone(), db_path).unwrap();
        let sess = Session::new(&username, passwd.clone(), db_path).unwrap();

        sess.store_secret("password", &label, secret.to_string())
            .unwrap();

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

        create_new_account(&username, passwd.clone(), db_path).unwrap();
        let sess = Session::new(&username, passwd.clone(), db_path).unwrap();

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
    fn should_return_empty_on_nonexistent_labels() {
        let test_db = TestDb::new();
        let db_path = test_db.get_path();
        let username = String::from("test_user");
        let passwd = String::from("test_pass");

        create_new_account(&username, passwd.clone(), db_path).unwrap();
        let sess = Session::new(&username, passwd.clone(), db_path).unwrap();

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

        create_new_account(&username, passwd.clone(), db_path).unwrap();
        let sess = Session::new(&username, passwd.clone(), db_path).unwrap();

        sess.store_secret("password", &label, secret.to_string())
            .unwrap();

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

        create_new_account(&username, passwd.clone(), db_path).unwrap();
        let sess = Session::new(&username, passwd.clone(), db_path).unwrap();

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

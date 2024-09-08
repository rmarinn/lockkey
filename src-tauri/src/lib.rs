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

/// Represents a user session, including the user's ID, encryption key,
/// database connection, and timestamp of the last activity.
pub struct Session {
    user_id: i64,
    key: [u8; 32],
    db_conn: DbConn,
    pub last_activity: Instant,
}

/// Represents a stored secret, containing a label, encrypted data, and its type.
#[derive(serde::Serialize, Debug, PartialEq)]
pub struct Secret {
    label: String,
    data: String,
    kind: String,
}

/// Creates a new user account by storing a username and password in the database.
///
/// # Arguments
/// * `username` - The username of the new account.
/// * `passwd` - The password associated with the account (will be hashed and zeroized after use).
/// * `db_path` - The path to the database where user data is stored.
///
/// # Returns
/// * `Result<()>` - An empty result if the operation is successful, otherwise an error.
pub fn create_new_account(username: &str, mut passwd: String, db_path: &str) -> Result<()> {
    let db_conn = DbConn::new(db_path)?;
    let enc_salt = generate_salt();
    let passwd_hash = hash_password(&passwd)?;
    db_conn.create_user(username, &passwd_hash, &enc_salt)?;

    passwd.zeroize();
    Ok(())
}

impl Session {
    /// Creates a new session for an existing user by verifying the password and retrieving the encryption key.
    ///
    /// # Arguments
    /// * `usrname` - The username of the user.
    /// * `passwd` - The user's password (will be zeroized after use).
    /// * `db_path` - The path to the database where user data is stored.
    ///
    /// # Returns
    /// * `Result<Session>` - A new session if the login is successful, otherwise an error.
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

    /// Deletes the current user from the database after verifying the password.
    ///
    /// # Arguments
    /// * `passwd` - The user's password to verify the deletion (will be zeroized after use).
    ///
    /// # Returns
    /// * `Result<()>` - An empty result if the operation is successful, otherwise an error.
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

    /// Encrypts and stores a secret in the database for the current user.
    ///
    /// # Arguments
    /// * `kind` - The type of the secret (e.g., password, text, etc.).
    /// * `label` - The label for the secret.
    /// * `data` - The secret data to be encrypted and stored (will be zeroized after use).
    ///
    /// # Returns
    /// * `Result<()>` - An empty result if the operation is successful, otherwise an error.
    pub fn store_secret(&self, kind: &str, label: &str, mut data: String) -> Result<()> {
        let encrypted = encrypt_using_key(&self.key, &data)?;
        self.db_conn
            .store_secret(self.user_id, Kind::from_str(kind)?, label, encrypted)?;
        data.zeroize();
        Ok(())
    }

    /// Updates an existing secret in the database with a new label or data.
    ///
    /// # Arguments
    /// * `label` - The current label of the secret to be updated.
    /// * `new_label` - The new label for the secret.
    /// * `new_data` - The new secret data (will be encrypted).
    ///
    /// # Returns
    /// * `Result<()>` - An empty result if the operation is successful, otherwise an error.
    pub fn edit_secret(&self, label: &str, new_label: &str, new_data: String) -> Result<()> {
        let encrypted = encrypt_using_key(&self.key, &new_data)?;
        self.db_conn
            .edit_secret(self.user_id, label, new_label, encrypted)?;
        Ok(())
    }

    /// Retrieves and decrypts a secret by its label.
    ///
    /// # Arguments
    /// * `label` - The label of the secret to retrieve.
    ///
    /// # Returns
    /// * `Result<Option<Secret>>` - The decrypted secret if found, otherwise `None`.
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

    /// Retrieves a list of labels for all stored secrets belonging to the current user.
    ///
    /// # Returns
    /// * `Result<Vec<RetrieveLabelsQueryResult>>` - A list of labels for the user's stored
    /// secrets.
    pub fn retrieve_labels(&self) -> Result<Vec<RetrieveLabelsQueryResult>> {
        Ok(self.db_conn.get_labels(self.user_id)?)
    }

    /// Deletes a secret from the database by its label.
    ///
    /// # Arguments
    /// * `label` - The label of the secret to be deleted.
    ///
    /// # Returns
    /// * `Result<()>` - An empty result if the operation is successful, otherwise an error.
    pub fn delete_secret(&self, label: &str) -> Result<()> {
        self.db_conn.delete_secret(self.user_id, label)?;
        Ok(())
    }

    /// Logs the user out by zeroizing the encryption key.
    ///
    /// # Returns
    /// * `Result<()>` - An empty result indicating the user has been logged out.
    pub fn logout(&mut self) -> Result<()> {
        self.key.zeroize();
        Ok(())
    }
}

impl Drop for Session {
    /// Ensures the database connection is closed and the encryption key is zeroized when the session is dropped.
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

    /// A struct representing a test database that generates a unique temporary SQLite file.
    struct TestDb {
        path: PathBuf,
    }

    impl TestDb {
        /// Creates a new test database with a unique file path.
        /// This uses an atomic counter to ensure uniqueness across test runs.
        fn new() -> Self {
            static TEST_ID: AtomicUsize = AtomicUsize::new(0);
            let test_id = TEST_ID.fetch_add(1, Ordering::SeqCst);
            let mut path = std::env::temp_dir();
            path.push(format!("lockkey_lib_test_db_{test_id:?}.sqlite"));
            TestDb { path }
        }

        /// Returns the file path of the test database as a string slice.
        fn get_path(&self) -> &str {
            &self.path.to_str().unwrap()
        }
    }

    impl Drop for TestDb {
        /// Deletes the test database file when the struct goes out of scope.
        /// This attempts to delete the file up to five times if the operation initially fails.
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

    /// Test to verify that a secret can be stored and retrieved successfully.
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

    /// Test to verify that labels for stored secrets can be retrieved.
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

    /// Test to verify that retrieving a non-existent secret returns `None`.
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

    /// Test to verify that a secret can be deleted successfully.
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

    /// Test to verify that secrets can be stored and retrieved within an `Arc<Mutex<Session>>`,
    /// ensuring thread safety when accessing the session.
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

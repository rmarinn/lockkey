use anyhow::{anyhow, Result};

use rusqlite::{params, Connection, OptionalExtension, Transaction};

/// Struct representing a connection to the SQLite database.
pub struct DbConn {
    conn: Option<Connection>,
}

/// Struct used for retrieving labels from the database.
pub struct RetrieveLabelsQueryResult {
    pub kind: String,
    pub label: String,
}

/// Enum representing the kind of secret (either password or text).
#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Password,
    Text,
}

/// Converts a string to a `Kind` enum.
///
/// # Arguments
///
/// * `input` - A string slice that holds the kind value.
///
/// # Errors
///
/// Returns an error if the input does not match a valid kind.
impl Kind {
    pub fn from_str(input: &str) -> Result<Kind> {
        match input {
            "password" => Ok(Kind::Password),
            "text" => Ok(Kind::Text),
            _ => Err(anyhow!("invalid kind: {:?}", input)),
        }
    }

    /// Converts the `Kind` enum to a string.
    ///
    /// # Returns
    ///
    /// A string representation of the `Kind`.
    pub fn to_str(self) -> String {
        match self {
            Kind::Password => String::from("password"),
            Kind::Text => String::from("text"),
        }
    }
}

impl DbConn {
    /// Creates a new database connection and initializes the required tables.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice that holds the path to the SQLite database file.
    ///
    /// # Errors
    ///
    /// Returns an error if the connection cannot be established or table creation fails.
    pub fn new(path: &str) -> Result<DbConn> {
        let mut conn = DbConn {
            conn: Some(Connection::open(path)?),
        };
        conn.create_table()?;
        Ok(conn)
    }

    /// Returns a reference to the active SQLite connection.
    ///
    /// # Errors
    ///
    /// Returns an error if no connection is available.
    fn get_conn(&self) -> Result<&Connection> {
        match &self.conn {
            Some(conn) => Ok(conn),
            None => Err(anyhow!("does not have connection to the dabase")),
        }
    }

    /// Starts a new transaction in the current SQLite connection.
    ///
    /// # Errors
    ///
    /// Returns an error if the transaction cannot be started or the connection is unavailable.
    fn start_transaction(&mut self) -> Result<Transaction> {
        match &mut self.conn {
            Some(conn) => Ok(conn.transaction()?),
            None => Err(anyhow!("does not have connection to the dabase")),
        }
    }

    /// Creates the required tables (`users` and `secrets`) in the database if they do not already exist.
    ///
    /// # Errors
    ///
    /// Returns an error if the table creation queries fail.
    fn create_table(&mut self) -> Result<()> {
        let conn = self.start_transaction()?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                user_id     INTEGER PRIMARY KEY,
                username    TEXT UNIQUE NOT NULL CHECK(length(username) <= 24),
                passwd_hash TEXT NOT NULL,
                enc_salt    BLOB NOT NULL
            );
            ",
            (),
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS secrets (
                id          INTEGER PRIMARY KEY,
                user_id     INTEGER,
                kind        TEXT NOT NULL CHECK(kind IN ('text', 'password')),
                label       TEXT NOT NULL CHECK(length(label) >= 3 AND length(label) <= 32),
                data        BLOB NOT NULL CHECK(length(data) > 3),
                FOREIGN KEY(user_id) REFERENCES users(user_id)
            );
            CREATE INDEX idx_secrets_user_id ON secrets (user_id);
            ",
            (),
        )?;

        conn.commit()?;
        Ok(())
    }

    /// Checks if a user with the given username already exists in the database.
    ///
    /// # Arguments
    ///
    /// * `username` - A string slice representing the username to check.
    ///
    /// # Errors
    ///
    /// Returns an error if the query fails.
    fn check_user_already_exists(&self, username: &str) -> Result<bool> {
        let conn = self.get_conn()?;

        let mut stmt = conn.prepare("SELECT COUNT(*) FROM users WHERE username = ?1;")?;
        let count: i64 = stmt.query_row(params![username], |row| row.get(0))?;

        Ok(count > 0)
    }

    /// Creates a new user in the database with the provided username, password hash, and encryption salt.
    ///
    /// # Arguments
    ///
    /// * `username` - A string slice representing the username.
    /// * `passwd_hash` - A string slice representing the password hash.
    /// * `enc_salt` - A byte slice representing the encryption salt.
    ///
    /// # Errors
    ///
    /// Returns an error if the user already exists or if the query fails.
    pub fn create_user(&self, username: &str, passwd_hash: &str, enc_salt: &[u8]) -> Result<()> {
        let conn = self.get_conn()?;

        if self.check_user_already_exists(username)? {
            return Err(anyhow!("The username, {username:?}, is already taken"));
        }

        conn.execute(
            "INSERT INTO users (username, passwd_hash, enc_salt) VALUES (?1, ?2, ?3)",
            (username, passwd_hash, enc_salt),
        )?;

        Ok(())
    }

    /// Retrieves the user ID associated with the given username.
    ///
    /// # Arguments
    ///
    /// * `username` - A string slice representing the username.
    ///
    /// # Errors
    ///
    /// Returns an error if the query fails.
    pub fn get_user_id(&self, username: &str) -> Result<Option<i64>> {
        let conn = self.get_conn()?;

        let mut stmt = conn.prepare("SELECT user_id FROM users WHERE username = ?1")?;
        let user_id: Option<i64> = stmt.query_row([username], |row| row.get(0)).optional()?;

        Ok(user_id)
    }

    /// Retrieves the username associated with the given user ID.
    ///
    /// # Arguments
    ///
    /// * `user_id` - A reference to the user ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the query fails.
    pub fn get_username(&self, user_id: &i64) -> Result<Option<String>> {
        let conn = self.get_conn()?;

        let mut stmt = conn.prepare("SELECT username FROM users WHERE user_id = ?1")?;
        let user_id: Option<String> = stmt.query_row([user_id], |row| row.get(0)).optional()?;

        Ok(user_id)
    }

    /// Retrieves the password hash associated with the given username.
    ///
    /// # Arguments
    ///
    /// * `username` - A string slice representing the username.
    ///
    /// # Errors
    ///
    /// Returns an error if the query fails.
    pub fn get_user_passwd_hash(&self, username: &str) -> Result<Option<String>> {
        let conn = self.get_conn()?;

        let mut stmt = conn.prepare("SELECT passwd_hash FROM users WHERE username = ?1")?;
        let db_passwd_hash: Option<String> =
            stmt.query_row([username], |row| row.get(0)).optional()?;

        Ok(db_passwd_hash)
    }

    /// Retrieves the encryption salt associated with the given username.
    ///
    /// # Arguments
    ///
    /// * `username` - A string slice representing the username.
    ///
    /// # Errors
    ///
    /// Returns an error if the query fails.
    pub fn get_user_enc_salt(&self, username: &str) -> Result<Option<Vec<u8>>> {
        let conn = self.get_conn()?;

        let mut stmt = conn.prepare("SELECT enc_salt FROM users WHERE username = ?1;")?;
        let enc_salt: Option<Vec<u8>> = stmt.query_row([username], |row| row.get(0)).optional()?;

        Ok(enc_salt)
    }

    pub fn delete_user(&self, username: &str) -> Result<()> {
        let conn = self.get_conn()?;
        let mut stmt = conn.prepare("DELETE FROM users WHERE username = ?1")?;
        stmt.execute([username])?;
        Ok(())
    }

    /// Deletes a user from the database based on the given username.
    ///
    /// # Arguments
    ///
    /// * `username` - A string slice representing the username.
    ///
    /// # Errors
    ///
    /// Returns an error if the deletion fails.
    fn check_if_secret_label_exists(&self, user_id: i64, label: &str) -> Result<bool> {
        let conn = self.get_conn()?;

        let mut stmt =
            conn.prepare("SELECT COUNT(*) FROM secrets WHERE user_id = ?1 AND label = ?2;")?;
        let count: i64 = stmt.query_row(params![user_id, label], |row| row.get(0))?;

        Ok(count > 0)
    }

    /// Checks if a secret with the given label already exists for the specified user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID.
    /// * `label` - A string slice representing the label of the secret.
    ///
    /// # Errors
    ///
    /// Returns an error if the query fails.
    pub fn store_secret(&self, user_id: i64, kind: Kind, label: &str, data: Vec<u8>) -> Result<()> {
        let conn = self.get_conn()?;

        if self.check_if_secret_label_exists(user_id, label)? {
            return Err(anyhow!("A secret with the same label already exists"));
        }

        let mut stmt = conn
            .prepare("INSERT INTO secrets (user_id, kind, label, data) VALUES (?1, ?2, ?3, ?4);")?;
        stmt.execute(params![user_id, kind.to_str(), label, data])?;

        Ok(())
    }

    pub fn edit_secret(
        &self,
        user_id: i64,
        label: &str,
        new_label: &str,
        new_data: Vec<u8>,
    ) -> Result<()> {
        let conn = self.get_conn()?;

        let mut stmt = conn.prepare(
            "UPDATE OR ABORT secrets SET label = ?1, data = ?2 WHERE user_id = ?3 AND label = ?4;",
        )?;
        stmt.execute(params![new_label, new_data, user_id, label])?;

        Ok(())
    }

    pub fn get_labels(&self, user_id: i64) -> Result<Vec<RetrieveLabelsQueryResult>> {
        let conn = self.get_conn()?;

        let mut stmt = conn.prepare("SELECT kind, label FROM secrets WHERE user_id = ?1")?;
        let rows = stmt.query_map([user_id], |row| Ok((row.get(0)?, row.get(1)?)))?;

        let mut labels: Vec<RetrieveLabelsQueryResult> = Vec::new();
        for row in rows {
            match row {
                Ok((kind, label)) => labels.push(RetrieveLabelsQueryResult { kind, label }),
                _ => (),
            }
        }
        Ok(labels)
    }

    pub fn get_secret(&self, user_id: i64, label: &str) -> Result<Option<(String, Vec<u8>)>> {
        let conn = self.get_conn()?;

        let mut stmt =
            conn.prepare("SELECT kind, data FROM secrets WHERE user_id = ?1 AND label = ?2")?;

        let data = stmt
            .query_row([user_id.to_string(), label.to_string()], |row| {
                Ok((row.get(0)?, row.get(1)?))
            })
            .optional()?;
        Ok(data)
    }

    pub fn delete_secret(&self, user_id: i64, label: &str) -> Result<()> {
        let conn = self.get_conn()?;

        let mut stmt = conn.prepare("DELETE FROM secrets WHERE user_id == ?1 AND label = ?2;")?;
        stmt.execute([user_id.to_string(), label.to_string()])?;
        Ok(())
    }

    pub fn close(&mut self) -> Result<()> {
        if let Some(conn) = self.conn.take() {
            conn.close().expect("should close db connection");
        }
        Ok(())
    }
}

impl Drop for DbConn {
    fn drop(&mut self) {
        if let Some(conn) = self.conn.take() {
            conn.close().expect("should close db connection");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{
        collections::HashMap,
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
            path.push(format!("lockkey_data_test_db_{test_id:?}.sqlite"));
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
            fs::remove_file(&self.path).expect("should delete the test database file");
        }
    }

    #[test]
    fn can_create_db_connection() {
        let test_db = TestDb::new();
        let db_path = test_db.get_path();

        DbConn::new(db_path).unwrap();
    }

    #[test]
    fn can_insert_and_retrieve_data_from_table() {
        let test_db = TestDb::new();
        let db_path = test_db.get_path();
        let conn = DbConn::new(db_path).unwrap();

        // setup credentials
        let username = "test_user";
        let passwd = "test_pass";
        let enc_salt = b"test_salt";

        conn.create_user(username, passwd, enc_salt).unwrap();

        let passwd: Vec<u8> = "passwd".into();
        let label1 = "pass1".to_string();
        conn.store_secret(1, Kind::Password, &label1, passwd.clone())
            .unwrap();

        let label2 = "pass2".to_string();
        conn.store_secret(1, Kind::Text, &label2, passwd.clone())
            .unwrap();

        // getting back the data
        let (kind1, data1) = conn.get_secret(1, &label1).unwrap().unwrap();
        assert_eq!(&kind1, &Kind::Password.to_str());
        assert_eq!(&data1, &passwd);

        let (kind2, data2) = conn.get_secret(1, &label2).unwrap().unwrap();
        assert_eq!(&kind2, &Kind::Text.to_str());
        assert_eq!(&data2, &passwd);
    }

    #[test]
    fn can_retrieve_labels() {
        let test_db = TestDb::new();
        let db_path = test_db.get_path();
        let conn = DbConn::new(db_path).unwrap();

        // setup credentials
        let username = "test_user";
        let passwd = "test_pass";
        let enc_salt = b"test_salt";
        conn.create_user(username, passwd, enc_salt).unwrap();

        let passwd: Vec<u8> = "passwd".into();
        let label1 = "pass1".to_string();
        let kind1 = Kind::Password;
        conn.store_secret(1, kind1.clone(), &label1, passwd.clone())
            .expect("should insert into table");

        let label2 = "pass2".to_string();
        let kind2 = Kind::Text;
        conn.store_secret(1, kind2.clone(), &label2, passwd.clone())
            .expect("should insert into table");

        let query = conn.get_labels(1).expect("should retrieve labels");

        assert_eq!(query.len(), 2);

        let mut inputs = HashMap::new();
        inputs.insert(label1, kind1.to_str());
        inputs.insert(label2, kind2.to_str());

        for result in query {
            assert!(inputs.contains_key(&result.label));
            assert_eq!(inputs.get(&result.label), Some(&result.kind));
        }
    }

    #[test]
    #[should_panic]
    fn should_have_unique_labels() {
        let test_db = TestDb::new();
        let db_path = test_db.get_path();
        let conn = DbConn::new(db_path).unwrap();

        conn.store_secret(1, Kind::Password, &String::from("pass1"), b"pass1".to_vec())
            .unwrap();
        conn.store_secret(1, Kind::Text, &String::from("pass1"), b"pass2".to_vec())
            .unwrap();
    }

    #[test]
    fn can_delete_data_from_table() {
        let test_db = TestDb::new();
        let db_path = test_db.get_path();

        let conn = DbConn::new(db_path).unwrap();

        // setup credentials
        let username = "test_user";
        let passwd = "test_pass";
        let enc_salt = b"test_salt";
        conn.create_user(username, passwd, enc_salt).unwrap();

        let passwd: Vec<u8> = "passwd".into();
        let label = "pass1".to_string();
        conn.store_secret(1, Kind::Password, &label, passwd.clone())
            .expect("should insert into table");

        // check if data is inserted
        let data = conn.get_secret(1, &label).unwrap();
        assert_eq!(data, Some((Kind::Password.to_str(), passwd)));

        conn.delete_secret(1, &label).expect("should delete data");

        // try to get data again
        let data = conn.get_secret(1, &label).unwrap();
        assert_eq!(data, None);
    }

    #[test]
    fn should_get_empty_when_fetching_nonexistent_data() {
        let test_db = TestDb::new();
        let db_path = test_db.get_path();

        let conn = DbConn::new(db_path).unwrap();

        let data = conn.get_secret(1, &String::from("data_label")).unwrap();
        assert_eq!(data, None);
    }

    #[test]
    fn can_create_and_delete_user() {
        let test_db = TestDb::new();
        let db_path = test_db.get_path();

        let conn = DbConn::new(db_path).unwrap();

        let username1 = "test_user1";
        let username2 = "test_user2";
        let passwd_hash = "test_pass";
        let salt = b"salt";
        conn.create_user(username1, &passwd_hash, salt).unwrap();
        conn.create_user(username2, &passwd_hash, salt).unwrap();

        // check if user is created
        let retrieved_hash = conn.get_user_passwd_hash(&username2).unwrap();
        let user_id = conn.get_user_id(username2).unwrap();
        assert_eq!(retrieved_hash, Some(passwd_hash.to_string()));
        assert_eq!(user_id, Some(2));

        // delete user
        conn.delete_user(&username2).unwrap();
        let retrieved_hash = conn.get_user_passwd_hash(&username2).unwrap();
        assert_eq!(retrieved_hash, None);
    }

    #[test]
    #[should_panic]
    fn cannot_create_user_with_same_username() {
        let test_db = TestDb::new();
        let db_path = test_db.get_path();

        let conn = DbConn::new(db_path).unwrap();

        let username = "test_user";
        let passwd_hash = "test_pass";
        let salt = b"salt";
        conn.create_user(username, passwd_hash, salt).unwrap();
        conn.create_user(username, passwd_hash, salt).unwrap();
    }

    #[test]
    fn should_only_retrieve_data_for_user() {
        let test_db = TestDb::new();
        let db_path = test_db.get_path();

        let conn = DbConn::new(db_path).unwrap();

        let username1 = "test_user1";
        let username2 = "test_user2";
        let passwd_hash = "passwd";
        let salt = b"salt";

        conn.create_user(username1, passwd_hash, salt).unwrap();
        conn.create_user(username2, passwd_hash, salt).unwrap();

        // check if user is created
        conn.store_secret(1, Kind::Password, &"sec1".to_string(), b"sec1".to_vec())
            .unwrap();
        conn.store_secret(2, Kind::Password, &"sec2".to_string(), b"sec2".to_vec())
            .unwrap();
        conn.store_secret(2, Kind::Password, &"sec3".to_string(), b"sec3".to_vec())
            .unwrap();

        let labels1 = conn.get_labels(1).unwrap();
        assert_eq!(labels1.len(), 1);

        let labels2 = conn.get_labels(2).unwrap();
        assert_eq!(labels2.len(), 2);

        let labels3 = conn.get_labels(3).unwrap();
        assert_eq!(labels3.len(), 0);
    }

    #[test]
    fn can_edit_secret() {
        let test_db = TestDb::new();
        let db_path = test_db.get_path();
        let conn = DbConn::new(db_path).unwrap();

        // setup credentials
        let username = "test_user";
        let master_pass = "test_pass";
        let enc_salt = b"test_salt";

        conn.create_user(username, master_pass, enc_salt).unwrap();

        // store data
        let passwd: Vec<u8> = "passwd".into();
        let label = "pass1".to_string();
        conn.store_secret(1, Kind::Password, &label, passwd.clone())
            .unwrap();

        // check if data is first in the db
        let (kind, data) = conn.get_secret(1, &label).unwrap().unwrap();
        assert_eq!(&kind, &Kind::Password.to_str());
        assert_eq!(&data, &passwd);

        // edit the data
        let new_passwd: Vec<u8> = "new_passwd".into();
        conn.edit_secret(1, &label, &label, new_passwd.clone())
            .unwrap();
        let (kind, data) = conn.get_secret(1, &label).unwrap().unwrap();
        assert_eq!(&kind, &Kind::Password.to_str());
        assert_ne!(&data, &passwd);
        assert_eq!(&data, &new_passwd);

        // edit the label and the password
        let new_label = "pass2".to_string();
        conn.edit_secret(1, &label, &new_label, passwd.clone())
            .unwrap();
        let (kind, data) = conn.get_secret(1, &new_label).unwrap().unwrap();
        let old_label = conn.get_secret(1, &label).unwrap();
        assert_eq!(&kind, &Kind::Password.to_str());
        assert_ne!(&data, &new_passwd);
        assert_eq!(&data, &passwd);
        assert_eq!(old_label, None);
    }
}

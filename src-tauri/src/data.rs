use anyhow::{anyhow, Result};

use rusqlite::{params, Connection, OptionalExtension, Transaction};

pub struct DbConn {
    conn: Option<Connection>,
}

pub struct RetrieveLabelsQueryResult {
    pub kind: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Password,
    Text,
}

impl Kind {
    pub fn from_str(input: &str) -> Result<Kind> {
        match input {
            "password" => Ok(Kind::Password),
            "text" => Ok(Kind::Text),
            _ => Err(anyhow!("invalid kind: {:?}", input)),
        }
    }

    pub fn to_str(self) -> String {
        match self {
            Kind::Password => String::from("password"),
            Kind::Text => String::from("text"),
        }
    }
}

impl DbConn {
    pub fn new(path: &str) -> Result<DbConn> {
        let mut conn = DbConn {
            conn: Some(Connection::open(path)?),
        };
        conn.create_table()?;
        Ok(conn)
    }

    fn get_conn(&self) -> Result<&Connection> {
        match &self.conn {
            Some(conn) => Ok(conn),
            None => Err(anyhow!("does not have connection to the dabase")),
        }
    }

    fn start_transaction(&mut self) -> Result<Transaction> {
        match &mut self.conn {
            Some(conn) => Ok(conn.transaction()?),
            None => Err(anyhow!("does not have connection to the dabase")),
        }
    }

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

    fn check_user_already_exists(&self, username: &str) -> Result<bool> {
        let conn = self.get_conn()?;

        let mut stmt = conn.prepare("SELECT COUNT(*) FROM users WHERE username = ?1;")?;
        let count: i64 = stmt.query_row(params![username], |row| row.get(0))?;

        Ok(count > 0)
    }

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

    pub fn get_user_id(&self, username: &str) -> Result<Option<i64>> {
        let conn = self.get_conn()?;

        let mut stmt = conn.prepare("SELECT user_id FROM users WHERE username = ?1")?;
        let user_id: Option<i64> = stmt.query_row([username], |row| row.get(0)).optional()?;

        Ok(user_id)
    }

    pub fn get_username(&self, user_id: &i64) -> Result<Option<String>> {
        let conn = self.get_conn()?;

        let mut stmt = conn.prepare("SELECT username FROM users WHERE user_id = ?1")?;
        let user_id: Option<String> = stmt.query_row([user_id], |row| row.get(0)).optional()?;

        Ok(user_id)
    }

    pub fn get_user_passwd_hash(&self, username: &str) -> Result<Option<String>> {
        let conn = self.get_conn()?;

        let mut stmt = conn.prepare("SELECT passwd_hash FROM users WHERE username = ?1")?;
        let db_passwd_hash: Option<String> =
            stmt.query_row([username], |row| row.get(0)).optional()?;

        Ok(db_passwd_hash)
    }

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

    fn check_if_secret_label_exists(&self, user_id: i64, label: &str) -> Result<bool> {
        let conn = self.get_conn()?;

        let mut stmt =
            conn.prepare("SELECT COUNT(*) FROM secrets WHERE user_id = ?1 AND label = ?2;")?;
        let count: i64 = stmt.query_row(params![user_id, label], |row| row.get(0))?;

        Ok(count > 0)
    }

    pub fn store_secret(&self, user_id: i64, kind: Kind, label: &str, data: Vec<u8>) -> Result<()> {
        let conn = self.get_conn()?;

        if self.check_if_secret_label_exists(user_id, label)? {
            return Err(anyhow!("A secret with the same label already exists"));
        }

        let mut stmt = conn
            .prepare("INSERT INTO secrets (user_id, kind, label, data) VALUES (?1, ?2, ?3, ?4)")?;
        stmt.execute(params![user_id, kind.to_str(), label, data])?;

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

    pub fn get_secret(&self, user_id: i64, label: &str) -> Result<Option<Vec<u8>>> {
        let conn = self.get_conn()?;

        let mut stmt =
            conn.prepare("SELECT data FROM secrets WHERE user_id = ?1 AND label = ?2")?;

        let data = stmt
            .query_row([user_id.to_string(), label.to_string()], |row| row.get(0))
            .optional()?;
        Ok(data)
    }

    pub fn delete_secret(&self, user_id: &i64, label: &str) -> Result<()> {
        let conn = self.get_conn()?;

        let mut stmt = conn.prepare("DELETE FROM secrets WHERE user_id == ?1 AND label = ?2;")?;
        stmt.execute([user_id.to_string(), label.to_string()])?;
        Ok(())
    }

    pub fn close(mut self) -> Result<()> {
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
        let data1 = conn.get_secret(1, &label1).unwrap().unwrap();
        assert_eq!(&data1, &passwd);

        let data2 = conn.get_secret(1, &label2).unwrap().unwrap();
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
        assert_eq!(data, Some(passwd));

        conn.delete_secret(&1, &label).expect("should delete data");

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
        conn.store_secret(1, Kind::Password, &"sec1".to_string(), b"s1".to_vec())
            .unwrap();
        conn.store_secret(2, Kind::Password, &"sec2".to_string(), b"s2".to_vec())
            .unwrap();
        conn.store_secret(2, Kind::Password, &"sec3".to_string(), b"s3".to_vec())
            .unwrap();

        let labels1 = conn.get_labels(1).unwrap();
        assert_eq!(labels1.len(), 1);

        let labels2 = conn.get_labels(2).unwrap();
        assert_eq!(labels2.len(), 2);

        let labels3 = conn.get_labels(3).unwrap();
        assert_eq!(labels3.len(), 0);
    }
}

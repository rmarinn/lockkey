use anyhow::{anyhow, Result};

use rusqlite::{Connection, OptionalExtension};

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
        let conn = DbConn {
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

    fn create_table(&self) -> Result<()> {
        let conn = self.get_conn()?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS secrets (
                id      INTEGER PRIMARY KEY,
                kind    TEXT NOT NULL CHECK(kind IN ('text', 'password')),
                label   TEXT UNIQUE NOT NULL CHECK(length(label) <= 32),
                data    BLOB NOT NULL
            )
            ",
            (),
        )?;

        Ok(())
    }

    pub fn insert_into_table(&self, kind: Kind, label: &str, data: &Vec<u8>) -> Result<()> {
        let conn = self.get_conn()?;

        conn.execute(
            "INSERT INTO secrets (kind, label, data) VALUES (?1, ?2, ?3)",
            (kind.to_str(), label, data),
        )?;
        Ok(())
    }

    pub fn retrieve_labels(&self) -> Result<Vec<RetrieveLabelsQueryResult>> {
        let conn = self.get_conn()?;

        let mut stmt = conn.prepare("SELECT kind, label FROM secrets")?;

        let rows = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?;

        let mut labels: Vec<RetrieveLabelsQueryResult> = Vec::new();
        for row in rows {
            match row {
                Ok((kind, label)) => labels.push(RetrieveLabelsQueryResult { kind, label }),
                _ => (),
            }
        }
        Ok(labels)
    }

    pub fn retrieve_data(&self, label: &str) -> Result<Option<Vec<u8>>> {
        let conn = self.get_conn()?;

        let mut stmt = conn.prepare("SELECT data FROM secrets WHERE label = ?1")?;

        let data = stmt
            .query_row([label], |row| row.get(0))
            .optional()
            .map_err(|e| e.into());
        data
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

        let passwd: Vec<u8> = "passwd".into();
        let label1 = "pass1".to_string();
        conn.insert_into_table(Kind::Password, &label1, &passwd)
            .expect("should insert into table");

        let label2 = "pass2".to_string();
        conn.insert_into_table(Kind::Text, &label2, &passwd)
            .expect("should insert into table");

        // getting back the data
        let data1 = conn
            .retrieve_data(&label1)
            .expect("should retrieve data")
            .unwrap();
        assert_eq!(&data1, &passwd);

        let data2 = conn
            .retrieve_data(&label2)
            .expect("should retrieve data")
            .unwrap();
        assert_eq!(&data2, &passwd);
    }

    #[test]
    fn can_retrieve_labels() {
        let test_db = TestDb::new();
        let db_path = test_db.get_path();

        let conn = DbConn::new(db_path).unwrap();

        let passwd: Vec<u8> = "passwd".into();
        let label1 = "pass1".to_string();
        let kind1 = Kind::Password;
        conn.insert_into_table(kind1.clone(), &label1, &passwd)
            .expect("should insert into table");

        let label2 = "pass2".to_string();
        let kind2 = Kind::Text;
        conn.insert_into_table(kind2.clone(), &label2, &passwd)
            .expect("should insert into table");

        let query = conn.retrieve_labels().expect("should retrieve labels");

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

        conn.insert_into_table(Kind::Password, &String::from("pass1"), &"pass1".into())
            .unwrap();
        conn.insert_into_table(Kind::Text, &String::from("pass1"), &"pass2".into())
            .unwrap();
    }
}

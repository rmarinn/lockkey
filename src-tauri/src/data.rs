use anyhow::Result;

use rusqlite::{Connection, OptionalExtension};

pub struct DbConn {
    conn: Connection,
}

impl DbConn {
    pub fn new(path: &str) -> Result<DbConn> {
        let conn = DbConn {
            conn: Connection::open(path)?,
        };
        conn.create_table()?;
        Ok(conn)
    }

    fn create_table(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS secrets (
                id      INTEGER PRIMARY KEY,
                label   TEXT UNIQUE NOT NULL,
                data    BLOB
            )
            ",
            (),
        )?;
        Ok(())
    }

    pub fn insert_into_table(&self, label: &str, data: &Vec<u8>) -> Result<()> {
        self.conn.execute(
            "INSERT INTO secrets (label, data) VALUES (?1, ?2)",
            (&label, &data),
        )?;
        Ok(())
    }

    pub fn retrieve_labels(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT label FROM secrets")?;

        let rows = stmt.query_map([], |row| row.get(0))?;

        let mut labels = Vec::new();
        for label in rows {
            match label {
                Ok(value) => labels.push(value),
                _ => (),
            }
        }
        Ok(labels)
    }

    pub fn retrieve_data(&self, label: &str) -> Result<Option<Vec<u8>>> {
        let mut stmt = self
            .conn
            .prepare("SELECT data FROM secrets WHERE label = ?1")?;

        let data = stmt
            .query_row([label], |row| row.get(0))
            .optional()
            .map_err(|e| e.into());
        data
    }

    pub fn close(self) -> Result<()> {
        self.conn.close().map_err(|(_, e)| e)?;
        Ok(())
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

        let conn = DbConn::new(db_path).unwrap();
        conn.close().unwrap();
    }

    #[test]
    fn can_insert_and_retrieve_data_from_table() {
        let test_db = TestDb::new();
        let db_path = test_db.get_path();

        let conn = DbConn::new(db_path).unwrap();

        let passwd: Vec<u8> = "passwd".into();

        let label1 = "pass1".to_string();
        conn.insert_into_table(&label1, &passwd)
            .expect("should insert into table");

        let label2 = "pass2".to_string();
        conn.insert_into_table(&label2, &passwd)
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

        conn.close().unwrap();
    }

    #[test]
    #[should_panic]
    fn should_have_unique_labels() {
        let test_db = TestDb::new();
        let db_path = test_db.get_path();

        let conn = DbConn::new(db_path).unwrap();

        let label1 = "pass1".to_string();
        let pass: Vec<u8> = "pass".into();
        conn.insert_into_table(&label1, &pass).unwrap();

        let label2 = "pass1".to_string();
        conn.insert_into_table(&label2, &pass).unwrap();

        conn.close().unwrap();
    }
}

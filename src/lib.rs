pub mod data;
pub mod encryption;

use anyhow::Result;

use crate::data::DbConn;
use crate::encryption::*;

pub fn insert_into_db(conn: &DbConn, key: &[u8; 32], label: &str, secret: &String) -> Result<()> {
    let ciphertext = encrypt(&key, secret.as_bytes())?;
    conn.insert_into_table(label, &ciphertext)?;
    Ok(())
}

pub fn retrieve_from_db(conn: &DbConn, key: &[u8; 32], label: &str) -> Result<Option<String>> {
    let data = match conn.retrieve_data(label)? {
        Some(cipertext) => cipertext,
        _ => return Ok(None),
    };

    let decrypted_text = decrypt(&key, &data)?;
    let decoded_string = String::from_utf8(decrypted_text.to_vec())?;
    Ok(Some(decoded_string))
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
        let db_conn =
            DbConn::new(Some(db_path.to_str().unwrap())).expect("should create db connection");

        let master_pass = b"a$$word";
        let label = "my pass";
        let secret = "my secret".to_string();

        let key = derive_key(master_pass).expect("should derive");

        insert_into_db(&db_conn, &key, &label, &secret).expect("should insert secret into db");
        let retrieved_secret = retrieve_from_db(&db_conn, &key, &label)
            .expect("should retrieve secret from db")
            .unwrap();

        assert_eq!(secret, retrieved_secret);

        // cleanup
        db_conn.close().unwrap();
        remove_db_with_retry(&db_path);
    }
}

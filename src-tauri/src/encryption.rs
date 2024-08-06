use aes_gcm::{aead::Aead, AeadCore, Aes256Gcm, KeyInit, Nonce};
use anyhow::{anyhow, Result};
use argon2::Argon2;
use rand::rngs::OsRng;
use rand::Rng;

/// generates a 16-bytes-long salt
fn generate_salt() -> [u8; 16] {
    rand::thread_rng().gen::<[u8; 16]>()
}

/// generates a 32-bytes-long key from a password and a salt
fn derive_key(password: &[u8], salt: &[u8]) -> Result<[u8; 32]> {
    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(password, salt, &mut key)
        .map_err(|e| anyhow!("hashing password to a key failed: {:?}", e))?;
    return Ok(key);
}

pub fn generate_salt_from_string(string: &str) -> Vec<u8> {
    use std::cmp::min;

    const FIXED_SALT: &[u8] = b"!s4lTy_L0cK+k3Y~";
    let string_bytes = string.as_bytes();

    let mut combined_salt: Vec<u8> = Vec::with_capacity(string_bytes.len() + FIXED_SALT.len());

    let string_len = string_bytes.len();
    let salt_len = FIXED_SALT.len();

    for i in 0..min(string_len, salt_len) {
        combined_salt.push(string_bytes[i]);
        combined_salt.push(FIXED_SALT[i]);
    }

    if string_len > salt_len {
        combined_salt.extend_from_slice(&string_bytes[salt_len..string_len]);
    } else {
        combined_salt.extend_from_slice(&FIXED_SALT[string_len..salt_len]);
    }

    combined_salt
}

/// generates a 32-bytes-long key from a password and a salt
pub fn derive_key_from_string(string: String, salt: Vec<u8>) -> Result<[u8; 32]> {
    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(string.as_bytes().as_ref(), &salt, &mut key)
        .map_err(|e| anyhow!("hashing string to a key failed: {:?}", e))?;
    return Ok(key);
}

pub fn encrypt_using_key(key: &[u8], secret: String) -> Result<Vec<u8>> {
    let salt = generate_salt();
    let ciper_key = derive_key(key, &salt)?;
    let cipher = Aes256Gcm::new(&ciper_key.into());
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, secret.as_bytes())
        .map_err(|e| anyhow!("encryption failed: {:?}", e))?;
    Ok([salt.to_vec(), nonce.to_vec(), ciphertext].concat())
}

pub fn decrypt_using_key(key: &[u8], ciphertext: Vec<u8>) -> Result<String> {
    if ciphertext.len() <= 28 {
        return Err(anyhow!("invalid ciphertext!"));
    }
    let (salt, rest) = ciphertext.split_at(16);
    let (nonce, ciphertext) = rest.split_at(12);
    let cipher_key = derive_key(key, &salt)?;
    let cipher = Aes256Gcm::new(&cipher_key.into());
    let decrypted_bytes = cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .map_err(|_| anyhow!("decryption failed"))?;
    Ok(String::from_utf8(decrypted_bytes)?)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_generate_salt_from_string() {
        let string = "abcdefg";
        let salt = generate_salt_from_string(&string);
        assert_eq!(b"a!bsc4dleTfyg_L0cK+k3Y~".to_vec(), salt);

        let string = "abcdefghijklmnopqrstuv";
        let salt = generate_salt_from_string(&string);
        assert_eq!(b"a!bsc4dleTfyg_hLi0jckKl+mkn3oYp~qrstuv".to_vec(), salt);
    }

    #[test]
    fn can_encrypt_and_decrypt_using_key() {
        let username = "user1".to_string();
        let passwd = "test_password".to_string();
        let secret = "Hello world".to_string();

        let salt = generate_salt_from_string(&username);
        let key = derive_key_from_string(passwd, salt).unwrap();

        let ciphertext = encrypt_using_key(&key, secret.clone()).unwrap();
        let decrypted = decrypt_using_key(&key, ciphertext).unwrap();

        assert_eq!(secret, decrypted);
    }
}

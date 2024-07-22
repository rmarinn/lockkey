use aes_gcm::{aead::Aead, AeadCore, Aes256Gcm, KeyInit, Nonce};
use anyhow::{anyhow, Result};
use argon2::Argon2;
use rand::rngs::OsRng;
use rand::Rng;

fn generate_salt() -> [u8; 16] {
    rand::thread_rng().gen::<[u8; 16]>()
}

pub fn derive_key(password: &[u8]) -> Result<[u8; 32]> {
    let salt = generate_salt();
    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(password, &salt, &mut key)
        .map_err(|e| anyhow!("hashing password to a key failed: {:?}", e))?;
    return Ok(key);
}

pub fn encrypt(key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new(key.into());
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bit nonce
    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|e| anyhow!("encryption failed: {:?}", e))?;
    Ok([nonce.to_vec(), ciphertext].concat())
}

pub fn decrypt(key: &[u8; 32], ciphertext: &[u8]) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new(key.into());
    if ciphertext.len() <= 12 {
        return Err(anyhow!("invalid ciphertext!"));
    }
    let (nonce, ciphertext) = ciphertext.split_at(12);
    let plaintext = cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .map_err(|e| anyhow!("decryption failed: {:?}", e))?;
    Ok(plaintext)
}

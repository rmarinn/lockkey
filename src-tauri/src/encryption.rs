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

/// encrypts a secret using a password
pub fn encrypt(passwd: &str, secret: &str) -> Result<Vec<u8>> {
    let salt = generate_salt();
    let key = derive_key(passwd.as_bytes(), &salt)?;
    let cipher = Aes256Gcm::new(&key.into());
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, secret.as_bytes())
        .map_err(|e| anyhow!("encryption failed: {:?}", e))?;
    Ok([salt.to_vec(), nonce.to_vec(), ciphertext].concat())
}

pub fn decrypt(passwd: &str, ciphertext: &[u8]) -> Result<String> {
    if ciphertext.len() <= 28 {
        return Err(anyhow!("invalid ciphertext!"));
    }
    let (salt, rest) = ciphertext.split_at(16);
    let (nonce, ciphertext) = rest.split_at(12);
    let key = derive_key(passwd.as_bytes(), &salt)?;
    let cipher = Aes256Gcm::new(&key.into());
    let decrypted_bytes = cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .map_err(|_| anyhow!("decryption failed"))?;
    Ok(String::from_utf8(decrypted_bytes)?)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_encrypt_and_decrypt() {
        let passwd = "a$$word";
        let secret = "Hello world";

        let ciphertext = encrypt(&passwd, &secret).expect("should encrypt plaintext");
        let decrpytedtext = decrypt(&passwd, &ciphertext).expect("should decrypt ciphertext");

        assert_eq!(secret, decrpytedtext);
    }
}

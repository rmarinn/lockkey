use aes_gcm::{aead::Aead, AeadCore, Aes256Gcm, KeyInit, Nonce};
use anyhow::{anyhow, Ok, Result};
use argon2::Argon2;
use rand::rngs::OsRng;
use rand::Rng;

const ENC_KEY_SALT_LEN: usize = 16; // encryption key salt length
const ENC_KEY_LEN: usize = 32; // encryption key length
const NONCE_LENGTH: usize = 12; // nonce length for AES-GCM

/// Generates a random 16-byte salt.
/// This function uses a cryptographically secure random number generator.
pub fn generate_salt() -> [u8; ENC_KEY_SALT_LEN] {
    rand::thread_rng().gen::<[u8; ENC_KEY_SALT_LEN]>()
}

/// Derives a 32-byte encryption key from a password and a salt using the Argon2 key derivation function.
///
/// # Arguments
/// * `passwd` - The input password string.
/// * `salt` - A slice of bytes used as the salt for key derivation.
///
/// # Returns
/// * `Result<[u8; 32]>` - A fixed-size array of 32 bytes, representing the derived key.
pub fn derive_encryption_key(passwd: &str, salt: &[u8]) -> Result<[u8; 32]> {
    let mut generated_key = [0u8; ENC_KEY_LEN];
    Argon2::default()
        .hash_password_into(&passwd.as_bytes(), &salt, &mut generated_key)
        .map_err(|e| anyhow!("failed to generate key: {e:?}"))?;
    Ok(generated_key)
}

/// Derives a 32-byte cipher key from an encryption key and a salt.
/// This is useful for creating a cipher key from a previously derived encryption key.
///
/// # Arguments
/// * `encryption_key` - A slice of bytes representing the encryption key.
/// * `salt` - A slice of bytes used as the salt for the second key derivation.
///
/// # Returns
/// * `Result<[u8; 32]>` - A fixed-size array of 32 bytes, representing the derived cipher key.
fn derive_cipherkey(encryption_key: &[u8], salt: &[u8]) -> Result<[u8; 32]> {
    let mut cipherkey = [0u8; 32];
    Argon2::default()
        .hash_password_into(encryption_key, &salt, &mut cipherkey)
        .map_err(|e| anyhow!("hashing password to a key failed: {:?}", e))?;
    return Ok(cipherkey);
}

/// Encrypts a string using a provided encryption key with AES-256-GCM.
/// It also generates a salt and nonce for added security.
///
/// # Arguments
/// * `encryption_key` - A slice of bytes representing the encryption key.
/// * `secret` - The string message to be encrypted.
///
/// # Returns
/// * `Result<Vec<u8>>` - A vector of bytes representing the concatenated salt, nonce, and ciphertext.
pub fn encrypt_using_key(encryption_key: &[u8], secret: &str) -> Result<Vec<u8>> {
    let salt = generate_salt();
    let ciper_key = derive_cipherkey(encryption_key, &salt)?;
    let cipher = Aes256Gcm::new(&ciper_key.into());
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, secret.as_bytes())
        .map_err(|e| anyhow!("encryption failed: {:?}", e))?;
    Ok([salt.to_vec(), nonce.to_vec(), ciphertext].concat())
}

/// Decrypts a ciphertext using a provided encryption key with AES-256-GCM.
/// The ciphertext must include a salt, nonce, and the encrypted data.
///
/// # Arguments
/// * `encryption_key` - A slice of bytes representing the encryption key.
/// * `ciphertext` - A vector of bytes that contains the salt, nonce, and encrypted data.
///
/// # Returns
/// * `Result<String>` - The decrypted string message.
pub fn decrypt_using_key(encryption_key: &[u8], ciphertext: Vec<u8>) -> Result<String> {
    if ciphertext.len() <= 28 {
        return Err(anyhow!("invalid ciphertext!"));
    }
    let (salt, rest) = ciphertext.split_at(ENC_KEY_SALT_LEN);
    let (nonce, ciphertext) = rest.split_at(NONCE_LENGTH);
    let cipher_key = derive_cipherkey(encryption_key, salt)?;
    let cipher = Aes256Gcm::new(&cipher_key.into());
    let decrypted_bytes = cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .map_err(|_| anyhow!("decryption failed"))?;
    Ok(String::from_utf8(decrypted_bytes)?)
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests encryption and decryption to ensure that the same secret message
    /// can be encrypted and decrypted correctly using the provided key.
    #[test]
    fn can_encrypt_and_decrypt_using_key() {
        let passwd = "test_password".to_string();
        let secret = "Hello world".to_string();

        let salt = generate_salt();
        let enc_key = derive_encryption_key(&passwd, &salt).unwrap();

        let ciphertext = encrypt_using_key(&enc_key, &secret).unwrap();
        let decrypted = decrypt_using_key(&enc_key, ciphertext).unwrap();

        assert_eq!(secret, decrypted);
    }
}

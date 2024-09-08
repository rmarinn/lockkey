use anyhow::{anyhow, Ok, Result};
use argon2::password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString};
use argon2::{Argon2, PasswordHash};

/// Hashes the provided password using the Argon2 hashing algorithm.
///
/// # Arguments
/// * `passwd` - A string slice representing the password to be hashed.
///
/// # Returns
/// * `Result<String>` - A result containing the hashed password as a PHC string if successful, or an error if the hashing process fails.
pub fn hash_password(passwd: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(passwd.as_bytes(), &salt)
        .map_err(|_| anyhow!("an error occured while trying to generate a hash for the password"))?
        .to_string();
    Ok(hash)
}

/// Verifies whether the provided password matches the hashed password.
///
/// # Arguments
/// * `passwd` - A string slice representing the plain-text password to verify.
/// * `hash` - A string slice representing the PHC formatted hash to check against.
///
/// # Returns
/// * `Result<bool>` - A result containing a boolean indicating whether the password is valid (`true` if it matches, `false` otherwise), or an error if verification fails.
pub fn verify_passwd(passwd: &str, hash: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(&hash)
        .map_err(|_| anyhow!("an error occured while trying to parse the PHC string"))?;
    Ok(Argon2::default()
        .verify_password(passwd.as_bytes(), &parsed_hash)
        .is_ok())
}

#[cfg(test)]
mod test {
    use super::*;

    /// Test case for ensuring that a password can be hashed and then successfully verified.
    #[test]
    fn can_hash_and_verify_passwd() {
        let passwd = "test_pass".to_string();
        let hash = hash_password(&passwd).unwrap();
        assert!(verify_passwd(&passwd, &hash).unwrap());
    }
}

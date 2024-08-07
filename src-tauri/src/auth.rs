use anyhow::{anyhow, Ok, Result};
use argon2::password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString};
use argon2::{Argon2, PasswordHash};

pub fn hash_password(passwd: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(passwd.as_bytes(), &salt)
        .map_err(|_| anyhow!("an error occured while trying to generate a hash for the password"))?
        .to_string();
    Ok(hash)
}

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

    #[test]
    fn can_hash_and_verify_passwd() {
        let passwd = "test_pass".to_string();
        let hash = hash_password(&passwd).unwrap();
        assert!(verify_passwd(&passwd, &hash).unwrap());
    }
}

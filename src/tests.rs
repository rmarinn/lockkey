use super::encryption::*;

#[test]
fn can_encrypt_and_decrypt() {
    let master_pass = b"a$$word";
    let key = derive_key(master_pass).expect("should derive encryption key");

    let plaintext = b"Hello world";
    let ciphertext = encrypt(&key, plaintext).expect("should encrypt plaintext");
    let decrpytedtext = decrypt(&key, &ciphertext).expect("should decrypt ciphertext");

    assert_eq!(plaintext, &decrpytedtext[..]);
}

#[test]
#[should_panic]
fn should_not_decrypt_invalid_ciphertext() {
    let master_pass = b"a$$word";
    let key = derive_key(master_pass).expect("should derive encryption key");

    let plaintext = b"Hello world";
    let _decryptedtext = decrypt(&key, plaintext).expect("should decrypt");
}

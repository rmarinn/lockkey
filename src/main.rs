use lockkey::encryption::*;

fn main() {
    let master_pass = b"a$$word";
    let key = derive_key(master_pass).expect("should derive");

    let plaintext = b"Hello world";
    let ciphertext = encrypt(&key, plaintext).expect("should encrypt");
    let decryptedtext = decrypt(&key, &ciphertext).expect("should decrypt");

    println!(
        "plaintext: {:?}",
        String::from_utf8(plaintext.to_vec()).expect("should decode plaintext")
    );
    println!("ciphertext: {:?}", ciphertext);
    println!(
        "decryptedtext: {:?}",
        String::from_utf8(decryptedtext.to_vec()).expect("should decode plaintext from decryption")
    );
}

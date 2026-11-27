use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit, OsRng, rand_core::RngCore},
};

fn main() {
    let message = "2026_КБЗІ-23-1_Кондратюк_Ярослав_варіант:07";

    // Generate a random AES-256 key.
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);

    // Generate a random 96-bit nonce.
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);

    let cipher = Aes256Gcm::new_from_slice(&key).unwrap();
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Encrypt the message.
    let ciphertext = cipher.encrypt(nonce, message.as_bytes()).unwrap();

    println!("Original text: {}", message);
    println!("AES-256 key: {}", hex::encode(key));
    println!("Nonce: {}", hex::encode(nonce_bytes));
    println!("Ciphertext: {}", hex::encode(&ciphertext));

    // Decrypt the message.
    let decrypted = cipher.decrypt(nonce, ciphertext.as_ref()).unwrap();

    // Detect modified ciphertext.
    let mut modified = ciphertext.clone();
    modified[0] ^= 1;

    let tampering_detected = cipher.decrypt(nonce, modified.as_ref()).is_err();

    println!("Tampering detected: {}", tampering_detected);

    // Reject an incorrect key.
    let mut wrong_key = [0u8; 32];
    OsRng.fill_bytes(&mut wrong_key);

    let wrong_cipher = Aes256Gcm::new_from_slice(&wrong_key).unwrap();

    let wrong_key_rejected = wrong_cipher.decrypt(nonce, ciphertext.as_ref()).is_err();

    println!("Wrong key rejected: {}", wrong_key_rejected);

    println!("Decrypted text: {}", String::from_utf8(decrypted).unwrap());
}

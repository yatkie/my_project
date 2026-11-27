use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit, OsRng, rand_core::RngCore},
};

use std::fs;
use std::path::Path;

/// Generates a random 256-bit encryption key.
#[cfg(test)]
pub fn generate_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    key
}

/// Encrypts data using AES-256-GCM.
pub fn encrypt(key: &[u8; 32], plaintext: &[u8]) -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| "Invalid encryption key")?;

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);

    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|_| "Encryption failed")?;

    Ok((nonce_bytes.to_vec(), ciphertext))
}

/// Decrypts data using AES-256-GCM.
pub fn decrypt(key: &[u8; 32], nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, &'static str> {
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| "Invalid encryption key")?;

    let nonce: &[u8; 12] = nonce.try_into().map_err(|_| "Invalid nonce length")?;

    cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .map_err(|_| "Decryption failed or authentication failed")
}

/// Encrypts data and saves nonce + ciphertext to a file.
pub fn encrypt_to_file(
    key: &[u8; 32],
    plaintext: &[u8],
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let (nonce, ciphertext) = encrypt(key, plaintext).map_err(std::io::Error::other)?;

    let mut output = nonce;
    output.extend_from_slice(&ciphertext);

    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, output)?;

    Ok(())
}

/// Loads and decrypts data from a file.
pub fn decrypt_from_file(
    key: &[u8; 32],
    path: &Path,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let data = fs::read(path)?;

    // 12 bytes nonce + 16 bytes minimum GCM authentication tag.
    if data.len() < 28 {
        return Err(std::io::Error::other("Encrypted file is too short").into());
    }

    let nonce = &data[..12];
    let ciphertext = &data[12..];

    let plaintext = decrypt(key, nonce, ciphertext).map_err(std::io::Error::other)?;

    Ok(plaintext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encryption_and_decryption_work() {
        let key = generate_key();
        let original = b"SolanaTokenManager secret data";

        let (nonce, ciphertext) = encrypt(&key, original).unwrap();
        let decrypted = decrypt(&key, &nonce, &ciphertext).unwrap();

        assert_eq!(original.to_vec(), decrypted);
    }

    #[test]
    fn modified_ciphertext_is_rejected() {
        let key = generate_key();

        let (nonce, mut ciphertext) = encrypt(&key, b"secret").unwrap();

        ciphertext[0] ^= 1;

        assert!(decrypt(&key, &nonce, &ciphertext).is_err());
    }

    #[test]
    fn invalid_nonce_is_rejected() {
        let key = generate_key();

        assert!(decrypt(&key, &[0u8; 8], b"ciphertext").is_err());
    }

    #[test]
    fn file_encryption_and_decryption_work() {
        let key = generate_key();
        let original = b"Solana private key test";

        let path =
            std::env::temp_dir().join(format!("solana_crypto_test_{}.enc", std::process::id()));

        encrypt_to_file(&key, original, &path).unwrap();

        let decrypted = decrypt_from_file(&key, &path).unwrap();

        assert_eq!(original.to_vec(), decrypted);

        fs::remove_file(path).unwrap();
    }
}

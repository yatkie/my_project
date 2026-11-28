use crate::crypto;
use rand_core::{OsRng, RngCore};
use rsa::{Oaep, RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::error::Error;

const RSA_KEY_SIZE: usize = 2048;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedEnvelope {
    pub encrypted_key: String,
    pub nonce: String,
    pub ciphertext: String,
}

pub fn generate_rsa_keypair() -> Result<(RsaPrivateKey, RsaPublicKey), Box<dyn Error>> {
    let mut rng = OsRng;

    let private_key = RsaPrivateKey::new(&mut rng, RSA_KEY_SIZE)?;
    let public_key = RsaPublicKey::from(&private_key);

    Ok((private_key, public_key))
}

// Encryption happens on the sender's device.
pub fn encrypt_for_recipient(
    public_key: &RsaPublicKey,
    plaintext: &[u8],
) -> Result<EncryptedEnvelope, Box<dyn Error>> {
    let mut rng = OsRng;

    // Generate a random AES-256 session key.
    let mut aes_key = [0u8; 32];
    rng.fill_bytes(&mut aes_key);

    // Encrypt the message using AES-256-GCM.
    let (nonce, ciphertext) =
        crypto::encrypt(&aes_key, plaintext).map_err(std::io::Error::other)?;

    // Protect the AES key using the recipient's RSA public key.
    let encrypted_key = public_key.encrypt(&mut rng, Oaep::new::<Sha256>(), &aes_key)?;

    Ok(EncryptedEnvelope {
        encrypted_key: hex::encode(encrypted_key),
        nonce: hex::encode(nonce),
        ciphertext: hex::encode(ciphertext),
    })
}

// Only the recipient with the private RSA key can decrypt.
pub fn decrypt_for_recipient(
    private_key: &RsaPrivateKey,
    envelope: &EncryptedEnvelope,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let encrypted_key = hex::decode(&envelope.encrypted_key)?;
    let nonce = hex::decode(&envelope.nonce)?;
    let ciphertext = hex::decode(&envelope.ciphertext)?;

    let aes_key = private_key.decrypt(Oaep::new::<Sha256>(), &encrypted_key)?;

    let aes_key: [u8; 32] = aes_key
        .try_into()
        .map_err(|_| std::io::Error::other("Invalid AES key length"))?;

    let plaintext =
        crypto::decrypt(&aes_key, &nonce, &ciphertext).map_err(std::io::Error::other)?;

    Ok(plaintext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn e2ee_encryption_and_decryption_work() {
        let (private_key, public_key) = generate_rsa_keypair().unwrap();

        let original = b"Confidential client-server message";

        let envelope = encrypt_for_recipient(&public_key, original).unwrap();

        let decrypted = decrypt_for_recipient(&private_key, &envelope).unwrap();

        assert_eq!(decrypted, original);
    }

    #[test]
    fn encrypted_envelope_does_not_contain_plaintext() {
        let (_, public_key) = generate_rsa_keypair().unwrap();

        let original = b"Private information";

        let envelope = encrypt_for_recipient(&public_key, original).unwrap();

        let serialized = serde_json::to_string(&envelope).unwrap();

        assert!(!serialized.contains("Private information"));
    }

    #[test]
    fn modified_ciphertext_is_rejected() {
        let (private_key, public_key) = generate_rsa_keypair().unwrap();

        let mut envelope = encrypt_for_recipient(&public_key, b"Secret message").unwrap();

        let mut ciphertext = hex::decode(&envelope.ciphertext).unwrap();

        ciphertext[0] ^= 1;

        envelope.ciphertext = hex::encode(ciphertext);

        assert!(decrypt_for_recipient(&private_key, &envelope).is_err());
    }

    #[test]
    fn wrong_private_key_is_rejected() {
        let (_, public_key) = generate_rsa_keypair().unwrap();
        let (wrong_private_key, _) = generate_rsa_keypair().unwrap();

        let envelope = encrypt_for_recipient(&public_key, b"Secret message").unwrap();

        assert!(decrypt_for_recipient(&wrong_private_key, &envelope).is_err());
    }
}

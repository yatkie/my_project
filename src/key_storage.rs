use crate::crypto;
use solana_sdk::signature::Keypair;
use std::error::Error;
use std::path::Path;

/// Encrypts and stores a Solana keypair.
pub fn save_keypair(
    keypair: &Keypair,
    encryption_key: &[u8; 32],
    path: &Path,
) -> Result<(), Box<dyn Error>> {
    let secret_bytes = keypair.to_bytes();

    crypto::encrypt_to_file(encryption_key, &secret_bytes, path)?;

    Ok(())
}

/// Loads and restores a Solana keypair.
pub fn load_keypair(encryption_key: &[u8; 32], path: &Path) -> Result<Keypair, Box<dyn Error>> {
    let secret_bytes = crypto::decrypt_from_file(encryption_key, path)?;

    let keypair = Keypair::try_from(secret_bytes.as_slice())?;

    Ok(keypair)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::generate_key;
    use solana_sdk::signature::Signer;

    #[test]
    fn keypair_storage_and_restoration_work() {
        let keypair = Keypair::new();
        let original_pubkey = keypair.pubkey();

        let encryption_key = generate_key();

        let path =
            std::env::temp_dir().join(format!("solana_keypair_test_{}.enc", std::process::id()));

        save_keypair(&keypair, &encryption_key, &path).unwrap();

        let restored = load_keypair(&encryption_key, &path).unwrap();

        assert_eq!(original_pubkey, restored.pubkey());

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn wrong_encryption_key_is_rejected() {
        let keypair = Keypair::new();

        let encryption_key = generate_key();
        let wrong_key = generate_key();

        let path =
            std::env::temp_dir().join(format!("solana_wrong_key_test_{}.enc", std::process::id()));

        save_keypair(&keypair, &encryption_key, &path).unwrap();

        assert!(load_keypair(&wrong_key, &path).is_err());

        std::fs::remove_file(path).unwrap();
    }
}

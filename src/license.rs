use rsa::{RsaPublicKey, pkcs8::DecodePublicKey, pss::VerifyingKey};
use sha2::Sha256;
use signature::Verifier;
use std::{fs, path::Path};

/// Перевіряє цифровий підпис ліцензії RSA-PSS із SHA-256.
pub fn verify_license(
    license_path: &Path,
    signature_path: &Path,
    public_key_path: &Path,
) -> Result<bool, Box<dyn std::error::Error>> {
    let license = fs::read(license_path)?;
    let signature_bytes = fs::read(signature_path)?;
    let public_key_pem = fs::read_to_string(public_key_path)?;

    let public_key = RsaPublicKey::from_public_key_pem(&public_key_pem)?;

    let verifying_key = VerifyingKey::<Sha256>::new(public_key);

    let signature = rsa::pss::Signature::try_from(signature_bytes.as_slice())?;

    Ok(verifying_key.verify(&license, &signature).is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_license_is_valid() {
        let result = verify_license(
            Path::new("license.json"),
            Path::new("license.sig"),
            Path::new("keys/public_key.pem"),
        );

        assert!(result.unwrap());
    }

    #[test]
    fn modified_license_is_rejected() {
        let result = verify_license(
            Path::new("license_modified.json"),
            Path::new("license.sig"),
            Path::new("keys/public_key.pem"),
        );

        assert!(!result.unwrap());
    }
}

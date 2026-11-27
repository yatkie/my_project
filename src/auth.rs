
use bcrypt::{hash, verify};
use std::error::Error;

const COST: u32 = 12;

/// Створення bcrypt-хешу пароля.
pub fn hash_password(password: &str) -> Result<String, Box<dyn Error>> {
    if password.is_empty() || password.len() > 72 {
        return Err("Invalid password length".into());
    }

    Ok(hash(password, COST)?)
}

/// Перевірка пароля за збереженим хешем.
pub fn verify_password(
    password: &str,
    password_hash: &str,
) -> Result<bool, Box<dyn Error>> {
    if password.is_empty() || password.len() > 72 {
        return Ok(false);
    }

    Ok(verify(password, password_hash)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_password() {
        let password = "TestPassword123!";
        let hashed = hash_password(password).unwrap();

        assert!(verify_password(password, &hashed).unwrap());
    }

    #[test]
    fn incorrect_password() {
        let hashed = hash_password("TestPassword123!").unwrap();

        assert!(!verify_password("WrongPassword", &hashed).unwrap());
    }

    #[test]
    fn empty_password() {
        assert!(hash_password("").is_err());
        assert!(!verify_password("", "invalid").unwrap());
    }

    #[test]
    fn long_password() {
        let password = "a".repeat(73);
        assert!(hash_password(&password).is_err());
        assert!(!verify_password(&password, "invalid").unwrap());
    }
}

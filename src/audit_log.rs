use crate::blockchain::Blockchain;
use std::error::Error;
use std::fs;
use std::path::Path;

pub struct AuditLog {
    pub blockchain: Blockchain,
}

impl AuditLog {
    pub fn new() -> Self {
        Self {
            blockchain: Blockchain::new(),
        }
    }

    pub fn append_event(&mut self, event: &str) -> Result<(), Box<dyn Error>> {
        if !self.blockchain.is_valid() {
            return Err("Audit log integrity verification failed".into());
        }

        self.blockchain.add_block(event);

        Ok(())
    }

    pub fn verify(&self) -> bool {
        self.blockchain.is_valid()
    }

    pub fn save(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        if !self.verify() {
            return Err("Cannot save an invalid audit log".into());
        }

        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(&self.blockchain)?;

        fs::write(path, json)?;

        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self, Box<dyn Error>> {
        let json = fs::read_to_string(path)?;

        let blockchain: Blockchain = serde_json::from_str(&json)?;

        if !blockchain.is_valid() {
            return Err("Audit log integrity verification failed".into());
        }

        Ok(Self { blockchain })
    }

    pub fn print(&self) {
        self.blockchain.print_chain();
    }
}

impl Default for AuditLog {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temporary_path(test_name: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "my_project_audit_{}_{}.json",
            std::process::id(),
            test_name
        ))
    }

    #[test]
    fn audit_events_can_be_saved_and_loaded() {
        let path = temporary_path("save_load");

        let mut log = AuditLog::new();

        log.append_event("User authenticated").unwrap();
        log.append_event("Wallet created").unwrap();

        log.save(&path).unwrap();

        let loaded = AuditLog::load(&path).unwrap();

        assert!(loaded.verify());
        assert_eq!(loaded.blockchain.chain.len(), 3);

        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn modified_audit_log_is_rejected() {
        let path = temporary_path("tamper");

        let mut log = AuditLog::new();

        log.append_event("Original event").unwrap();
        log.save(&path).unwrap();

        let mut loaded = AuditLog::load(&path).unwrap();

        loaded.blockchain.chain[1].data = "Modified event".to_string();

        assert!(!loaded.verify());
        assert!(loaded.save(&path).is_err());

        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn invalid_log_cannot_accept_new_events() {
        let mut log = AuditLog::new();

        log.append_event("Original event").unwrap();

        log.blockchain.chain[1].data = "Tampered event".to_string();

        assert!(log.append_event("Another event").is_err());
    }
}

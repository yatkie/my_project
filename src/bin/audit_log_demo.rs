#![allow(dead_code)]

#[path = "../blockchain.rs"]
mod blockchain;

#[path = "../audit_log.rs"]
mod audit_log;

use audit_log::AuditLog;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== BLOCKCHAIN AUDIT LOG ===");

    let path = Path::new("logs/audit_chain.json");

    let mut log = AuditLog::new();

    println!("\n=== RECORDING SECURITY EVENTS ===");

    log.append_event("User authentication successful")?;
    log.append_event("Wallet loaded")?;
    log.append_event("Token account prepared")?;
    log.append_event("Transaction signed")?;
    log.append_event("Security event recorded")?;

    println!("Five security events recorded.");

    println!("\n=== VERIFYING AUDIT LOG ===");

    if !log.verify() {
        return Err("Audit log verification failed".into());
    }

    println!("Audit integrity verified.");

    println!("\n=== SAVING AUDIT LOG ===");

    log.save(path)?;

    println!("Audit log saved successfully.");

    println!("\n=== LOADING AUDIT LOG ===");

    let loaded = AuditLog::load(path)?;

    loaded.print();

    println!("\n=== FINAL VERIFICATION ===");

    assert!(loaded.verify());

    println!("Audit integrity: TRUE");
    println!("Audit log demonstration completed successfully!");

    Ok(())
}

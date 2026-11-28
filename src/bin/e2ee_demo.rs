#[allow(dead_code)]
#[path = "../crypto.rs"]
mod crypto;

#[path = "../e2ee.rs"]
mod e2ee;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== END-TO-END ENCRYPTION DEMO ===");

    // Recipient generates RSA keys.
    let (private_key, public_key) = e2ee::generate_rsa_keypair()?;

    let original = b"Confidential message from client to recipient";

    println!("\nOriginal message:");
    println!("{}", String::from_utf8_lossy(original));

    // Sender encrypts the message.
    let envelope = e2ee::encrypt_for_recipient(&public_key, original)?;

    println!("\n=== ENCRYPTED DATA ===");

    let json = serde_json::to_string_pretty(&envelope)?;

    println!("{}", json);

    // Simulate server storage or forwarding.
    let server_received = serde_json::from_str(&json)?;

    println!("\nServer received encrypted envelope.");

    // Recipient decrypts the message.
    let decrypted = e2ee::decrypt_for_recipient(&private_key, &server_received)?;

    println!("\n=== RECIPIENT DECRYPTION ===");

    println!("{}", String::from_utf8_lossy(&decrypted));

    assert_eq!(original.to_vec(), decrypted);

    println!("\nE2EE demonstration completed successfully!");

    Ok(())
}

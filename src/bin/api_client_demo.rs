#[path = "../api_client.rs"]
mod api_client;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Rust Library API Client ===");

    let client_id = std::env::var("LIBRARY_CLIENT_ID")?;
    let client_secret = std::env::var("LIBRARY_CLIENT_SECRET")?;

    let api = api_client::ApiClient::authenticate(&client_id, &client_secret)?;

    println!("\n=== AUTHENTICATION ===");
    println!("Authentication successful!");

    println!("\n=== WHOAMI ===");
    let reader = api.get_reader()?;
    println!("Reader: {}", reader.reader_name);
    println!("Client ID: {}", reader.client_id);
    println!("Reader ID: {}", reader.reader_id);
    println!("Variant: {}", reader.variant);
    println!("Library card: {}", reader.library_card);
    println!("Watermark: {}", reader.watermark);

    println!("\n=== BOOKS ===");
    let books = api.get_books()?;
    println!("Total books: {}", books.count);
    println!("Variant: {}", books.variant);
    println!("Watermark: {}", books.watermark);

    println!("\n=== LOANS ===");
    let loans = api.get_loans()?;
    println!("Active loans: {}", loans.count);
    println!("Variant: {}", loans.variant);
    println!("Watermark: {}", loans.watermark);

    println!("\n=== SUMMARY ===");
    let summary = api.get_summary()?;
    println!("Total books: {}", summary.total_books);
    println!("Available books: {}", summary.available_books);
    println!("Active loans: {}", summary.active_loans);
    println!("Variant: {}", summary.variant);
    println!("Watermark: {}", summary.watermark);

    println!("\nRust API client completed successfully!");

    Ok(())
}

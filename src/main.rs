mod access_control;
mod auth;

use access_control::{Permission, Role, authorize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    program_pack::Pack,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_system_interface::instruction::create_account;
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};
use spl_token::instruction::{initialize_mint, mint_to};
use std::{env, error::Error};

const SOLANA_RPC_URL: &str = "https://api.devnet.solana.com";

// Перевірка пароля користувача.
fn authenticate() -> Result<bool, Box<dyn Error>> {
    let password_hash =
        env::var("APP_PASSWORD_HASH").map_err(|_| "APP_PASSWORD_HASH is not configured")?;

    println!("=== Authentication ===");

    for attempt in 1..=3 {
        let password = rpassword::prompt_password("Enter password: ")?;

        if auth::verify_password(&password, &password_hash)? {
            println!("Authentication successful.");
            return Ok(true);
        }

        println!("Incorrect password. Attempts remaining: {}", 3 - attempt);
    }

    println!("Access denied.");
    Ok(false)
}

// Генерація bcrypt-хешу для налаштування пароля.
fn generate_password_hash() -> Result<(), Box<dyn Error>> {
    let password = rpassword::prompt_password("Enter new password: ")?;

    let password_hash = auth::hash_password(&password)?;

    println!("Generated password hash:");
    println!("{}", password_hash);

    Ok(())
}

// Перевірка окремого дозволу користувача.
fn require_permission(role: Role, permission: Permission) -> Result<(), Box<dyn Error>> {
    authorize(Some(role), permission)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Окрема команда для генерації хешу пароля.
    if env::args().nth(1).as_deref() == Some("hash-password") {
        return generate_password_hash();
    }

    println!("=== SolanaTokenManager ===");

    // 1. Автентифікація.
    if !authenticate()? {
        return Ok(());
    }

    // 2. Визначення ролі користувача.
    let role_value = env::var("APP_ROLE")?;

    let role =
        Role::parse(&role_value).ok_or("Invalid APP_ROLE. Use admin, operator or viewer.")?;

    println!("User role: {:?}", role);

    // 3. Перевірка права перегляду інформації.
    require_permission(role, Permission::ViewInfo)?;

    println!("Access granted.");

    // 4. Підключення до Solana Devnet.
    let client = RpcClient::new(SOLANA_RPC_URL.to_string());

    // 5. Генерація ключових пар.
    require_permission(role, Permission::GenerateKeys)?;

    let payer = Keypair::new();
    let mint = Keypair::new();

    let decimals = 9;

    println!("Wallet: {}", payer.pubkey());
    println!("Mint: {}", mint.pubkey());

    // 6. Отримання мінімальної суми для rent exemption.
    let rent = client.get_minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN)?;

    // 7. Створення Mint Account.
    require_permission(role, Permission::CreateMint)?;

    let create_mint = create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        rent,
        spl_token::state::Mint::LEN as u64,
        &spl_token::ID,
    );

    // 8. Ініціалізація Mint.
    require_permission(role, Permission::InitializeMint)?;

    let initialize = initialize_mint(
        &spl_token::ID,
        &mint.pubkey(),
        &payer.pubkey(),
        None,
        decimals,
    )?;

    // 9. Отримання Associated Token Account.
    let owner_account = get_associated_token_address(&payer.pubkey(), &mint.pubkey());

    // 10. Створення Token Account.
    require_permission(role, Permission::CreateTokenAccount)?;

    let create_account = create_associated_token_account(
        &payer.pubkey(),
        &payer.pubkey(),
        &mint.pubkey(),
        &spl_token::ID,
    );

    // 11. Підготовка випуску токенів.
    require_permission(role, Permission::MintTokens)?;

    let mint_tokens = mint_to(
        &spl_token::ID,
        &mint.pubkey(),
        &owner_account,
        &payer.pubkey(),
        &[],
        1_000_000_000,
    )?;

    // 12. Отримання останнього Blockhash.
    let blockhash = client.get_latest_blockhash()?;

    // 13. Формування підписаної транзакції.
    require_permission(role, Permission::SignTransaction)?;

    let transaction = Transaction::new_signed_with_payer(
        &[create_mint, initialize, create_account, mint_tokens],
        Some(&payer.pubkey()),
        &[&payer, &mint],
        blockhash,
    );

    println!("Transaction prepared successfully.");
    println!("Token account: {}", owner_account);
    println!("Signatures: {}", transaction.signatures.len());

    Ok(())
}

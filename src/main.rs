mod access_control;
mod auth;
mod crypto;
mod key_storage;
mod license;

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
use std::{env, error::Error, path::Path};

const SOLANA_RPC_URL: &str = "https://api.devnet.solana.com";

// Отримання ключа AES-256 із змінної середовища.
fn encryption_key_from_env() -> Result<[u8; 32], Box<dyn Error>> {
    let encoded = env::var("APP_ENCRYPTION_KEY_HEX")?;
    let decoded = hex::decode(encoded.trim())?;

    let key: [u8; 32] = decoded
        .try_into()
        .map_err(|_| std::io::Error::other("Encryption key must contain exactly 32 bytes"))?;

    Ok(key)
}

// Завантаження або створення зашифрованого гаманця.
fn load_or_create_keypair(
    encryption_key: &[u8; 32],
    path: &Path,
) -> Result<Keypair, Box<dyn Error>> {
    if path.exists() {
        println!("Loading encrypted keypair: {}", path.display());

        return key_storage::load_keypair(encryption_key, path);
    }

    println!("Generating a new Solana keypair.");

    let keypair = Keypair::new();

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    key_storage::save_keypair(&keypair, encryption_key, path)?;

    println!("Encrypted keypair saved: {}", path.display());

    Ok(keypair)
}

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

// Генерація bcrypt-хешу пароля.
fn generate_password_hash() -> Result<(), Box<dyn Error>> {
    let password = rpassword::prompt_password("Enter new password: ")?;

    let password_hash = auth::hash_password(&password)?;

    println!("Generated password hash:");
    println!("{}", password_hash);

    Ok(())
}

// Перевірка прав користувача.
fn require_permission(role: Role, permission: Permission) -> Result<(), Box<dyn Error>> {
    authorize(Some(role), permission)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Перевірка цифрового підпису ліцензії.
    println!("=== License verification ===");

    let license_valid = license::verify_license(
        Path::new("license.json"),
        Path::new("license.sig"),
        Path::new("keys/public_key.pem"),
    )?;

    if !license_valid {
        eprintln!("License verification failed. Access denied.");
        return Ok(());
    }

    println!("License verified successfully.");
    // Окрема команда для створення хешу пароля.
    if env::args().nth(1).as_deref() == Some("hash-password") {
        return generate_password_hash();
    }

    println!("=== SolanaTokenManager ===");

    // 1. Авторизація.
    if !authenticate()? {
        return Ok(());
    }

    // 2. Визначення ролі користувача.
    let role_value = env::var("APP_ROLE")?;

    let role =
        Role::parse(&role_value).ok_or("Invalid APP_ROLE. Use admin, operator or viewer.")?;

    println!("User role: {:?}", role);

    // 3. Перевірка прав доступу.
    require_permission(role, Permission::ViewInfo)?;
    println!("Access granted.");

    // 4. Підключення до Solana Devnet.
    let client = RpcClient::new(SOLANA_RPC_URL.to_string());

    // 5. Отримання ключа шифрування.
    require_permission(role, Permission::GenerateKeys)?;

    let encryption_key = encryption_key_from_env()?;

    // 6. Завантаження або створення гаманців.
    let payer = load_or_create_keypair(&encryption_key, Path::new("secrets/payer.enc"))?;

    let mint = load_or_create_keypair(&encryption_key, Path::new("secrets/mint.enc"))?;

    let decimals = 9;

    println!("Wallet: {}", payer.pubkey());
    println!("Mint: {}", mint.pubkey());

    // 7. Отримання мінімальної суми для rent exemption.
    let rent = client.get_minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN)?;

    // 8. Створення Mint Account.
    require_permission(role, Permission::CreateMint)?;

    let create_mint = create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        rent,
        spl_token::state::Mint::LEN as u64,
        &spl_token::ID,
    );

    // 9. Ініціалізація Mint.
    require_permission(role, Permission::InitializeMint)?;

    let initialize = initialize_mint(
        &spl_token::ID,
        &mint.pubkey(),
        &payer.pubkey(),
        None,
        decimals,
    )?;

    // 10. Отримання Associated Token Account.
    let owner_account = get_associated_token_address(&payer.pubkey(), &mint.pubkey());

    // 11. Створення Token Account.
    require_permission(role, Permission::CreateTokenAccount)?;

    let create_token_account = create_associated_token_account(
        &payer.pubkey(),
        &payer.pubkey(),
        &mint.pubkey(),
        &spl_token::ID,
    );

    // 12. Підготовка випуску токенів.
    require_permission(role, Permission::MintTokens)?;

    let mint_tokens = mint_to(
        &spl_token::ID,
        &mint.pubkey(),
        &owner_account,
        &payer.pubkey(),
        &[],
        1_000_000_000,
    )?;

    // 13. Отримання останнього Blockhash.
    let blockhash = client.get_latest_blockhash()?;

    // 14. Формування підписаної транзакції.
    require_permission(role, Permission::SignTransaction)?;

    let transaction = Transaction::new_signed_with_payer(
        &[create_mint, initialize, create_token_account, mint_tokens],
        Some(&payer.pubkey()),
        &[&payer, &mint],
        blockhash,
    );

    println!("Transaction prepared successfully.");
    println!("Token account: {}", owner_account);
    println!("Signatures: {}", transaction.signatures.len());

    // Транзакція поки не відправляється в мережу.
    Ok(())
}

mod access_control;
mod auth;
mod crypto;
mod key_storage;
mod license;
mod logger;

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

use tracing::{error, info, warn};

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
        info!(path = %path.display(), "Loading encrypted keypair");

        return key_storage::load_keypair(encryption_key, path);
    }

    info!(path = %path.display(), "Generating new Solana keypair");

    let keypair = Keypair::new();

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    key_storage::save_keypair(&keypair, encryption_key, path)?;

    info!(path = %path.display(), "Encrypted keypair saved");

    Ok(keypair)
}

// Перевірка пароля користувача.
fn authenticate() -> Result<bool, Box<dyn Error>> {
    let password_hash =
        env::var("APP_PASSWORD_HASH").map_err(|_| "APP_PASSWORD_HASH is not configured")?;

    println!("=== Authentication ===");

    info!("Authentication process started");

    for attempt in 1..=3 {
        let password = rpassword::prompt_password("Enter password: ")?;

        if auth::verify_password(&password, &password_hash)? {
            info!("User authentication successful");
            println!("Authentication successful.");
            return Ok(true);
        }

        warn!(
            attempt = attempt,
            attempts_remaining = 3 - attempt,
            "Authentication failed"
        );

        println!("Incorrect password. Attempts remaining: {}", 3 - attempt);
    }

    error!("Access denied after 3 failed attempts");
    println!("Access denied.");

    Ok(false)
}

// Генерація bcrypt-хешу пароля.
fn generate_password_hash() -> Result<(), Box<dyn Error>> {
    info!("Password hash generation started");

    let password = rpassword::prompt_password("Enter new password: ")?;

    let password_hash = auth::hash_password(&password)?;

    println!("Generated password hash:");
    println!("{}", password_hash);

    info!("Password hash generated successfully");

    Ok(())
}

// Перевірка прав користувача.
fn require_permission(role: Role, permission: Permission) -> Result<(), Box<dyn Error>> {
    authorize(Some(role), permission).map_err(|err| {
        warn!(
            role = ?role,
            permission = ?permission,
            "Access permission denied"
        );

        Box::<dyn Error>::from(err)
    })?;

    info!(
        role = ?role,
        permission = ?permission,
        "Access permission granted"
    );

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Ініціалізація системи логування.
    let _guard = logger::init_logger()?;

    info!("Application started");

    // Перевірка цифрового підпису ліцензії.
    println!("=== License verification ===");

    info!("License verification started");

    let license_valid = license::verify_license(
        Path::new("license.json"),
        Path::new("license.sig"),
        Path::new("keys/public_key.pem"),
    )?;

    if !license_valid {
        error!("License verification failed. Access denied.");
        eprintln!("License verification failed. Access denied.");
        return Ok(());
    }

    info!("License verified successfully");
    println!("License verified successfully.");

    // Окрема команда для створення хешу пароля.
    if env::args().nth(1).as_deref() == Some("hash-password") {
        info!("Password hash command selected");

        let result = generate_password_hash();

        if let Err(ref err) = result {
            error!(error = %err, "Password hash generation failed");
        }

        info!("Application finished");
        return result;
    }

    println!("=== SolanaTokenManager ===");

    // 1. Авторизація.
    if !authenticate()? {
        warn!("Application stopped: authentication failed");
        return Ok(());
    }

    // 2. Визначення ролі користувача.
    let role_value = env::var("APP_ROLE")?;

    let role =
        Role::parse(&role_value).ok_or("Invalid APP_ROLE. Use admin, operator or viewer.")?;

    info!(role = ?role, "User role identified");
    println!("User role: {:?}", role);

    // 3. Перевірка прав доступу.
    require_permission(role, Permission::ViewInfo)?;

    info!("Access granted");
    println!("Access granted.");

    // 4. Підключення до Solana Devnet.
    info!("Connecting to Solana Devnet");

    let client = RpcClient::new(SOLANA_RPC_URL.to_string());

    // 5. Отримання ключа шифрування.
    require_permission(role, Permission::GenerateKeys)?;

    let encryption_key = encryption_key_from_env()?;

    info!("Encryption key loaded from environment");

    // 6. Завантаження або створення гаманців.
    let payer = load_or_create_keypair(&encryption_key, Path::new("secrets/payer.enc"))?;

    let mint = load_or_create_keypair(&encryption_key, Path::new("secrets/mint.enc"))?;

    let decimals = 9;

    println!("Wallet: {}", payer.pubkey());
    println!("Mint: {}", mint.pubkey());

    info!("Wallets loaded successfully");

    // 7. Отримання мінімальної суми для rent exemption.
    info!("Requesting minimum balance for rent exemption");

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

    info!("Mint account creation instruction prepared");

    // 9. Ініціалізація Mint.
    require_permission(role, Permission::InitializeMint)?;

    let initialize = initialize_mint(
        &spl_token::ID,
        &mint.pubkey(),
        &payer.pubkey(),
        None,
        decimals,
    )?;

    info!("Mint initialization instruction prepared");

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

    info!("Token account creation instruction prepared");

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

    info!("Token minting instruction prepared");

    // 13. Отримання останнього Blockhash.
    info!("Requesting latest Solana blockhash");

    let blockhash = client.get_latest_blockhash()?;

    // 14. Формування підписаної транзакції.
    require_permission(role, Permission::SignTransaction)?;

    let transaction = Transaction::new_signed_with_payer(
        &[create_mint, initialize, create_token_account, mint_tokens],
        Some(&payer.pubkey()),
        &[&payer, &mint],
        blockhash,
    );

    info!(
        signatures = transaction.signatures.len(),
        "Transaction prepared successfully"
    );

    println!("Transaction prepared successfully.");
    println!("Token account: {}", owner_account);
    println!("Signatures: {}", transaction.signatures.len());

    // Транзакція поки не відправляється в мережу.
    info!("Application finished successfully");

    Ok(())
}

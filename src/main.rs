mod auth;

use solana_program_pack::Pack;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};

use spl_associated_token_account::{
    get_associated_token_address,
    instruction::create_associated_token_account,
};

use spl_token::instruction::{initialize_mint, mint_to};
use std::error::Error;

const SOLANA_RPC_URL: &str =
    "https://api.devnet.solana.com";

fn authenticate() -> Result<bool, Box<dyn Error>> {
    let password_hash = std::env::var("APP_PASSWORD_HASH")
        .map_err(|_| "APP_PASSWORD_HASH is not configured")?;

    println!("=== Authentication ===");

    for attempt in 1..=3 {
        let password =
            rpassword::prompt_password("Enter password: ")?;

        if auth::verify_password(&password, &password_hash)? {
            println!("Authentication successful.");
            return Ok(true);
        }

        println!(
            "Incorrect password. Attempts remaining: {}",
            3 - attempt
        );
    }

    println!("Access denied.");
    Ok(false)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== SolanaTokenManager ===");

    if !authenticate()? {
        return Ok(());
    }

    let client = RpcClient::new(
        SOLANA_RPC_URL.to_string()
    );

    let payer = Keypair::new();
    let mint = Keypair::new();
    let decimals = 9;

    println!("Wallet: {}", payer.pubkey());
    println!("Mint: {}", mint.pubkey());

    let rent = client
        .get_minimum_balance_for_rent_exemption(
            spl_token::state::Mint::LEN
        )?;

    let create_mint = system_instruction::create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        rent,
        spl_token::state::Mint::LEN as u64,
        &spl_token::ID,
    );

    let initialize = initialize_mint(
        &spl_token::ID,
        &mint.pubkey(),
        &payer.pubkey(),
        None,
        decimals,
    )?;

    let owner_account = get_associated_token_address(
        &payer.pubkey(),
        &mint.pubkey(),
    );

    let create_account = create_associated_token_account(
        &payer.pubkey(),
        &payer.pubkey(),
        &mint.pubkey(),
        &spl_token::ID,
    );

    let mint_tokens = mint_to(
        &spl_token::ID,
        &mint.pubkey(),
        &owner_account,
        &payer.pubkey(),
        &[],
        1_000_000_000,
    )?;

    let blockhash = client.get_latest_blockhash()?;

    let transaction = Transaction::new_signed_with_payer(
        &[
            create_mint,
            initialize,
            create_account,
            mint_tokens,
        ],
        Some(&payer.pubkey()),
        &[&payer, &mint],
        blockhash,
    );

    println!("Transaction prepared successfully.");
    println!("Token account: {}", owner_account);
    println!("Signatures: {}", transaction.signatures.len());

    Ok(())
}
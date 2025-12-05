use solana_sdk::{
    signature::{Keypair, Signer},
    pubkey::Pubkey,
};
use std::fs;
use std::path::Path;

pub fn generate_wallet() -> Keypair {
    Keypair::new()
}

pub fn get_address(keypair: &Keypair) -> Pubkey {
    keypair.pubkey()
}

pub fn save_wallet(keypair: &Keypair, path: &str) -> Result<(), std::io::Error> {
    let keypair_bytes = keypair.to_bytes();
    let json = serde_json::to_string(&keypair_bytes.to_vec())?;
    fs::write(path, json)?;
    println!("✅ Wallet saved to: {}", path);
    Ok(())
}

pub fn load_wallet(path: &str) -> Result<Keypair, Box<dyn std::error::Error>> {
    if !Path::new(path).exists() {
        return Err("Wallet file not found".into());
    }
    
    let json = fs::read_to_string(path)?;
    let bytes: Vec<u8> = serde_json::from_str(&json)?;
    let keypair = Keypair::from_bytes(&bytes)?;
    
    println!("✅ Wallet loaded from: {}", path);
    Ok(keypair)
}

pub fn display_wallet_info(keypair: &Keypair) {
    println!("\n💳 Wallet Information:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📍 Address (Public Key): {}", keypair.pubkey());
    println!("🔐 This is your RECEIVING address");
    println!("📋 Share this with customers to receive payments");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
}

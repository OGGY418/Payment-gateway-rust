use solana_sdk::signature::{Keypair, Signer};
use payment_gateway_rust::utils::wallet;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    println!("\n🌐 Solana Devnet Wallet Setup");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("Choose an option:");
    println!("1. Generate NEW wallet");
    println!("2. Load EXISTING wallet");
    print!("\nEnter choice (1 or 2): ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    let keypair = match choice {
        "1" => {
            println!("\n🔑 Generating new wallet...");
            let kp = wallet::generate_wallet();
            
            if let Err(e) = wallet::save_wallet(&kp, "wallet.json") {
                eprintln!("❌ Failed to save wallet: {}", e);
                return;
            }
            
            kp
        }
        "2" => {
            println!("\n📂 Loading wallet from wallet.json...");
            match wallet::load_wallet("wallet.json") {
                Ok(kp) => kp,
                Err(e) => {
                    eprintln!("❌ Failed to load wallet: {}", e);
                    return;
                }
            }
        }
        _ => {
            println!("❌ Invalid choice!");
            return;
        }
    };

    wallet::display_wallet_info(&keypair);

    println!("🌐 Connecting to Solana Devnet...");
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(
        rpc_url.to_string(),
        CommitmentConfig::confirmed(),
    );

    match client.get_balance(&keypair.pubkey()) {
        Ok(balance) => {
            let sol_balance = balance as f64 / LAMPORTS_PER_SOL as f64;
            println!("💰 Current Balance: {} SOL", sol_balance);
            
            if balance == 0 {
                println!("\n⚠️  Your wallet has 0 SOL!");
                println!("🎁 You need devnet SOL to test transactions.");
                println!("\n📋 To get free devnet SOL:");
                println!("   1. Visit: https://faucet.solana.com");
                println!("   2. Paste your address: {}", keypair.pubkey());
                println!("   3. Select 'Devnet' from dropdown");
                println!("   4. Click 'Request Airdrop'");
                println!("   5. You'll get 2 SOL for free!\n");
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to get balance: {}", e);
        }
    }

    println!("\n📝 Add this to your .env file:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("SOLANA_RPC_URL=https://api.devnet.solana.com");
    println!("SOLANA_NETWORK=devnet");
    println!("WALLET_ADDRESS={}", keypair.pubkey());
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("✅ Setup complete!");
    println!("💾 Wallet saved to: wallet.json");
    println!("⚠️  Keep wallet.json SECRET! It contains your private key!\n");
}

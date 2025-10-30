use solana_sdk::signature::{Keypair, Signer};
use serde_json::json;

fn main() {
    println!("🔑 Generating Solana Keypairs\n");
    
    // Generate keypair for Alice
    let alice = Keypair::new();
    println!("Alice:");
    println!("  Public Key:  {}", alice.pubkey());
    println!("  Private Key: {}", json!(alice.to_bytes().to_vec()));
    println!("  Private Key (base58): {}\n", bs58::encode(alice.to_bytes()).into_string());
    
    // Generate keypair for Bob
    let bob = Keypair::new();
    println!("Bob:");
    println!("  Public Key:  {}", bob.pubkey());
    println!("  Private Key: {}", json!(bob.to_bytes().to_vec()));
    println!("  Private Key (base58): {}\n", bs58::encode(bob.to_bytes()).into_string());
    
    println!("💡 Use these keys for testing transfers!");
    println!("   Keep the private keys secret in production!");
}
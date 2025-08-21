use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::str::FromStr;

fn main()  {
    let program_id = Pubkey::from_str("yzKg3w29hwBimp9Fp2PFCge9CZSJfJm6Ndv86G9mr4N").unwrap();
    
    let client = RpcClient::new("https://api.devnet.solana.com");
    let payer = Keypair::new();
    
    
    println!("Getting SOL for transactions...");
    let airdrop_sig = client.request_airdrop(&payer.pubkey(), 1_000_000_000).unwrap();
    println!("Airdrop: {}", airdrop_sig);
    
    // Wait a moment for airdrop
    std::thread::sleep(std::time::Duration::from_secs(5));
    
    // Test all operations
    test_operation(&client, &payer, &program_id, 0, 15, 7, "Add");
    test_operation(&client, &payer, &program_id, 1, 20, 8, "Subtract");  
    test_operation(&client, &payer, &program_id, 2, 6, 4, "Multiply");
    test_operation(&client, &payer, &program_id, 3, 24, 6, "Divide");
    test_operation(&client, &payer, &program_id, 4, 17, 5, "Modulus");
    test_operation(&client, &payer, &program_id, 5, 3, 4, "Power");
}
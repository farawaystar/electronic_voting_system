use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
//use solana_sdk::transaction::TransactionConfig;
use solana_program::{
    pubkey::Pubkey,    
};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
    instruction::{AccountMeta, Instruction},
};
use std::str::FromStr;

// Replace with your program ID
// const PROGRAM_ID: &str = "Y44mbnZciSj2joW6Cf94DX1ZQnffLwLxNMFp95MYNbF29";

fn main() {
    let rpc_url = "https://api.devnet.solana.com".to_string();
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let payer = Keypair::new();
    // let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
    let program_id = Pubkey::from_str("6HZ1ctyoSAeRuzSDJnqy63ihi3YfZ5sDaizcz8HQtnbW").unwrap();

    // Airdrop some SOL to the payer for transaction fees
    // let airdrop_signature = client.request_airdrop(&payer.pubkey(), 1_000_000_000).unwrap();
    // client.confirm_transaction(&airdrop_signature).unwrap();

    match request_airdrop(&client, &payer.pubkey(), 1_000_000_000) {
        Ok(_) => println!("Airdrop successful"),
        Err(e) => {
            eprintln!("Failed to request airdrop: {:?}", e);
            return;
        }
    }

    // Initialize voting and store the vote state account pubkey
    // let vote_state_pubkey = initialize_voting(&client, &payer, &program_id);

    let vote_state_pubkey = match initialize_voting(&client, &payer, &program_id) {
        Ok(pubkey) => pubkey,
        Err(e) => {
            eprintln!("Failed to initialize voting: {:?}", e);
            return;
        }
    };

    // Cast a vote
    // cast_vote(&client, &payer, &program_id, &vote_state_pubkey);

    if let Err(e) = cast_vote(&client, &payer, &program_id, &vote_state_pubkey) {
        eprintln!("Failed to cast vote: {:?}", e);
    }
}

fn request_airdrop(client: &RpcClient, pubkey: &Pubkey, lamports: u64) -> Result<(), Box<dyn std::error::Error>> {
    let airdrop_signature = client.request_airdrop(pubkey, lamports)?;
    // Fetch the latest blockhash
    let recent_blockhash = client.get_latest_blockhash()?;
    // Confirm the transaction with spinner
    client.confirm_transaction_with_spinner(&airdrop_signature, &recent_blockhash, CommitmentConfig::confirmed())?;    
    Ok(())
}


//fn initialize_voting(client: &RpcClient, payer: &Keypair, program_id: &Pubkey)-> Pubkey {
fn initialize_voting(client: &RpcClient, payer: &Keypair, program_id: &Pubkey) -> Result<Pubkey, Box<dyn std::error::Error>> {
    let vote_state_account = Keypair::new();
    let vote_state_pubkey = vote_state_account.pubkey();
    let instruction_data = vec![
        0, // Instruction index for Initialize
        0, 0, 0, 0, 0, 0, 0, 0, // Start time (current time)
        255, 255, 255, 255, 0, 0, 0, 0, // End time (far in the future)
    ];

    let instruction = Instruction::new_with_bytes(
        *program_id,
        &instruction_data,
        vec![
            AccountMeta::new(vote_state_account.pubkey(), false),
        ],
    );

    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer, &vote_state_account],
        recent_blockhash,
    );

    // let signature = client.send_and_confirm_transaction(&transaction).unwrap();
    // let signature = client.send_and_confirm_transaction_with_spinner(&transaction)?;
    let signature = client.send_and_confirm_transaction_with_spinner_and_commitment(
        &transaction,
        CommitmentConfig::confirmed(),
    )?; 
    println!("Voting initialized. Signature: {}", signature);    
    
    // vote_state_pubkey
    Ok(vote_state_pubkey)
}

// fn cast_vote(client: &RpcClient, payer: &Keypair, program_id: &Pubkey, vote_state_pubkey: &Pubkey) {
fn cast_vote(client: &RpcClient, payer: &Keypair, program_id: &Pubkey, vote_state_pubkey: &Pubkey) -> Result<(), Box<dyn std::error::Error>> {
    let voter_account = Keypair::new();

    let instruction_data = vec![1]; // Instruction index for Vote

    let instruction = Instruction::new_with_bytes(
        *program_id,
        &instruction_data,
        vec![
            AccountMeta::new(*vote_state_pubkey, false),
            AccountMeta::new(voter_account.pubkey(), true),
        ],
    );

    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer, &voter_account],
        recent_blockhash,
    );

    //let signature = client.send_and_confirm_transaction(&transaction).unwrap();
    //let signature = client.send_and_confirm_transaction_with_spinner(&transaction)?;
    let signature = client.send_and_confirm_transaction_with_spinner_and_commitment(
        &transaction,
        CommitmentConfig::confirmed(),
    )?;
    
    println!("Vote cast. Signature: {}", signature);    
    Ok(())
}

use solana_client::rpc_client::RpcClient;
use log::debug;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, read_keypair_file, Signer},
    pubkey::Pubkey,
    transaction::Transaction,
    instruction::{AccountMeta, Instruction},
};
use std::str::FromStr;
use std::error::Error;
use std::env;
use solana_sdk::sysvar::rent::Rent;
use solana_sdk::vote::state::VoteState;
use solana_sdk::system_instruction;
// use solana_sdk::sysvar::Sysvar;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let rpc_url = "https://api.devnet.solana.com".to_string();
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // Load keypairs from JSON 
    debug!("Executing query1: Load keypairs from JSON");
    // Display the current working directory concisely
    println!("The current working directory is: {}",
        env::current_dir()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|e| format!("Failed to get current directory: {}", e))
    );
    let payer = read_keypair_file("payer_keypair.json")?;
    debug!("Executing query2: Loaded payer keypair from JSON");
    let vote_state_account = read_keypair_file("vote_state_keypair.json")?;
    let voter_account = read_keypair_file("voter_keypair.json")?;

    let program_id = Pubkey::from_str("44mbnZciSj2joW6Cf94DX1ZQnffLwLxNMFp95MYNbF29")?;
    
    println!("Payer pubkey: {:?}", payer.pubkey());
    println!("Vote state pubkey: {:?}", vote_state_account.pubkey());
    println!("Voter pubkey: {:?}", voter_account.pubkey()); 

    // Initialize vote_state_account
    initialize_vote_state_account(&client, &payer, &program_id, &vote_state_account)?;

    // Initialize voting and store the vote state account pubkey
    let vote_state_pubkey = initialize_voting(&client, &payer, &program_id, &vote_state_account)?;

    // Cast a vote
    cast_vote(&client, &payer, &program_id, &vote_state_pubkey, &voter_account)?;
    Ok(())
}

fn initialize_voting(client: &RpcClient, payer: &Keypair, program_id: &Pubkey, vote_state_account: &Keypair) -> Result<Pubkey, Box<dyn Error>> {
    
    let vote_state_pubkey = vote_state_account.pubkey();
    let instruction_data = vec![
        0, // Instruction index for Initialize
        0, 0, 0, 0, 0, 0, 0, 0, // Start time (current time)
        255, 255, 255, 255, 0, 0, 0, 0, // End time (far in the future)
    ];

    let instruction = Instruction::new_with_bytes(
        *program_id,
        &instruction_data,
        vec![AccountMeta::new(vote_state_pubkey, true)],
    );

    let recent_blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer, vote_state_account],
        recent_blockhash,
    );

    let signature = client.send_and_confirm_transaction_with_spinner_and_commitment(
        &transaction,
        CommitmentConfig::confirmed(),
    )?;

    println!("Voting initialized. Signature: {}", signature);    
    
    Ok(vote_state_pubkey)
}

fn cast_vote(client: &RpcClient, payer: &Keypair, program_id: &Pubkey, vote_state_pubkey: &Pubkey, voter_account: &Keypair) -> Result<(), Box<dyn Error>> {
    let instruction_data = vec![1]; // Instruction index for Vote

    let instruction = Instruction::new_with_bytes(
        *program_id,
        &instruction_data,
        vec![
            AccountMeta::new(*vote_state_pubkey, false),
            AccountMeta::new(voter_account.pubkey(), true),
        ],
    );

    let recent_blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer, voter_account],
        recent_blockhash,
    );

    let signature = client.send_and_confirm_transaction_with_spinner_and_commitment(
        &transaction,
        CommitmentConfig::confirmed(),
    )?;
    
    println!("Vote cast. Signature: {}", signature);    
    Ok(())
}

fn initialize_vote_state_account(client: &RpcClient, payer: &Keypair, program_id: &Pubkey, vote_state_account: &Keypair) -> Result<(), Box<dyn Error>> {    
    let vote_state_pubkey = vote_state_account.pubkey();

    // Check if the account already exists
    let account_exists = client.get_account(&vote_state_pubkey).is_ok();

    if !account_exists {
        // Create the account if it doesn't exist
        let rent = Rent::default();
        let required_lamports = rent.minimum_balance(std::mem::size_of::<VoteState>());
        
        let create_account_ix = system_instruction::create_account(
            &payer.pubkey(),
            &vote_state_pubkey,
            required_lamports,
            std::mem::size_of::<VoteState>() as u64,
            &program_id,
        );
        let recent_blockhash = client.get_latest_blockhash()?;
        let transaction = Transaction::new_signed_with_payer(
            &[create_account_ix],
            Some(&payer.pubkey()),
            &[payer, vote_state_account],
            recent_blockhash,
        );

        client.send_and_confirm_transaction_with_spinner_and_commitment(&transaction, CommitmentConfig::confirmed())?;
        println!("New Vote state account created successfully.");
    }

        // Check if the account is initialized
        let account_data = client.get_account_data(&vote_state_pubkey)?;
        let is_initialized = account_data[0] != 0;

    if !is_initialized {
        // Initialize the account if it's not initialized
        let instruction_data = vec![
            0, // Instruction index for Initialize
            0, 0, 0, 0, 0, 0, 0, 0, // Start time (current time)
            255, 255, 255, 255, 0, 0, 0, 0, // End time (far in the future)
        ];

        let instruction = Instruction::new_with_bytes(
            *program_id,
            &instruction_data,
            vec![AccountMeta::new(vote_state_pubkey, false)],
        );

        let recent_blockhash = client.get_latest_blockhash()?;
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[payer],
            recent_blockhash,
        );

        let signature = client.send_and_confirm_transaction_with_spinner_and_commitment(
            &transaction,
            CommitmentConfig::confirmed(),
        )?;

        println!("Vote state account initialized. Signature: {}", signature);
    } else {
        println!("Vote state account already initialized. Skipping initialization.");

        }    

    Ok(())
}
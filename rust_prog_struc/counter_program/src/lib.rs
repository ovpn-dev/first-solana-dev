use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},   
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Deserialize the instruction using Borsh
    let instruction = CounterInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    // Route to the appropriate handler based on the instruction
    match instruction {
        CounterInstruction::InitializeCounter { initial_value } => {
            process_initialize_counter(program_id, accounts, initial_value)?
        }
        CounterInstruction::IncrementCounter => {
            process_increment_counter(program_id, accounts)?
        }
    };

    Ok(())
}

// Handler function for initializing a counter
fn process_initialize_counter(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    initial_value: u64,
) -> ProgramResult {
    // Implementation coming next
    Ok(())
}

// Handler function for incrementing a counter
fn process_increment_counter(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    // Implementation coming next
    Ok(())
}


//Define the data structure for our counter account 
// the derive macro enable automatic serialization/deserialization 
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CounterInstruction {
    // Store the counter value
    InitializeCounter { initial_value: u64},
    IncrementCounter,
    pub count: u64
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CounterAccount {
    // Store the counter value
    pub count: u64
}

// Test module - only compiled when running tests
#[cfg(test)]
mod test {
    use super::*;  // Import everything from the parent module
    use litesvm::LiteSVM;
    use solana_sdk::{
        account::ReadableAccount,
        instruction::{AccountMeta, Instruction},
        message::Message,
        signature::{Keypair, Signer},
        system_program,
        transaction::Transaction,
    };

    #[test]
    fn test_counter_program() {
        // Create a new instance of the Solana VM for testing
        let mut svm = LiteSVM::new();

        // Create a keypair for the transaction payer
        let payer = Keypair::new();

        // Fund the payer account with 1 SOL (1 billion lamports)
        // This is needed to pay for transaction fees and account creation
        svm.airdrop(&payer.pubkey(), 1_000_000_000)
            .expect("Failed to airdrop");

    }
}
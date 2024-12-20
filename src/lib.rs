use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    program_error::ProgramError,
    clock::Clock,
    sysvar::Sysvar,
};

// Define the structure of our voting data
#[derive(Debug, Default)]
struct VoteState {
    is_initialized: bool,
    vote_count: u64,
    start_time: i64,
    end_time: i64,
}

// Define the instruction data structure
enum VoteInstruction {
    Initialize { start_time: i64, end_time: i64 },
    Vote,
}

// Declare the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint implementation
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = VoteInstruction::unpack(instruction_data)?;

    match instruction {
        VoteInstruction::Initialize { start_time, end_time } => {
            initialize_voting(accounts, start_time, end_time)
        }
        VoteInstruction::Vote => vote(accounts),
    }
}

fn initialize_voting(accounts: &[AccountInfo], start_time: i64, end_time: i64) -> ProgramResult {
    let vote_state_account = &accounts[0];
    
    if !vote_state_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    let mut vote_state = VoteState::unpack(&vote_state_account.data.borrow())?;
    
    if vote_state.is_initialized {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    vote_state.is_initialized = true;
    vote_state.start_time = start_time;
    vote_state.end_time = end_time;
    vote_state.vote_count = 0;

    VoteState::pack(&vote_state, &mut vote_state_account.data.borrow_mut())?;

    msg!("Voting initialized. Start time: {}, End time: {}", start_time, end_time);
    Ok(())
}

fn vote(accounts: &[AccountInfo]) -> ProgramResult {
    let vote_state_account = &accounts[0];
    let voter_account = &accounts[1];
    let clock = Clock::get()?;

    if !vote_state_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    let mut vote_state = VoteState::unpack(&vote_state_account.data.borrow())?;

    if !vote_state.is_initialized {
        return Err(ProgramError::UninitializedAccount);
    }

    if clock.unix_timestamp < vote_state.start_time || clock.unix_timestamp > vote_state.end_time {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Check if the voter has already voted
    if voter_account.data.borrow()[0] != 0 {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // Mark the voter as having voted
    voter_account.data.borrow_mut()[0] = 1;

    vote_state.vote_count += 1;
    VoteState::pack(&vote_state, &mut vote_state_account.data.borrow_mut())?;    

    msg!("Vote cast successfully. Total votes: {}", vote_state.vote_count);
    Ok(())
}

impl VoteState {
    fn unpack(data: &[u8]) -> Result<Self, ProgramError> {
        if data.len() < std::mem::size_of::<Self>() {
            return Err(ProgramError::InvalidAccountData);
        }
        let vote_state = Self {
            is_initialized: data[0] != 0,
            vote_count: u64::from_le_bytes(data[1..9].try_into().unwrap()),
            start_time: i64::from_le_bytes(data[9..17].try_into().unwrap()),
            end_time: i64::from_le_bytes(data[17..25].try_into().unwrap()),
        };
        Ok(vote_state)
    }

    fn pack(&self, dst: &mut [u8]) -> Result<(), ProgramError> {    
        if dst.len() < std::mem::size_of::<Self>() {
            return Err(ProgramError::InvalidAccountData);
        }
        dst[0] = self.is_initialized as u8;
        dst[1..9].copy_from_slice(&self.vote_count.to_le_bytes());
        dst[9..17].copy_from_slice(&self.start_time.to_le_bytes());
        dst[17..25].copy_from_slice(&self.end_time.to_le_bytes());
        Ok(())
    }
}

impl VoteInstruction {
    fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        if input.is_empty() {
            return Err(ProgramError::InvalidInstructionData);
        }
        match input[0] {
            0 => {
                if input.len() < 17 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let start_time = i64::from_le_bytes(input[1..9].try_into().unwrap());
                let end_time = i64::from_le_bytes(input[9..17].try_into().unwrap());
                Ok(VoteInstruction::Initialize { start_time, end_time })
            }
            1 => Ok(VoteInstruction::Vote),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

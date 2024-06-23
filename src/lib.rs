use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    program_error::ProgramError,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct InstructionData {
    pub data: u64,
}

entrypoint!(process_instruction);
 
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction_data = InstructionData::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    let accounts_iter = &mut accounts.iter();
    let storage_account = next_account_info(accounts_iter)?;

    if storage_account.owner != program_id {
        msg!("Storage account is not owned by the program");
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut data = storage_account.try_borrow_mut_data()?;
    data[..8].copy_from_slice(&instruction_data.data.to_le_bytes());

    msg!("Data stored successfully: {}", instruction_data.data);
    Ok(())
}
use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey, msg, program_error::ProgramError
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
enum TestInstruction {
    Exec { lamports: u64, description: String }
}

entrypoint!(execute);

fn execute(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let mut data = instruction_data;

    let instr_result: Result<TestInstruction, std::io::Error> = borsh::BorshDeserialize::deserialize(&mut data);

    match instr_result {
        Ok(instruction) => {
            msg!(
                "transaction_data: {}: {} accounts, data={:?}",
                program_id,
                accounts.len(),
                instruction
            );
            return Ok(());
        },
        Err(error) => {
            msg!("Error trying to parse instruction: {:?}", error);
            return Err(ProgramError::InvalidInstructionData);
        }
    }
}
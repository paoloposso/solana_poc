use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey, msg
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

    msg!("msg: {:?}", instruction_data);

    let mut t = instruction_data;

    let instr: TestInstruction = borsh::BorshDeserialize::deserialize(&mut t).unwrap();

    msg!(
        "transaction_data: {}: {} accounts, data={:?}",
        program_id,
        accounts.len(),
        instr
    );
    Ok(())
}
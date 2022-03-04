use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey, msg
};

entrypoint!(execute);

fn execute(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!(
        "transaction_data: {}: {} accounts, data={:?}",
        program_id,
        accounts.len(),
        String::from_utf8(instruction_data.to_vec()).unwrap()
    );
    Ok(())
}
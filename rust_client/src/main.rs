use std::{error::Error, str::FromStr};

use borsh::{BorshSerialize, BorshDeserialize};
use solana_client::rpc_client::RpcClient;
use solana_program::{pubkey::Pubkey, instruction::Instruction};
use solana_sdk::{signature::{Keypair, Signature}, transaction, signer::Signer};

const URL: &str = "https://api.devnet.solana.com";
const LAMPORTS_PER_SOL: f64 = 1000000000.0;

fn main() {
    let rpc_client = RpcClient::new(URL);
    let payer = &create_keypair();
    let _ = request_air_drop(&rpc_client, &payer.pubkey(), 1.0).unwrap();
    let _ = exec_program(payer);
}

pub fn create_keypair() -> Keypair {
    Keypair::new()
}

pub fn request_air_drop(rpc_client: &RpcClient, pub_key: &Pubkey, amount_sol: f64) -> Result<Signature, Box<dyn Error>> {
    let sig = rpc_client.request_airdrop(&pub_key, (amount_sol * LAMPORTS_PER_SOL) as u64)?;
    loop {
        let confirmed = rpc_client.confirm_transaction(&sig)?;
        if confirmed {
            break;
        }
    }
    println!("Airdrop for {:?}", pub_key.to_owned());
    Ok(sig)
}

pub fn exec_program(payer: &Keypair) -> core::result::Result<(), Box<dyn Error>>{
    let rpc_client = RpcClient::new(URL);
    let program_key = Pubkey::from_str("HAKGVjYFMfhsaTHMk215ZeVbfuYn1fJgwNsE9iJ24zZ9")?;
    
    match execute_program(&rpc_client, &payer, &program_key) {
        Ok(_) => {},
        Err(err) => println!("{:?}", err),
    }

    Ok(())
}

#[derive(BorshSerialize, BorshDeserialize)]
enum TestInstruction {
    Exec { lamports: u64, description: String }
}

pub fn execute_program(rpc_client: &RpcClient, payer: &Keypair, program_id: &Pubkey) 
        -> core::result::Result<Signature, Box<dyn Error>> {
    let latest_blockhash = rpc_client.get_latest_blockhash()?;
    let bank_instruction = TestInstruction::Exec { lamports: 46, description: "tst".to_owned() };

    let instruction = Instruction::new_with_borsh(
        *program_id,
        &bank_instruction,
        vec![],
    );

    println!("Executing...");

    let mut tx = 
        transaction::Transaction::new_with_payer(&vec!(instruction), Some(&payer.pubkey()));

    tx.sign(&[payer], latest_blockhash);
    
    Ok(rpc_client.send_and_confirm_transaction(&tx)?)
}

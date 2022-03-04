import web3 = require('@solana/web3.js');

const connection = new web3.Connection(web3.clusterApiUrl("devnet"));

async function main() {

    console.log('generating key...');
    const signer = web3.Keypair.generate();

    let airdropSignature = await connection.requestAirdrop(
        signer.publicKey,
        web3.LAMPORTS_PER_SOL
    );

    await connection.confirmTransaction(airdropSignature);

    const programId = new web3.PublicKey('HAKGVjYFMfhsaTHMk215ZeVbfuYn1fJgwNsE9iJ24zZ9');

    const transaction = new web3.Transaction().add(
        new web3.TransactionInstruction({
            keys: [],
            programId,
            data: Buffer.from('test')
        })
    );

    console.log('sending transaction...');
    var s = await web3.sendAndConfirmTransaction(connection, transaction, [signer]); 
    console.log(`sig: ${s}`);
}

main().then(() => process.exit(0))
    .catch(error => {
        console.error(error);
        process.exit(1);
    });
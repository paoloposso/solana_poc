import web3 = require('@solana/web3.js');

const connection = new web3.Connection(web3.clusterApiUrl("devnet"));

async function main() {
    const signer = web3.Keypair.generate();

    let airdropSignature = await connection.requestAirdrop(
        signer.publicKey,
        web3.LAMPORTS_PER_SOL
    );

    await connection.confirmTransaction(airdropSignature);

    const programId = new web3.PublicKey('8aj79DwFzfWV9qpuLAgpAkV1FKMeQpkVduiWcfg1WBrx');

    const transaction = new web3.Transaction().add(
        new web3.TransactionInstruction({
            keys: [],
            programId,
            data: Buffer.from('data sent')
        })
    );

    var s = await web3.sendAndConfirmTransaction(connection, transaction, [signer]); 
    console.log(`sig: ${s}`);
}

main().then(() => process.exit(0))
    .catch(error => {
        console.error(error);
        process.exit(1);
    });
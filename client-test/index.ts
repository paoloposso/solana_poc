import web3 = require('@solana/web3.js');

const connection = new web3.Connection(web3.clusterApiUrl("devnet"));
const key: Uint8Array = Uint8Array.from([30,46,176,83,4,127,47,102,208,146,79,222,113,45,3,57,27,224,70,38,239,188,138,93,253,5,73,176,133,211,219,136,67,167,68,138,34,155,122,107,94,42,64,226,185,78,33,171,247,238,29,20,81,109,212,189,31,91,180,81,164,2,80,83]);

async function main() {
    const signer = web3.Keypair.fromSecretKey(key);

    try {
        let balance = await connection.getBalance(signer.publicKey);
        console.log(`SOL: ${balance / web3.LAMPORTS_PER_SOL}`);
    } catch (error) {
        console.error(error);
    }
}

main().then(() => process.exit(0))
    .catch(error => {
        console.error(error);
        process.exit(1);
    });
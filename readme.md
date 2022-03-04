# Solana Program Example

## Requirements

###  WSL (if running on Windows)
See https://docs.microsoft.com/en-us/windows/wsl/install

### Solana Tool Suite
See https://docs.solana.com/cli/install-solana-cli-tools

### Install Node
See https://nodejs.org/en/download/

## Before you start:

#### devnet
For this example we are using the Solana devnet: Access it: https://explorer.solana.com/?cluster=devnet.
The transactions will be shown there.

To make sure where you are pointing to the devnet, run ```solana config get```. The following output will be shown:

```
Config File: /home/paolo/.config/solana/cli/config.yml
RPC URL: https://api.devnet.solana.com 
WebSocket URL: wss://api.devnet.solana.com/ (computed)
Keypair Path: /home/paolo/.config/solana/id.json 
Commitment: confirmed 
```
If it's not set to https://api.devnet.solana.com, run the following command:

```
solana config set --url devnet
```

Run  ```solana config get``` again to make sure you are good to go.

#### Signer
The transaction signer, in this case, is generated at the same moment we are executing it.
We could, instead, use your locally generated key pair to sign the transaction.

#### My key pair
You can generate a key pair locally using the ```solana-keygen new``` command, like so:
```
mkdir ~/my-solana-wallet
solana-keygen new --outfile ~/my-solana-wallet/my-keypair.json
```
This will generate a File System Wallet Keypair

## Deploying the Solana Program
- Navigate to the _solana_program_ folder
- run the command ```cargo build-bpf```

The instructions to deploy the program will be shown after building it:
```
To deploy this program:
  $ solana program deploy /home/paolo/git/solana/solana_program/target/deploy/solana_program.so
The program address will default to this keypair (override with --program-id):
  /home/paolo/git/solana/solana_program/target/deploy/solana_program-keypair.json
```

- Run the ```solana program deploy /home/paolo/git/solana/solana_program/target/deploy/solana_program.so``` setting your correct directory.
The Program Id will be output. In my case it was the following:

```
Program Id: 8aj79DwFzfWV9qpuLAgpAkV1FKMeQpkVduiWcfg1WxXx
```

### Test the Program using a Node Client:

#### Update the Client to run your program
- Open ```index.ts```
- Change the following code snippet to include the Program Id generated when you published the program
```
const programId = new web3.PublicKey('8aj79DwFzfWV9qpuLAgpAkV1FKMeQpkVduiWcfg1WxXx');
```
(use your generated program id instead)

#### Running the Client
- Navigate to the _client_test_ directory
- Run the command ```npm install``` to install the dependencies
- (in a different terminal) Run the command ```solana logs 8aj79DwFzfWV9qpuLAgpAkV1FKMeQpkVduiWcfg1WxXx```. This will allow you to see the deployed Solana Program logs while executing the transaction.
- Run the command ```npx ts-node index.ts``` to send the transaction to the Program.

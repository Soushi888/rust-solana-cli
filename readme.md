# Rust Solana CLI program

This program is of Proof of Concept for creating a Solana Token (SPL Token), minting 10_000_000 of it, disable theminting and show the balance for this token with the command `rust-solana-cli create-token --name=<token-name>`.

The `rust-solana-cli show-accounts` list all the tokens of the current account;

The program need you to have the `Solana Toolchain` installed, a keypair generated (`solana-keygen new`), the RPC URL set to the solana testnet (`http://api.testnet.solana.com`) at least some SOL air dropped on your account.

### Setting commands
```
$ solana-keygen new
$ solana config set --url https://api.testnet.solana.com

if needed :
$ solana config set --keypair ${HOME}/new-keypair.json

$ solana airdrop 1
```

## Technologies used

- [Rust](https://www.rust-lang.org)
- [Solana Token Program](https://github.com/solana-labs/solana-program-library/tree/master/token)

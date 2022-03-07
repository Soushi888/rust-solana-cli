# Rust Solana libp2p program

This program is of Proof of Concept for creating, minting and register a token name of a Solana Token (SPL Token). 

## Technologies used

- [Rust](https://www.rust-lang.org)
- [libp2p](https://docs.rs/libp2p/latest/libp2p/index.html)
  - [websocket](https://docs.rs/libp2p/latest/libp2p/websocket/index.html)
- [Solana Token Program](https://github.com/solana-labs/solana-program-library/tree/master/token)
- [Solana Name Service](https://spl.solana.com/name-service)

## To do

- [ ] Create a fungible token on the Solana blockchain testnet for the connected user (solana wallet)
  - [ ] Create a fungible token
  - [ ] Create token account
  - [ ] Mint `10_000_000` of the created token
  - [ ] Disable futur mining
  - [ ] Register token name
- [ ] Show account infos

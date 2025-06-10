# Metaplex Core NFT Asset Example
An example repo for how to mint Solana NFT assets using Metaplex Core NFT Standard.

## Guide

Clone this repo and cd into it (please remember to star if find helpful)
```sh
git clone https://github.com/dProgrammingUniversity/metaplex-core-nft-asset-example.git
cd metaplex-core-nft-asset-example
```

Install dependencies
```sh
yarn install
```

Sync program keypair across Anchor.toml and lib.rs
```sh
anchor keys sync
```

Build the metaplex core nft asset minter program
```sh
anchor build
```

Deploy to Solana Devnet
```sh
anchor deploy
```

Test NFT minting
```sh
anchor run test
```
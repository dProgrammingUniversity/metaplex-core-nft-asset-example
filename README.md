# Metaplex Core NFT Asset Example
An example repo for how to mint Solana NFT assets using Metaplex Core NFT Standard.

## Guide

Clone this repo and cd into it (please remember to star if find helpful)
```sh
git clone https://github.com/dProgrammingUniversity/metaplex-core-nft-asset-example.git
cd metaplex-core-nft-asset-example
```

1. Install dependencies
```sh
yarn install
```

2. Sync program keypair across Anchor.toml and lib.rs
```sh
anchor keys sync
```

3. Build the metaplex core nft asset minter program
```sh
anchor build
```

4. Deploy to Solana Devnet
```sh
anchor deploy
```

5. Test NFT minting
```sh
anchor run test
```
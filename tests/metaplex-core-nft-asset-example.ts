import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CreateCoreAssetExample } from "../target/types/create_core_asset_example";
import { Keypair } from "@solana/web3.js";

describe("create-core-asset-example", () => {
  // 1. Proper provider setup
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  
  const program = anchor.workspace.CreateCoreAssetExample as Program<CreateCoreAssetExample>;
  const wallet = provider.wallet;
  const connection = provider.connection;

  it("Create Core NFT Asset", async () => {
    // 2. Generate asset keypair
    const asset = Keypair.generate();

    const createAssetArgs = {
      name: "My Core NFT",
      uri: "https://raw.githubusercontent.com/dProgrammingUniversity/metaplex-core-nft-asset-example/refs/heads/main/my-core-nft.json", // Replace with your actual URI
    };

    // 3. Send transaction
    const tx = await program.methods.createCoreAsset(createAssetArgs)
      .accounts({
        asset: asset.publicKey,
        collection: null,
        authority: null,
        payer: wallet.publicKey,
        owner: null,
        updateAuthority: null,
      })
      .signers([asset]) // 4. Only asset needs explicit signing
      .rpc();

    console.log(`Success! Transaction Signature: ${tx}`);
    
    // 5. Confirm transaction
    await connection.confirmTransaction(tx, "confirmed");
    console.log("Transaction confirmed");

    // 6. Verify asset creation
    const assetInfo = await connection.getAccountInfo(asset.publicKey);
    if (!assetInfo) {
      throw new Error("Asset account not created");
    }
    console.log(`Asset created at: ${asset.publicKey.toString()}`);
    console.log(`Owner: ${wallet.publicKey.toString()}`);
  });
});
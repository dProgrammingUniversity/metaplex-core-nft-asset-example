import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MetaplexCoreNftAssetExample } from "../target/types/metaplex_core_nft_asset_example";

describe("metaplex-core-nft-asset-example", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.metaplexCoreNftAssetExample as Program<MetaplexCoreNftAssetExample>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});

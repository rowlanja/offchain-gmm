import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { V3 } from "../target/types/v3";

describe("v3", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.V3 as Program<V3>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});

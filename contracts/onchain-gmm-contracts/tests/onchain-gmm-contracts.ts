import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from '@solana/web3.js';
import { OnchainGmmContracts } from "../target/types/onchain_gmm_contracts";
import { expect } from 'chai';

describe("onchain-gmm-contracts", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.OnchainGmmContracts as Program<OnchainGmmContracts>;

  it("Is initialized!", async () => {
    // Add your test here.

    const [depositPDA, _] = await PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode("deposit"),
          provider.wallet.publicKey.toBuffer()
        ],
        program.programId
      );

    const tx = await program.methods
      .initialize()
      .accounts({
        user: provider.wallet.publicKey,
        deposit: depositPDA,
      })
      .rpc();
    console.log("Your transaction signature", tx);

    await program.methods
    .deposit(100)
    .accounts({
      user: provider.wallet.publicKey,
      deposit: depositPDA,
    })
    .rpc();

    expect((await program.account.deposit.fetch(depositPDA)).amount).to.equal(100);
  });
});

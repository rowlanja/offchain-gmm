import * as anchor from "@coral-xyz/anchor";
import * as utils from "./utils";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID, Token } from "@solana/spl-token";
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

    expect((await program.account.deposit.fetch(depositPDA)).amount).to.equal(0);
  });

  it("deposits liquidity into pool!", async () => {
    // Add your test here.
    let poolKey = anchor.web3.Keypair.generate()
    let mintObject = await utils.createMint(poolKey, provider, provider.wallet.publicKey, null, 9, TOKEN_PROGRAM_ID);
    let mintPubkey = mintObject.publicKey;
    const amount = new anchor.BN(20000000);
    console.log("TESTING");

    await program.methods
    .depositLiquidity(amount)
    .accounts({
      systemProgram: poolKey.publicKey,
      applicationState: poolKey.publicKey,
      userWalletTokenA: poolKey.publicKey,
      poolWalletTokenB: poolKey.publicKey,
      userSending: poolKey.publicKey,
      mintOfTokenBeingSentA: poolKey.publicKey,
      mintOfTokenBeingSentB: poolKey.publicKey,
      tokenProgram: poolKey.publicKey,
    })
    .rpc();

  });
});

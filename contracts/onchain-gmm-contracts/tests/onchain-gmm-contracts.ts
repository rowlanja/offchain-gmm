import * as anchor from "@coral-xyz/anchor";
import * as utils from "./utils";
import * as spl from '@solana/spl-token';
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

  it("creates liquidity pool!", async () => {
    // Add your test here.
    const mintAddress = await createMint(provider.connection);
    const [alice, aliceWallet] = await createUserAndAssociatedWallet(provider.connection, mintAddress);
    let poolKey = anchor.web3.Keypair.generate()
    let mintObject = await utils.createMint(poolKey, provider, provider.wallet.publicKey, null, 9, TOKEN_PROGRAM_ID);
    const amount = new anchor.BN(20000000);

    const [, aliceBalancePre] = await readAccount(aliceWallet, provider);

    console.log("Balance : " + aliceBalancePre)

    await program.methods.createPool().accounts(
      {
        userWalletTokenA: aliceWallet,
        // poolWalletTokenB: poolKey.publicKey,
        // userSending: alice.publicKey,
        // mintOfTokenBeingSentA: mintObject.publicKey,
        // mintOfTokenBeingSentB: mintObject.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
      }
    )
    .rpc();
  });

  it("deposits liquidity into pool!", async () => {
    // Add your test here.
    const mintAddress = await createMint(provider.connection);
    const [alice, aliceWallet] = await createUserAndAssociatedWallet(provider.connection, mintAddress);
    let poolKey = anchor.web3.Keypair.generate()
    let mintObject = await utils.createMint(poolKey, provider, provider.wallet.publicKey, null, 9, TOKEN_PROGRAM_ID);
    const amount = new anchor.BN(20000000);

    const [, aliceBalancePre] = await readAccount(aliceWallet, provider);

    console.log("Balance : " + aliceBalancePre)

    await program.methods.depositLiquidity(amount).accounts(
      {
        userWalletTokenA: aliceWallet,
        // poolWalletTokenB: poolKey.publicKey,
        // userSending: alice.publicKey,
        // mintOfTokenBeingSentA: mintObject.publicKey,
        // mintOfTokenBeingSentB: mintObject.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
      }
    )
    .rpc();
  });

  const createUserAndAssociatedWallet = async (connection: anchor.web3.Connection, mint?: anchor.web3.PublicKey): Promise<[anchor.web3.Keypair, anchor.web3.PublicKey | undefined]> => {
    const user = new anchor.web3.Keypair();
    let userAssociatedTokenAccount: anchor.web3.PublicKey | undefined = undefined;

    // Fund user with some SOL
    let txFund = new anchor.web3.Transaction();
    txFund.add(anchor.web3.SystemProgram.transfer({
        fromPubkey: provider.wallet.publicKey,
        toPubkey: user.publicKey,
        lamports: 5 * anchor.web3.LAMPORTS_PER_SOL,
    }));
    const sigTxFund = await provider.sendAndConfirm(txFund);
    console.log(`[${user.publicKey.toBase58()}] Funded new account with 5 SOL: ${sigTxFund}`);

    if (mint) {
        // Create a token account for the user and mint some tokens
        userAssociatedTokenAccount = await spl.Token.getAssociatedTokenAddress(
            spl.ASSOCIATED_TOKEN_PROGRAM_ID,
            spl.TOKEN_PROGRAM_ID,
            mint,
            user.publicKey
        )

        const txFundTokenAccount = new anchor.web3.Transaction();
        txFundTokenAccount.add(spl.Token.createAssociatedTokenAccountInstruction(
            spl.ASSOCIATED_TOKEN_PROGRAM_ID,
            spl.TOKEN_PROGRAM_ID,
            mint,
            userAssociatedTokenAccount,
            user.publicKey,
            user.publicKey,
        ))
        txFundTokenAccount.add(spl.Token.createMintToInstruction(
            spl.TOKEN_PROGRAM_ID,
            mint,
            userAssociatedTokenAccount,
            provider.wallet.publicKey,
            [],
            1337000000,
        ));
        const txFundTokenSig = await provider.sendAndConfirm(txFundTokenAccount, [user]);
        console.log(`[${userAssociatedTokenAccount.toBase58()}] New associated account for mint ${mint.toBase58()}: ${txFundTokenSig}`);
    }
    return [user, userAssociatedTokenAccount];
  }

  const createMint = async (connection: anchor.web3.Connection): Promise<anchor.web3.PublicKey> => {
    const tokenMint = new anchor.web3.Keypair();
    const lamportsForMint = await provider.connection.getMinimumBalanceForRentExemption(spl.MintLayout.span);
    let tx = new anchor.web3.Transaction();

    // Allocate mint
    tx.add(
        anchor.web3.SystemProgram.createAccount({
            programId: spl.TOKEN_PROGRAM_ID,
            space: spl.MintLayout.span,
            fromPubkey: provider.wallet.publicKey,
            newAccountPubkey: tokenMint.publicKey,
            lamports: lamportsForMint,
        })
    )
    // Allocate wallet account
    tx.add(
        spl.Token.createInitMintInstruction(
            spl.TOKEN_PROGRAM_ID,
            tokenMint.publicKey,
            6,
            provider.wallet.publicKey,
            provider.wallet.publicKey,
        )
    );
    const signature = await provider.sendAndConfirm(tx, [tokenMint]);

    console.log(`[${tokenMint.publicKey}] Created new mint account at ${signature}`);
    return tokenMint.publicKey;
  }

  const readAccount = async (accountPublicKey: anchor.web3.PublicKey, provider: anchor.Provider): Promise<[spl.AccountInfo, string]> => {
    const tokenInfoLol = await provider.connection.getAccountInfo(accountPublicKey);
    const data = Buffer.from(tokenInfoLol.data);
    const accountInfo: spl.AccountInfo = spl.AccountLayout.decode(data);

    const amount = (accountInfo.amount as any as Buffer).readBigUInt64LE();
    return [accountInfo, amount.toString()];
  }

});

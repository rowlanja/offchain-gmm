import * as anchor from "@coral-xyz/anchor";
import * as utils from "./utils";
import * as spl from '@solana/spl-token';
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID, Token } from "@solana/spl-token";
import { OnchainGmmContracts } from "../target/types/onchain_gmm_contracts";
import { expect } from 'chai';

interface PDAParameters {
  escrowWalletTokenAKey: anchor.web3.PublicKey,
  escrowWalletTokenBKey: anchor.web3.PublicKey,
  stateKey: anchor.web3.PublicKey,
  escrowBumpTokenA: number,
  escrowBumpTokenB: number,
  stateBump: number,
  idx: anchor.BN,
}


describe("onchain-gmm-contracts", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.OnchainGmmContracts as Program<OnchainGmmContracts>;
  let mintAddress: anchor.web3.PublicKey;
  let alice: anchor.web3.Keypair;
  let aliceWallet: anchor.web3.PublicKey;
  let bob: anchor.web3.Keypair;

  let pda: PDAParameters;

  beforeEach(async () => {
    mintAddress = await createMint(provider.connection);
    [alice, aliceWallet] = await createUserAndAssociatedWallet(provider.connection, mintAddress);

    let _rest;
    [bob, ..._rest] = await createUserAndAssociatedWallet(provider.connection);

    pda = await getPdaParams(provider.connection, alice.publicKey, bob.publicKey, mintAddress);
});


  // it("creates liquidity pool!", async () => {

  //   const [poolStatePDA, poolBump] = await PublicKey.findProgramAddress(
  //     [
  //       anchor.utils.bytes.utf8.encode('user-stats'),
  //       alice.publicKey.toBuffer(),
  //     ],
  //     program.programId
  //   )

  //   const [poolWalletTokenAPDA, walletTokenABump] = await PublicKey.findProgramAddress(
  //     [
  //       anchor.utils.bytes.utf8.encode('pool_wallet_token_a'),
  //       mintAddress.toBuffer()
  //     ],
  //     program.programId
  //   )

  //   const [poolWalletTokenBPDA, walletTokenBBump] = await PublicKey.findProgramAddress(
  //     [
  //       anchor.utils.bytes.utf8.encode('pool_wallet_token_b'),
  //       mintAddress.toBuffer()
  //     ],
  //     program.programId
  //   )

  //   const [userStakePDA, userStakeBump] = await PublicKey.findProgramAddress(
  //     [
  //       anchor.utils.bytes.utf8.encode('stake'),
  //       alice.publicKey.toBuffer(),
  //       mintAddress.toBuffer()
  //     ],
  //     program.programId
  //   )

  //   // let poolKey = anchor.web3.Keypair.generate()
  //   // const amount = new anchor.BN(20000000);

  //   let [, aliceBalancePreTokenA] = await readAccount(aliceWallet, provider);
  //   console.log("[PRE] Creator Balance Token A : " + aliceBalancePreTokenA)

  //   // let [, aliceBalancePretokenB] = await readAccount(aliceWalletTokenB, provider);
  //   let tokenADepositAmount = new anchor.BN(100);
  //   let tokenBDepositAmount = new anchor.BN(200);

  //   await program.methods
  //   .createPool(1.1, 1.0, 0.5, tokenADepositAmount, tokenBDepositAmount)
  //   .accounts({
  //     user: alice.publicKey,
  //     poolState: poolStatePDA,
  //     poolWalletTokenA: poolWalletTokenAPDA,
  //     poolWalletTokenB: poolWalletTokenBPDA,
  //     stakeRecord: userStakePDA,
  //     userWalletTokenA: aliceWallet,
  //     userWalletTokenB: aliceWallet,
  //     mintOfTokenBeingSentA: mintAddress,
  //     mintOfTokenBeingSentB: mintAddress,
  //     tokenProgram: spl.TOKEN_PROGRAM_ID
  //   })
  //   .signers([alice])
  //   .rpc();

  //   [, aliceBalancePreTokenA] = await readAccount(aliceWallet, provider);
  //   console.log("[POST] Creator Balance Token A : " + aliceBalancePreTokenA);

  //   let [, poolBalancePreTokenA] = await readAccount(poolWalletTokenAPDA, provider);
  //   console.log("[POST] Pool Balance Token A : " + poolBalancePreTokenA);

  //   [, poolBalancePreTokenA] = await readAccount(poolWalletTokenBPDA, provider);
  //   console.log("[POST] Pool Balance Token A : " + poolBalancePreTokenA);

  //   const state = await program.account.deposit.fetch(userStakePDA);
  //   console.log("amount : " + state.amount.toString());
  //   console.log("timestamp : " + state.timestamp.toString());

  //   console.log("TIME TO SWAP ");

  //   let tokenAPurchaseAmount = new anchor.BN(1);

  //   await program.methods
  //   .swap(tokenAPurchaseAmount, mintAddress)
  //   .accounts({
  //     user: alice.publicKey,
  //     pool: poolStatePDA,
  //     poolWalletTokenA: poolWalletTokenAPDA,
  //     poolWalletTokenB: poolWalletTokenBPDA,
  //     userWalletTokenA: aliceWallet,
  //     userWalletTokenB: aliceWallet,
  //     mintOfTokenBeingSentA: mintAddress,
  //     mintOfTokenBeingSentB: mintAddress,
  //     tokenProgram: spl.TOKEN_PROGRAM_ID
  //   })
  //   .signers([alice])
  //   .rpc();
  // });

  it("creates position in concentrated pool!", async () => {
    // Add your test here.
    const mintAddress = await createMint(provider.connection);
    const [alice, aliceWallet] = await createUserAndAssociatedWallet(provider.connection, mintAddress);
    const upper_price = 5500.0;
    const lower_price = 4545.0;
    const current_price = 5000.0;

    const lower_tick_id = new anchor.BN(10);
    const upper_tick_id = new anchor.BN(20);
    const amount = new anchor.BN(1);

    const [lowerTick, lowerTickBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode('clamm'),
        mintAddress.toBuffer(),
        mintAddress.toBuffer(),
        lower_tick_id.toArrayLike(Buffer,"le",8)
      ],
      program.programId
    )

    const [upperTick, upperTickBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode('clamm'),
        mintAddress.toBuffer(),
        mintAddress.toBuffer(),
        upper_tick_id.toArrayLike(Buffer,"le",8)
      ],
      program.programId
    )

    const [position, positionBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode('clamm_position'),
        mintAddress.toBuffer(),
        mintAddress.toBuffer(),
        lower_tick_id.toArrayLike(Buffer,"le",8),
        upper_tick_id.toArrayLike(Buffer,"le",8)
      ],
      program.programId
    )

    const [poolStatePDA, poolBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode('user-stats'),
        alice.publicKey.toBuffer(),
      ],
      program.programId
    )

    const [poolWalletTokenAPDA, walletTokenABump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode('pool_wallet_token_a'),
        mintAddress.toBuffer()
      ],
      program.programId
    )

    const [poolWalletTokenBPDA, walletTokenBBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode('pool_wallet_token_b'),
        mintAddress.toBuffer()
      ],
      program.programId
    )

    await program.methods.createPositionConcentratedPool(
      lower_tick_id,
      upper_tick_id,
      lower_price,
      upper_price,
      current_price,
      amount
      ).accounts(
      {
        user: alice.publicKey,
        lowerTick: lowerTick,
        upperTick: upperTick,
        position: position,
        poolWalletTokenA: poolWalletTokenAPDA,
        poolWalletTokenB: poolWalletTokenAPDA,
        token0: mintAddress,
        token1: mintAddress,
        userWalletTokenA: aliceWallet,
        userWalletTokenB: aliceWallet,
        // poolWalletTokenB: poolKey.publicKey,
        // userSending: alice.publicKey,
        // mintOfTokenBeingSentA: mintObject.publicKey,
        // mintOfTokenBeingSentB: mintObject.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
      }
    )
    .signers([alice])
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

  const createAssociatedWallet = async (connection: anchor.web3.Connection, user: anchor.web3.PublicKey, mint?: anchor.web3.PublicKey): Promise<anchor.web3.PublicKey> => {
    let userAssociatedTokenAccount: anchor.web3.PublicKey | undefined = undefined;

    // Fund user with some SOL
    let txFund = new anchor.web3.Transaction();
    txFund.add(anchor.web3.SystemProgram.transfer({
        fromPubkey: provider.wallet.publicKey,
        toPubkey: user,
        lamports: 5 * anchor.web3.LAMPORTS_PER_SOL,
    }));
    const sigTxFund = await provider.sendAndConfirm(txFund);
    console.log(`[${user.toBase58()}] Funded new account with 5 SOL: ${sigTxFund}`);

    if (mint) {
        // Create a token account for the user and mint some tokens
        userAssociatedTokenAccount = await spl.Token.getAssociatedTokenAddress(
            spl.ASSOCIATED_TOKEN_PROGRAM_ID,
            spl.TOKEN_PROGRAM_ID,
            mint,
            user
        )

        const txFundTokenAccount = new anchor.web3.Transaction();
        txFundTokenAccount.add(spl.Token.createAssociatedTokenAccountInstruction(
            spl.ASSOCIATED_TOKEN_PROGRAM_ID,
            spl.TOKEN_PROGRAM_ID,
            mint,
            userAssociatedTokenAccount,
            user,
            user,
        ))
        txFundTokenAccount.add(spl.Token.createMintToInstruction(
            spl.TOKEN_PROGRAM_ID,
            mint,
            userAssociatedTokenAccount,
            provider.wallet.publicKey,
            [],
            1337000000,
        ));
        const txFundTokenSig = await provider.sendAndConfirm(txFundTokenAccount);
        console.log(`[${userAssociatedTokenAccount.toBase58()}] New associated account for mint ${mint.toBase58()}: ${txFundTokenSig}`);
    }
    return userAssociatedTokenAccount;
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

  const getPdaParams = async (connection: anchor.web3.Connection, alice: anchor.web3.PublicKey, bob: anchor.web3.PublicKey, mint: anchor.web3.PublicKey): Promise<PDAParameters> => {
    const uid = new anchor.BN(parseInt((Date.now() / 1000).toString()));
    const uidBuffer = uid.toBuffer('le', 8);

    let [statePubKey, stateBump] = await anchor.web3.PublicKey.findProgramAddress(
        [anchor.utils.bytes.utf8.encode("state"), alice.toBuffer(), mint.toBuffer()], program.programId,
    );
    let [walletPubKeyTokenA, walletBumpTokenA] = await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("wallet"), alice.toBuffer(), bob.toBuffer(), mint.toBuffer(), uidBuffer], program.programId,
    );
    let [walletPubKeyTokenB, walletBumpTokenB] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("wallet"), alice.toBuffer(), bob.toBuffer(), mint.toBuffer(), uidBuffer], program.programId,
  );
    return {
        idx: uid,
        escrowBumpTokenA: walletBumpTokenA,
        escrowBumpTokenB: walletBumpTokenB,
        escrowWalletTokenAKey: walletPubKeyTokenA,
        escrowWalletTokenBKey: walletPubKeyTokenB,
        stateBump,
        stateKey: statePubKey,
    }
}

});

export async function getAssociatedTokenAddress(
  mint: PublicKey,
  owner: PublicKey,
  programId = spl.TOKEN_PROGRAM_ID,
  associatedTokenProgramId = spl.ASSOCIATED_TOKEN_PROGRAM_ID
): Promise<PublicKey> {

  const [address] = await PublicKey.findProgramAddress(
      [owner.toBuffer(), programId.toBuffer(), mint.toBuffer()],
      associatedTokenProgramId
  );

  return address;
}
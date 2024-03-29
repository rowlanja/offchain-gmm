import * as anchor from "@coral-xyz/anchor";
import * as utils from "./utils";
import * as spl from '@solana/spl-token';
import { Program } from "@coral-xyz/anchor";
import { PublicKey, LAMPORTS_PER_SOL } from '@solana/web3.js';
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
  let solTokenAddress: anchor.web3.Keypair;
  let solRewardsPoolAddress: anchor.web3.Keypair;

  beforeEach(async () => {
    mintAddress = await createMint(provider.connection);
    [alice, aliceWallet] = await createUserAndAssociatedWallet(provider.connection, mintAddress);

    let _rest;
    [bob, ..._rest] = await createUserAndAssociatedWallet(provider.connection);
  });


  it("creates liquidity pool!", async () => {

    const [poolStatePDA, poolBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode('pool-state'),
        mintAddress.toBuffer(),
        mintAddress.toBuffer(),
      ],
      program.programId
    )

    const [solToken0StatePDA, solToken0StateBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode('sol'),
        mintAddress.toBuffer(),
      ],
      program.programId
    )

    const [solToken1StatePDA, solToken1StateBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode('sol'),
        mintAddress.toBuffer(),
      ],
      program.programId
    )

    const [poolWalletTokenAPDA, walletTokenABump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode('pool_wallet_token_0'),
        mintAddress.toBuffer()
      ],
      program.programId
    )

    const [poolWalletTokenBPDA, walletTokenBBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode('pool_wallet_token_1'),
        mintAddress.toBuffer()
      ],
      program.programId
    )

    const [poolWalletTokenBPDASol, walletSolTokenBBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode('pool_token_wallet'),
        mintAddress.toBuffer()
      ],
      program.programId
    )

    const [poolWalletTokenAPDASol, walletSolTokenABump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode('pool_token_wallet'),
        mintAddress.toBuffer()
      ],
      program.programId
    )

    const [userStakePDA, userStakeBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode('position'),
        alice.publicKey.toBuffer(),
        poolStatePDA.toBuffer()
      ],
      program.programId
    )

    const [userSolToken0StakePDA, userSolToken0StakeBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode('position'),
        alice.publicKey.toBuffer(),
        poolStatePDA.toBuffer()
      ],
      program.programId
    )

    const [userSolToken1StakePDA, userSolToken1StakeBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode('position'),
        alice.publicKey.toBuffer(),
        solToken1StatePDA.toBuffer()
      ],
      program.programId
    )

    const [stakeListPDA, stakeListBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode('stakers'),
        poolStatePDA.toBuffer()
      ],
      program.programId
    )

    let [, aliceBalancePreTokenA] = await readAccount(aliceWallet, provider);
    console.log("[Pre liquidity Provision] User Balance Token A : " + aliceBalancePreTokenA);

    let tokenADepositAmount = new anchor.BN(100);
    let tokenBDepositAmount = new anchor.BN(200);
    let tokenASwapAmount = new anchor.BN(5);

    await program.methods
    .createPool(tokenADepositAmount, tokenBDepositAmount, alice.publicKey)
    .accounts({
      user: alice.publicKey,
      poolState: poolStatePDA,
      poolWalletToken0: poolWalletTokenAPDA,
      poolWalletToken1: poolWalletTokenBPDA,
      position: userStakePDA,
      stakersList: stakeListPDA,
      userWalletToken0: aliceWallet,
      userWalletToken1: aliceWallet,
      token0Mint: mintAddress,
      token1Mint: mintAddress,
      tokenProgram: spl.TOKEN_PROGRAM_ID
    })
    .signers([alice])
    .rpc();

    [, aliceBalancePreTokenA] = await readAccount(aliceWallet, provider);
    console.log("[Post liquidity Provision] User Balance Token A : " + aliceBalancePreTokenA);

    let [, poolBalancePreTokenA] = await readAccount(poolWalletTokenAPDA, provider);
    console.log("[Post liquidity Provision] Pool Balance Token A : " + poolBalancePreTokenA);

    // SET UP TOKEN0_SOL pool
    console.log("[PRE] setting up SOL token0 Pool");
    await program.methods
    .createSolPool(new anchor.BN(100000), new anchor.BN(100))
    .accounts({
      user: alice.publicKey,
      poolState: solToken0StatePDA,
      poolTokenWallet: poolWalletTokenAPDASol,
      position: userSolToken0StakePDA,
      wallet: alice.publicKey,
      userWalletToken: aliceWallet,
      tokenMint: mintAddress,
      tokenProgram: spl.TOKEN_PROGRAM_ID
    })
    .signers([alice])
    .rpc();

    // /// flakey cant
    // console.log("[PRE] setting up SOL token1 Pool");
    // await program.methods
    // .createSolPool(new anchor.BN(100000), new anchor.BN(200))
    // .accounts({
    //   user: alice.publicKey,
    //   poolState: solToken1StatePDA,
    //   poolTokenWallet: poolWalletTokenBPDASol,
    //   position: userSolToken1StakePDA,
    //   wallet: alice.publicKey,
    //   userWalletToken: aliceWallet,
    //   tokenMint: mintAddress,
    //   tokenProgram: spl.TOKEN_PROGRAM_ID
    // })
    // .signers([alice])
    // .rpc();
    // let [, poolBalanceSolPoolTokenA] = await readAccount(poolWalletTokenAPDASol, provider);
    // console.log("[POST] Pool Balance Token A / SOL : " + poolBalanceSolPoolTokenA);

    // // SET UP TOKEN1_SOL pool
    // await program.methods
    // .createSolPool(new anchor.BN(1), new anchor.BN(100))
    // .accounts({
    //   user: alice.publicKey,
    //   poolState: solToken0StatePDA,
    //   poolTokenWallet: poolWalletTokenBPDASol,
    //   position: userSolToken0StakePDA,
    //   wallet: alice.publicKey,
    //   userWalletToken: aliceWallet,
    //   tokenMint: mintAddress,
    //   tokenProgram: spl.TOKEN_PROGRAM_ID
    // })
    // .signers([alice])
    // .rpc();

    // [, aliceBalancePreTokenA] = await readAccount(aliceWallet, provider);
    // console.log("[POST] Creator Balance Token A : " + aliceBalancePreTokenA);

    // let [, poolBalancePreTokenA] = await readAccount(poolWalletTokenAPDA, provider);
    // console.log("[POST] Pool Balance Token A : " + poolBalancePreTokenA);

    // [, poolBalancePreTokenA] = await readAccount(poolWalletTokenBPDA, provider);
    // console.log("[POST] Pool Balance Token B : " + poolBalancePreTokenA);

    // let state = await program.account.position.fetch(userStakePDA);
    // console.log("amount : " + state.amount.toString());
    // console.log("timestamp : " + state.timestamp.toString());


    // let poolPDA = await program.account.pool.fetch(poolStatePDA);
    // console.log("Pool PDA token 0 balance : " + poolPDA.totalStakedToken0.toString());
    // console.log("Pool PDA token 1 balance : " + poolPDA.totalStakedToken1.toString());

    let [, aliceBalanceTokenA] = await readAccount(aliceWallet, provider);
    console.log("[PRE SWAP] Swapper Balance Token A : " + aliceBalanceTokenA);

    let [, poolBalanceTokenA] = await readAccount(poolWalletTokenAPDA, provider);
    console.log("[PRE SWAP] Pool Balance Token A : " + poolBalanceTokenA);

    console.log("TIME TO SWAP ");
    await program.methods
    .swap(tokenASwapAmount, false)
    .accounts({
      user: alice.publicKey,
      pool: poolStatePDA,
      // rewardPool0For2: solToken0StatePDA,
      // rewardPool1For2: solToken1StatePDA,
      poolWalletToken0: poolWalletTokenAPDA,
      poolWalletToken1: poolWalletTokenBPDA,
      stakersList: stakeListPDA,
      userWalletToken0: aliceWallet,
      userWalletToken1: aliceWallet,
      token0Mint: mintAddress,
      token1Mint: mintAddress,
      tokenProgram: spl.TOKEN_PROGRAM_ID
    })
    .signers([alice])
    .rpc();

    [, aliceBalanceTokenA] = await readAccount(aliceWallet, provider);
    console.log("[POST SWAP] Swapper Balance Token A : " + aliceBalanceTokenA);

    [, poolBalanceTokenA] = await readAccount(poolWalletTokenAPDA, provider);
    console.log("[POST SWAP] Pool Balance Token A : " + poolBalanceTokenA);

    // poolPDA = await program.account.pool.fetch(poolStatePDA);
    // console.log("Pool PDA token 0 balance : " + poolPDA.totalStakedToken0.toString());
    // console.log("Pool PDA token 1 balance : " + poolPDA.totalStakedToken1.toString());

    // state = await program.account.position.fetch(userStakePDA);
    // console.log("amount : " + state.amount.toString());
    // console.log("timestamp : " + state.timestamp.toString());

  });

  const createUserAndAssociatedWallet = async (connection: anchor.web3.Connection, mint?: anchor.web3.PublicKey): Promise<[anchor.web3.Keypair, anchor.web3.PublicKey | undefined]> => {
    const user = new anchor.web3.Keypair();
    let userAssociatedTokenAccount: anchor.web3.PublicKey | undefined = undefined;

    // Fund user with some SOL
    let txFund = new anchor.web3.Transaction();
    txFund.add(anchor.web3.SystemProgram.transfer({
        fromPubkey: provider.wallet.publicKey,
        toPubkey: user.publicKey,
        lamports: 500 * anchor.web3.LAMPORTS_PER_SOL,
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
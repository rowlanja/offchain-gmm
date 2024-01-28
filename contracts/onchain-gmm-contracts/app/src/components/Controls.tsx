import { ConnectionProvider, WalletProvider } from '@solana/wallet-adapter-react';
import { WalletModalProvider, WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { PhantomWalletAdapter,
    SolflareWalletAdapter, } from '@solana/wallet-adapter-wallets';
import { FC, ReactNode } from 'react';
import styled from "styled-components";
import * as web3 from '@solana/web3.js';
import * as spl from '@solana/spl-token';
import * as anchor from "@coral-xyz/anchor";
import { AnchorProvider as Provider, Program } from '@project-serum/anchor';
import {useState} from 'react';
import { Box, Button, FormControl, FormLabel, Input, NumberDecrementStepper, NumberIncrementStepper, NumberInput, NumberInputField, NumberInputStepper, Textarea } from '@chakra-ui/react'
import { Buffer } from "buffer";
import idl from "../idl.json"
import { OnchainGmmContracts } from "../idl"
import { useAnchorWallet, useConnection } from '@solana/wallet-adapter-react';

require('@solana/wallet-adapter-react-ui/styles.css');
const DEFAULT_COMMITMENT: web3.Commitment = "confirmed";
export const USAGE_PROGRAM_ID = new web3.PublicKey(
  "FcXK2AuHYzE5fykqQj65955bvh5N2LojFkuoC8fz5nVw"
);
const theme = {
  blue: {
    default: "#3f51b5",
    hover: "#283593",
  },
  pink: {
    default: "#e91e63",
    hover: "#ad1457",
  },
};

const Botton = styled.button`
background-color: ${(props) => theme[props.theme].default};
color: white;
padding: 5px 15px;
border-radius: 5px;
outline: 0;
border: 0; 
text-transform: uppercase;
margin: 10px 0px;
cursor: pointer;
box-shadow: 0px 2px 2px lightgray;
transition: ease background-color 250ms;
&:hover {
  background-color: ${(props) => theme[props.theme].hover};
}
&:disabled {
  cursor: default;
  opacity: 0.7;
}
`;

Botton.defaultProps = {
  theme: "blue",
};

export const Controls: FC = () => {

  const clickMe = async () => {
    const connection = new web3.Connection('http://localhost:8899');
    const myKeypair = web3.Keypair.generate();
    setWallet(myKeypair)
    console.log("Key PK : " + myKeypair.publicKey);
    // SEND SOL 
    const fromAirDropSignature = await connection.requestAirdrop(new web3.PublicKey(myKeypair.publicKey), 20 * web3.LAMPORTS_PER_SOL);
    await connection.confirmTransaction(fromAirDropSignature);
    // CREATE MINT 
  
    //get the token accont of this solana address, if it does not exist, create it
    let mint0 = await spl.Token.createMint(
        connection,
        myKeypair,
        myKeypair.publicKey,
        null,
        6, // Number of decimal places in your token
        spl.TOKEN_PROGRAM_ID,
    );
    
    let fromTokenAccount0 = await mint0.getOrCreateAssociatedAccountInfo(
        myKeypair.publicKey,
    );
  
    await mint0.mintTo(
        fromTokenAccount0.address,
        myKeypair.publicKey,
        [],
        200000000
    );
    
    console.log("Token0 PK : " + mint0.publicKey);
    console.log("Token0 Addr : " + fromTokenAccount0.address);
    console.log("Token0 Balance : " + (await mint0.getAccountInfo(fromTokenAccount0.address)).amount);
  
    //get the token accont of this solana address, if it does not exist, create it
    let mint1 = await spl.Token.createMint(
      connection,
      myKeypair,
      myKeypair.publicKey,
      null,
      6, // Number of decimal places in your token
      spl.TOKEN_PROGRAM_ID,
    );
  
    let fromTokenAccount1 = await mint1.getOrCreateAssociatedAccountInfo(
        myKeypair.publicKey,
    );
  
    await mint1.mintTo(
        fromTokenAccount1.address,
        myKeypair.publicKey,
        [],
        100000000
    );
  
    console.log("Token1 PK : " + mint1.publicKey);
    console.log("Token1 Addr : " + fromTokenAccount1.address);
    console.log("Token1 Balance : " + (await mint1.getAccountInfo(fromTokenAccount1.address)).amount);
  }

  const handleSubmit = (event: any) => {
    event.preventDefault()
    handleTransactionSubmit(event)
  }
  
  const handleTransactionSubmit = async (event: any) => {
    console.log('token0Addr : ', token0Addr);
    console.log('token1Addr : ', token1Addr);

    const poolStatePDA = (await web3.PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode('pool-state'),
        new web3.PublicKey(token0Addr).toBuffer(),
        new web3.PublicKey(token1Addr).toBuffer()
      ],
      spl.TOKEN_PROGRAM_ID
    ))[0];

    const poolWalletToken0PDA = (await web3.PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode('pool_wallet_token_0'),
        new web3.PublicKey(token0Addr).toBuffer()
      ],
      spl.TOKEN_PROGRAM_ID
    ))[0];

    const poolWalletToken1PDA = (await web3.PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode('pool_wallet_token_1'),
        new web3.PublicKey(token1Addr).toBuffer()
      ],
      spl.TOKEN_PROGRAM_ID
    ))[0];

    const positionPDA = (await web3.PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode('position'),
        new web3.PublicKey(token1Addr).toBuffer(),
        poolStatePDA.toBuffer()
      ],
      spl.TOKEN_PROGRAM_ID
    ))[0];

    const stakersPDA = (await web3.PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode('stakers'),
        poolStatePDA.toBuffer()
      ],
      spl.TOKEN_PROGRAM_ID
    ))[0];
    console.log('PoolStatePDA : ', poolStatePDA);
    let tokenADepositAmount = new anchor.BN(100);
    let tokenBDepositAmount = new anchor.BN(200);
    let tokenASwapAmount = new anchor.BN(5);
    console.log('env : ', process.env);
    const wallet = useAnchorWallet();
    const { connection } = useConnection();
    if (!wallet) return null;
    const provider = new Provider(connection, wallet, opts);
    const shell =  new Program<OnchainGmmContracts>(idl, "FcXK2AuHYzE5fykqQj65955bvh5N2LojFkuoC8fz5nVw", provider);
    console.log('creating pool')
    await shell.methods
    .createPool(tokenADepositAmount, tokenBDepositAmount, wallet.publicKey, false)
    .accounts({
      user: wallet.publicKey,
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
    .signers([wallet])
    .rpc();
    console.log('created pool')
    // if (!idl) throw new Error("Usage IDL could not be found");

    // const program = new sanchor.Program(idl, 'FcXK2AuHYzE5fykqQj65955bvh5N2LojFkuoC8fz5nVw', provider);
    // console.log('Program Addr : ' + program)
  }
  
  const  [token0Addr, setToken0Addr] =  useState('');
  const  [token1Addr, setToken1Addr] =  useState('');
  const  [wallet, setWallet] =  useState<web3.Keypair>();

	const  handleChangeToken0Mint = (event) => {
		setToken0Addr(event.target.value);
	};

	const  handleChangeToken1Mint = (event) => {
		setToken1Addr(event.target.value);
	};

  return (
      <div>
         <Botton onClick={clickMe}>Create Token + mint to Address</Botton>
         <div>
          <Box
              p={4}
              display={{ md: "flex" }}
              maxWidth="32rem"
              borderWidth={1}
              margin={2}
              justifyContent="center"
          >
              <form onSubmit={handleSubmit}>
                  <FormControl isRequired>
                      <FormLabel color='gray.200'>
                        token0Addr
                      </FormLabel>
                      <Input
                          id='title'
                          color='gray.400'
                          onChange={event => setToken0Addr(event.currentTarget.value)}
                      />
                  </FormControl>
                  <FormControl isRequired>
                      <FormLabel color='gray.200'>
                        token1Addr
                      </FormLabel>
                      <Input
                          id='title'
                          color='gray.400'
                          onChange={event => setToken1Addr(event.currentTarget.value)}
                      />
                  </FormControl>
                  <Button width="full" mt={4} type="submit">
                      Submit Review
                  </Button>
              </form>
          </Box>
        </div>
      </div>
  );
};


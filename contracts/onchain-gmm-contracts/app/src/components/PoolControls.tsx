import { ConnectionProvider, WalletProvider } from '@solana/wallet-adapter-react';
import { WalletModalProvider, WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { PhantomWalletAdapter,
    SolflareWalletAdapter, } from '@solana/wallet-adapter-wallets';
import { FC, ReactNode } from 'react';
import styled from "styled-components";
import * as web3 from '@solana/web3.js';
import * as spl from '@solana/spl-token';
import * as anchor from "@coral-xyz/anchor";
import {useState} from 'react';
import { Box, Button, FormControl, FormLabel, Input, NumberDecrementStepper, NumberIncrementStepper, NumberInput, NumberInputField, NumberInputStepper, Textarea } from '@chakra-ui/react'
import { Buffer } from "buffer";
import idl from "../idl.json"

require('@solana/wallet-adapter-react-ui/styles.css');


export const PoolControls: FC = () => {
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

    const connection = new web3.Connection('http://localhost:8899');
  }

  
  const  [token0Addr, setToken0Addr] =  useState('');
  const  [token1Addr, setToken1Addr] =  useState('');

	const  handleChangeToken0Mint = (event) => {
		setToken0Addr(event.target.value);
	};

	const  handleChangeToken1Mint = (event) => {
		setToken1Addr(event.target.value);
	};

  return (
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
  );
};


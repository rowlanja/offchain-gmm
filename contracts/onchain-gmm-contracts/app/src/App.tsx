import { ConnectionProvider, WalletProvider } from '@solana/wallet-adapter-react';
import { WalletModalProvider, WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { PhantomWalletAdapter,
    SolflareWalletAdapter, } from '@solana/wallet-adapter-wallets';
import { FC, ReactNode } from 'react';
import styled from "styled-components";
import * as web3 from '@solana/web3.js';
import * as spl from '@solana/spl-token';
import { Controls } from '../src/components/Controls'
import { PoolControls } from '../src/components/PoolControls'

require('./App.css');
require('@solana/wallet-adapter-react-ui/styles.css');


const App: FC = () => {
    return (
        <Context>
            <Content />
        </Context>
    );
};
export default App;

const Context: FC<{ children: ReactNode }> = ({ children }) => {

    const SOLANA_WALLETS = [
        new PhantomWalletAdapter(),
        new SolflareWalletAdapter(),
      ];

    return (
        <ConnectionProvider endpoint={"http://127.0.0.1:8899"}>
            <WalletProvider wallets={SOLANA_WALLETS} autoConnect>
                <WalletModalProvider>{children}</WalletModalProvider>
            </WalletProvider>
        </ConnectionProvider>
    );
};

const Content: FC = () => {
    return (
        <div>
            <div className="App">
                <WalletMultiButton />
                <Controls/>
            </div>
        </div>
    );
};

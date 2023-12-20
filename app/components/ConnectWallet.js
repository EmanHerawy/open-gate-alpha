import { useState } from 'react'

import {
  SubstrateChain,
  SubstrateWalletPlatform,
  allSubstrateWallets,
  getSubstrateChain,
  isWalletInstalled,
  useBalance,
  useInkathon,
} from '@scio-labs/use-inkathon'
export default function ConnectButton() {
  const {
    activeChain,
    switchActiveChain,
    connect,
    disconnect,
    isConnecting,
    activeAccount,
    accounts,
    setActiveAccount,
  } = useInkathon()
  const { reducibleBalance, reducibleBalanceFormatted } = useBalance(activeAccount?.address, true, {
    forceUnit: false,
    fixedDecimals: 2,
    removeTrailingZeros: true,
  })
  

 
const connectWallet = async () => {
  try {
    console.log({ isConnecting });
      await  connect();
    } catch (error) {
      console.error(error);
    }
  }
  return (
    <>
      <div className="App">
        {
          activeAccount ? (
            <>
              <h3>Connected</h3>
            
            </>
          ) : (
              <button  onClick={connectWallet}>Connect</button>
          )
        }
      </div>
    </>
  );
}

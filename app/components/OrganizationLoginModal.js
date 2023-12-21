import React, { useEffect } from 'react'
import Button from '@mui/material/Button'

import { useSession } from 'next-auth/react'


import { useCall, useContract, useWallet, useAllWallets } from 'useink'

const metadata = require("../../smart_contracts/deployments/registration/metadata.json");


const CONTRACT_ID =
  "5G2oMVjPfb66uT35gcqFZkecGHEMJf25oarhctUNGMiFj4K3";


export default function LoginModal() {
  const { data: session } = useSession()
   const contract = useContract(CONTRACT_ID, metadata);


  const { account, connect, disconnect } = useWallet()
  const wallets = useAllWallets();
  useEffect(() => {
    if (session && account) {
      handleSmartContractRegister()
    }
  }, [session, account])


  const handleSmartContractRegister = async (type) => {

 
    // Creates a transactions to call the increment function
    // because it creates a TX and updates the contract state this requires the wallet to have enough coins to cover the costs and also to sign the Transaction
    try {
      try {

        const func = useCall(contract, 'join_as_open_source_project_creator');
        await func.send(activeSigner, { signer: activeSigner });
        onClose()
      } catch (error) {
        console.error(error)
      }
      onClose()
    } catch (error) {
      console.error(error)
    }
  }

 
  return (
    <Button onClick={handleSmartContractRegister}>Login As Organization</Button>
  )
}

const style = {
  position: 'absolute',
  top: '50%',
  left: '50%',
  transform: 'translate(-50%, -50%)',
  width: 400,
  bgcolor: 'background.paper',
  border: '2px solid #000',
  boxShadow: 24,
  p: 4,
}

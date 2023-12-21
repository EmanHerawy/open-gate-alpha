import { useEffect, useState } from 'react'
import { useSession } from 'next-auth/react'

import Box from '@mui/material/Box'
import Modal from '@mui/material/Modal'
import Button from '@mui/material/Button'
import GithubLogin from './GithubLogin'
import ConnectWallet from './ConnectWallet'
import { useCall, useContract, useWallet, useAllWallets } from 'useink'

const metadata = require("../../smart_contracts/deployments/registration/metadata.json");

 
 const CONTRACT_ID =
  "5G2oMVjPfb66uT35gcqFZkecGHEMJf25oarhctUNGMiFj4K3";

 
export default function LoginModal({ onClose, open }) {
  const { data: session } = useSession()
   const contract = useContract(CONTRACT_ID, metadata);
 
  const { account, connect, disconnect } = useWallet()
  const wallets = useAllWallets();
 

  const handleSmartContractRegister = async (type) => {
   
     // Creates a transactions to call the increment function
    // because it creates a TX and updates the contract state this requires the wallet to have enough coins to cover the costs and also to sign the Transaction
    try {
      const args = [session?.user.name]

      const func = useCall(contract, 'join_as_contributor', args);
      await func.send(activeSigner, { signer: activeSigner });
      onClose()
    } catch (error) {
      console.error(error)
    }
  }

  useEffect(() => {
    if (session && account) {
      handleSmartContractRegister()
    }
  }, [session, account])

  return (
    <>
      <Modal
        sx={{ zIndex: 2 }}
        open={open}
        onClose={onClose}
        aria-labelledby="modal-modal-title"
        aria-describedby="modal-modal-description"
      >
        <Box sx={style}>
          <Box>
            <ConnectWallet />
            <GithubLogin />
          </Box>
        </Box>
      </Modal>
    </>
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

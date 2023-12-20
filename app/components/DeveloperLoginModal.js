import { useEffect, useState } from 'react'
import { useSession } from 'next-auth/react'

import Box from '@mui/material/Box'
import Modal from '@mui/material/Modal'
import Button from '@mui/material/Button'
import GithubLogin from './GithubLogin'
import ConnectWallet from './ConnectWallet'
import {
  contractTx,
  contractQuery,
  decodeOutput,
  useInkathon,
  useRegisteredContract,
} from '@scio-labs/use-inkathon'
const registeration = require("../../smart_contracts/deployments/registration/metadata.json");

 
 const CONTRACT_ID =
  "5G2oMVjPfb66uT35gcqFZkecGHEMJf25oarhctUNGMiFj4K3";

 
export default function LoginModal({ onClose, open }) {
  const { data: session } = useSession()
  const { api, activeAccount, activeSigner } = useInkathon()
  const { contract, address: contractAddress } = useRegisteredContract('registration')
 
  // const fetchGreeting = async () => {
  //   if (!contract || !api) return

  //   setFetchIsLoading(true)
  //   try {
  //     const result = await contractQuery(api, '', contract, 'greet')
  //     const { output, isError, decodedOutput } = decodeOutput(result, contract, 'greet')
  //     if (isError) throw new Error(decodedOutput)
  //     setGreeterMessage(output)
  //   } catch (e) {
  //     console.error(e)
  //     toast.error('Error while fetching greeting. Try again…')
  //     setGreeterMessage(undefined)
  //   } finally {
  //     setFetchIsLoading(false)
  //   }
  // }
 

 

 

  const handleSmartContractRegister = async (type) => {
    if (!activeAccount || !contract || !activeSigner || !api) { 
      throw new Error('Wallet not connected. Try again…')
    }
     // Creates a transactions to call the increment function
    // because it creates a TX and updates the contract state this requires the wallet to have enough coins to cover the costs and also to sign the Transaction
    try {
      await contractTx(api, activeAccount.address, contract, 'join_as_contributor', {}, [
        session?.user.name,
      ]);
      onClose()
    } catch (error) {
      console.error(error)
    }
  }

  useEffect(() => {
    if (session && activeAccount) {
      handleSmartContractRegister()
    }
  }, [session, activeAccount])

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

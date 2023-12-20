import Button from '@mui/material/Button'

import { useSession } from 'next-auth/react'


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


const contractAddress = process.env.NEXT_PUBLIC_CONTRACT_ADDRESS

export default function LoginModal() {
  const { data: session } = useSession()
  const { api, activeAccount, activeSigner } = useInkathon()
  const { contract, address: contractAddress } = useRegisteredContract('registration')

  const handleSmartContractRegister = async (type) => {
    if (!activeAccount || !contract || !activeSigner || !api) {
      throw new Error('Wallet not connected. Try again…')
    }
    // Creates a transactions to call the increment function
    // because it creates a TX and updates the contract state this requires the wallet to have enough coins to cover the costs and also to sign the Transaction
    try {
      await contractTx(api, activeAccount.address, contract, 'join_as_open_source_project_creator', {}, [
   
      ]);
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

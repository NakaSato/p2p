// Here we export some useful types and functions for interacting with the Anchor program.
import { Account, getBase58Decoder, SolanaClient } from 'gill'
import { getProgramAccountsDecoded } from './helpers/get-program-accounts-decoded'
import { Dapp, DAPP_DISCRIMINATOR, DAPP_PROGRAM_ADDRESS, getDappDecoder } from './client/js'
import DappIDL from '../target/idl/dapp.json'

export type DappAccount = Account<Dapp, string>

// Re-export the generated IDL and type
export { DappIDL }

export * from './client/js'

export function getDappProgramAccounts(rpc: SolanaClient['rpc']) {
  return getProgramAccountsDecoded(rpc, {
    decoder: getDappDecoder(),
    filter: getBase58Decoder().decode(DAPP_DISCRIMINATOR),
    programAddress: DAPP_PROGRAM_ADDRESS,
  })
}

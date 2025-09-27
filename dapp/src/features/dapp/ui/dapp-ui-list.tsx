import { DappUiCard } from './dapp-ui-card'
import { useDappAccountsQuery } from '@/features/dapp/data-access/use-dapp-accounts-query'
import { UiWalletAccount } from '@wallet-ui/react'

export function DappUiList({ account }: { account: UiWalletAccount }) {
  const dappAccountsQuery = useDappAccountsQuery()

  if (dappAccountsQuery.isLoading) {
    return <span className="loading loading-spinner loading-lg"></span>
  }

  if (!dappAccountsQuery.data?.length) {
    return (
      <div className="text-center">
        <h2 className={'text-2xl'}>No accounts</h2>
        No accounts found. Initialize one to get started.
      </div>
    )
  }

  return (
    <div className="grid lg:grid-cols-2 gap-4">
      {dappAccountsQuery.data?.map((dapp) => (
        <DappUiCard account={account} key={dapp.address} dapp={dapp} />
      ))}
    </div>
  )
}

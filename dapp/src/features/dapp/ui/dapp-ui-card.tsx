import { DappAccount } from '@project/anchor'
import { ellipsify, UiWalletAccount } from '@wallet-ui/react'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { AppExplorerLink } from '@/components/app-explorer-link'
import { DappUiButtonClose } from './dapp-ui-button-close'
import { DappUiButtonDecrement } from './dapp-ui-button-decrement'
import { DappUiButtonIncrement } from './dapp-ui-button-increment'
import { DappUiButtonSet } from './dapp-ui-button-set'

export function DappUiCard({ account, dapp }: { account: UiWalletAccount; dapp: DappAccount }) {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Dapp: {dapp.data.count}</CardTitle>
        <CardDescription>
          Account: <AppExplorerLink address={dapp.address} label={ellipsify(dapp.address)} />
        </CardDescription>
      </CardHeader>
      <CardContent>
        <div className="flex gap-4 justify-evenly">
          <DappUiButtonIncrement account={account} dapp={dapp} />
          <DappUiButtonSet account={account} dapp={dapp} />
          <DappUiButtonDecrement account={account} dapp={dapp} />
          <DappUiButtonClose account={account} dapp={dapp} />
        </div>
      </CardContent>
    </Card>
  )
}

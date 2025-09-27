import { DappAccount } from '@project/anchor'
import { UiWalletAccount } from '@wallet-ui/react'
import { Button } from '@/components/ui/button'

import { useDappDecrementMutation } from '../data-access/use-dapp-decrement-mutation'

export function DappUiButtonDecrement({ account, dapp }: { account: UiWalletAccount; dapp: DappAccount }) {
  const decrementMutation = useDappDecrementMutation({ account, dapp })

  return (
    <Button variant="outline" onClick={() => decrementMutation.mutateAsync()} disabled={decrementMutation.isPending}>
      Decrement
    </Button>
  )
}

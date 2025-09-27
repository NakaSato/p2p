import { DappAccount } from '@project/anchor'
import { UiWalletAccount } from '@wallet-ui/react'
import { Button } from '@/components/ui/button'
import { useDappIncrementMutation } from '../data-access/use-dapp-increment-mutation'

export function DappUiButtonIncrement({ account, dapp }: { account: UiWalletAccount; dapp: DappAccount }) {
  const incrementMutation = useDappIncrementMutation({ account, dapp })

  return (
    <Button variant="outline" onClick={() => incrementMutation.mutateAsync()} disabled={incrementMutation.isPending}>
      Increment
    </Button>
  )
}

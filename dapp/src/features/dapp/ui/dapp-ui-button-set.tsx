import { DappAccount } from '@project/anchor'
import { UiWalletAccount } from '@wallet-ui/react'
import { Button } from '@/components/ui/button'

import { useDappSetMutation } from '@/features/dapp/data-access/use-dapp-set-mutation'

export function DappUiButtonSet({ account, dapp }: { account: UiWalletAccount; dapp: DappAccount }) {
  const setMutation = useDappSetMutation({ account, dapp })

  return (
    <Button
      variant="outline"
      onClick={() => {
        const value = window.prompt('Set value to:', dapp.data.count.toString() ?? '0')
        if (!value || parseInt(value) === dapp.data.count || isNaN(parseInt(value))) {
          return
        }
        return setMutation.mutateAsync(parseInt(value))
      }}
      disabled={setMutation.isPending}
    >
      Set
    </Button>
  )
}

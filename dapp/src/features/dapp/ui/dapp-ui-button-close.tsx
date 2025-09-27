import { DappAccount } from '@project/anchor'
import { UiWalletAccount } from '@wallet-ui/react'
import { Button } from '@/components/ui/button'

import { useDappCloseMutation } from '@/features/dapp/data-access/use-dapp-close-mutation'

export function DappUiButtonClose({ account, dapp }: { account: UiWalletAccount; dapp: DappAccount }) {
  const closeMutation = useDappCloseMutation({ account, dapp })

  return (
    <Button
      variant="destructive"
      onClick={() => {
        if (!window.confirm('Are you sure you want to close this account?')) {
          return
        }
        return closeMutation.mutateAsync()
      }}
      disabled={closeMutation.isPending}
    >
      Close
    </Button>
  )
}

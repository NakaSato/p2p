import { Button } from '@/components/ui/button'
import { UiWalletAccount } from '@wallet-ui/react'

import { useDappInitializeMutation } from '@/features/dapp/data-access/use-dapp-initialize-mutation'

export function DappUiButtonInitialize({ account }: { account: UiWalletAccount }) {
  const mutationInitialize = useDappInitializeMutation({ account })

  return (
    <Button onClick={() => mutationInitialize.mutateAsync()} disabled={mutationInitialize.isPending}>
      Initialize Dapp {mutationInitialize.isPending && '...'}
    </Button>
  )
}

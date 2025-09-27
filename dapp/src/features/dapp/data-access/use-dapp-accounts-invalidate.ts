import { useQueryClient } from '@tanstack/react-query'
import { useDappAccountsQueryKey } from './use-dapp-accounts-query-key'

export function useDappAccountsInvalidate() {
  const queryClient = useQueryClient()
  const queryKey = useDappAccountsQueryKey()

  return () => queryClient.invalidateQueries({ queryKey })
}

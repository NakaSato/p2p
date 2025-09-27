import { useSolana } from '@/components/solana/use-solana'
import { useQuery } from '@tanstack/react-query'
import { getDappProgramAccounts } from '@project/anchor'
import { useDappAccountsQueryKey } from './use-dapp-accounts-query-key'

export function useDappAccountsQuery() {
  const { client } = useSolana()

  return useQuery({
    queryKey: useDappAccountsQueryKey(),
    queryFn: async () => await getDappProgramAccounts(client.rpc),
  })
}

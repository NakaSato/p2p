import { useSolana } from '@/components/solana/use-solana'

export function useDappAccountsQueryKey() {
  const { cluster } = useSolana()

  return ['dapp', 'accounts', { cluster }]
}

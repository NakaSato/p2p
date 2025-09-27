import { useSolana } from '@/components/solana/use-solana'
import { WalletDropdown } from '@/components/wallet-dropdown'
import { AppHero } from '@/components/app-hero'
import { DappUiButtonInitialize } from './ui/dapp-ui-button-initialize'
import { DappUiList } from './ui/dapp-ui-list'
import { DappUiProgramExplorerLink } from './ui/dapp-ui-program-explorer-link'
import { DappUiProgramGuard } from './ui/dapp-ui-program-guard'

export default function DappFeature() {
  const { account } = useSolana()

  return (
    <DappUiProgramGuard>
      <AppHero
        title="Dapp"
        subtitle={
          account
            ? "Initialize a new dapp onchain by clicking the button. Use the program's methods (increment, decrement, set, and close) to change the state of the account."
            : 'Select a wallet to run the program.'
        }
      >
        <p className="mb-6">
          <DappUiProgramExplorerLink />
        </p>
        {account ? (
          <DappUiButtonInitialize account={account} />
        ) : (
          <div style={{ display: 'inline-block' }}>
            <WalletDropdown />
          </div>
        )}
      </AppHero>
      {account ? <DappUiList account={account} /> : null}
    </DappUiProgramGuard>
  )
}

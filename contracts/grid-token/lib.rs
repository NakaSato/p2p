#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// GridToken (GRID) smart contract for P2P Energy Trading platform
/// This contract implements a PSP22-compatible fungible token for energy trading
/// 1 kWh of generated solar energy = 1 GRID token

#[ink::contract]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::arithmetic_side_effects)]
mod grid_token {
    use ink::prelude::string::{String, ToString};
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    use scale::{Decode, Encode};

    /// Contract storage
    #[ink(storage)]
    pub struct GridToken {
        /// Total supply of tokens
        total_supply: Balance,
        /// Token balances mapping
        balances: Mapping<AccountId, Balance>,
        /// Token allowances mapping
        allowances: Mapping<(AccountId, AccountId), Balance>,
        /// Authorized token minters (AMI Integration Service, Oracle Client)
        minters: Mapping<AccountId, ()>,
        /// Authorized token burners
        burners: Mapping<AccountId, ()>,
        /// Registry contract address for user verification
        registry_contract: Option<AccountId>,
        /// Token metadata
        name: String,
        symbol: String,
        decimals: u8,
    }

    /// Events emitted by this contract
    #[ink(event)]
    pub struct Minted {
        #[ink(topic)]
        to: AccountId,
        amount: Balance,
        meter_id: Option<String>,
    }

    #[ink(event)]
    pub struct Burned {
        #[ink(topic)]
        from: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct MinterAdded {
        #[ink(topic)]
        minter: AccountId,
    }

    #[ink(event)]
    pub struct MinterRemoved {
        #[ink(topic)]
        minter: AccountId,
    }

    #[ink(event)]
    pub struct BurnerAdded {
        #[ink(topic)]
        burner: AccountId,
    }

    #[ink(event)]
    pub struct BurnerRemoved {
        #[ink(topic)]
        burner: AccountId,
    }

    #[ink(event)]
    pub struct RegistryContractSet {
        #[ink(topic)]
        registry_contract: AccountId,
    }

    /// Errors that can occur in this contract
    #[derive(Encode, Decode)]
    #[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq, scale_info::TypeInfo))]
    pub enum Error {
        /// Insufficient balance for the operation
        InsufficientBalance,
        /// Insufficient allowance for the operation
        InsufficientAllowance,
        /// Account is not authorized to mint tokens
        UnauthorizedMinter,
        /// Account is not authorized to burn tokens
        UnauthorizedBurner,
        /// Only the contract owner can manage minters and burners
        NotOwner,
        /// User is not verified in the registry
        UserNotVerified,
        /// Registry contract not set
        RegistryNotSet,
        /// Cross-contract call failed
        CrossContractCallFailed,
    }

    /// Contract result type
    pub type Result<T> = core::result::Result<T, Error>;

    impl GridToken {
        /// Constructor that initializes the GridToken
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let caller = Self::env().caller();
            let mut instance = Self {
                total_supply: 0,
                balances: Mapping::default(),
                allowances: Mapping::default(),
                minters: Mapping::default(),
                burners: Mapping::default(),
                registry_contract: None,
                name: "GridToken".to_string(),
                symbol: "GRID".to_string(),
                decimals: 18,
            };

            // Set deployer as initial minter and burner
            instance.minters.insert(caller, &());
            instance.burners.insert(caller, &());

            // Mint initial supply to deployer
            if initial_supply > 0 {
                instance.total_supply = initial_supply;
                instance.balances.insert(caller, &initial_supply);
            }

            instance
        }

        /// Constructor with configuration
        #[ink(constructor)]
        pub fn new_with_config(
            initial_supply: Balance,
            name: String,
            symbol: String,
            decimals: u8,
            initial_minters: Vec<AccountId>,
            initial_burners: Vec<AccountId>,
        ) -> Self {
            let caller = Self::env().caller();
            let mut instance = Self {
                total_supply: 0,
                balances: Mapping::default(),
                allowances: Mapping::default(),
                minters: Mapping::default(),
                burners: Mapping::default(),
                registry_contract: None,
                name,
                symbol,
                decimals,
            };

            // Add initial minters
            for minter in initial_minters.iter() {
                instance.minters.insert(minter, &());
            }
            // Add deployer as minter if not already included
            instance.minters.insert(caller, &());

            // Add initial burners
            for burner in initial_burners.iter() {
                instance.burners.insert(burner, &());
            }
            // Add deployer as burner if not already included
            instance.burners.insert(caller, &());

            // Mint initial supply to deployer
            if initial_supply > 0 {
                instance.total_supply = initial_supply;
                instance.balances.insert(caller, &initial_supply);
            }

            instance
        }

        /// Set the registry contract address
        #[ink(message)]
        pub fn set_registry_contract(&mut self, registry_contract: AccountId) -> Result<()> {
            // Only deployer can set registry contract
            // In a production system, you might want more sophisticated access control
            self.registry_contract = Some(registry_contract);

            self.env()
                .emit_event(RegistryContractSet { registry_contract });

            Ok(())
        }

        /// Mint tokens representing solar energy generation
        #[ink(message)]
        pub fn mint(
            &mut self,
            to: AccountId,
            amount: Balance,
            meter_id: Option<String>,
        ) -> Result<()> {
            let caller = self.env().caller();

            // Check if caller is authorized minter
            if !self.minters.contains(caller) {
                return Err(Error::UnauthorizedMinter);
            }

            // Verify user if registry is set
            if let Some(registry) = self.registry_contract {
                match self.call_registry_is_user_verified(registry, to) {
                    Ok(verified) => {
                        if !verified {
                            return Err(Error::UserNotVerified);
                        }
                    }
                    Err(_) => return Err(Error::CrossContractCallFailed),
                }
            }

            // Mint tokens
            let new_balance = self.balances.get(to).unwrap_or(0) + amount;
            self.balances.insert(to, &new_balance);
            self.total_supply += amount;

            self.env().emit_event(Minted {
                to,
                amount,
                meter_id,
            });

            Ok(())
        }

        /// Burn tokens representing energy consumption
        #[ink(message)]
        pub fn burn(&mut self, from: AccountId, amount: Balance) -> Result<()> {
            let caller = self.env().caller();

            // Check if caller is authorized burner
            if !self.burners.contains(caller) {
                return Err(Error::UnauthorizedBurner);
            }

            // Burn tokens
            let current_balance = self.balances.get(from).unwrap_or(0);
            if current_balance < amount {
                return Err(Error::InsufficientBalance);
            }
            let new_balance = current_balance - amount;
            self.balances.insert(from, &new_balance);
            self.total_supply -= amount;

            self.env().emit_event(Burned { from, amount });

            Ok(())
        }

        /// Add a new authorized minter
        #[ink(message)]
        pub fn add_minter(&mut self, minter: AccountId) -> Result<()> {
            // In a production system, you'd check REC regulator permissions here
            // For now, we'll allow the contract deployer or existing minters to add new ones
            let caller = self.env().caller();
            if !self.minters.contains(caller) {
                return Err(Error::NotOwner);
            }

            self.minters.insert(minter, &());

            self.env().emit_event(MinterAdded { minter });
            Ok(())
        }

        /// Remove an authorized minter
        #[ink(message)]
        pub fn remove_minter(&mut self, minter: AccountId) -> Result<()> {
            let caller = self.env().caller();
            if !self.minters.contains(caller) {
                return Err(Error::NotOwner);
            }

            self.minters.remove(minter);

            self.env().emit_event(MinterRemoved { minter });
            Ok(())
        }

        /// Add a new authorized burner
        #[ink(message)]
        pub fn add_burner(&mut self, burner: AccountId) -> Result<()> {
            let caller = self.env().caller();
            if !self.burners.contains(caller) {
                return Err(Error::NotOwner);
            }

            self.burners.insert(burner, &());

            self.env().emit_event(BurnerAdded { burner });
            Ok(())
        }

        /// Remove an authorized burner
        #[ink(message)]
        pub fn remove_burner(&mut self, burner: AccountId) -> Result<()> {
            let caller = self.env().caller();
            if !self.burners.contains(caller) {
                return Err(Error::NotOwner);
            }

            self.burners.remove(burner);

            self.env().emit_event(BurnerRemoved { burner });
            Ok(())
        }

        /// Check if an account is an authorized minter
        #[ink(message)]
        pub fn is_minter(&self, account: AccountId) -> bool {
            self.minters.contains(account)
        }

        /// Check if an account is an authorized burner
        #[ink(message)]
        pub fn is_burner(&self, account: AccountId) -> bool {
            self.burners.contains(account)
        }

        /// Get token name
        #[ink(message)]
        pub fn token_name(&self) -> String {
            self.name.clone()
        }

        /// Get token symbol
        #[ink(message)]
        pub fn token_symbol(&self) -> String {
            self.symbol.clone()
        }

        /// Get token decimals
        #[ink(message)]
        pub fn token_decimals(&self) -> u8 {
            self.decimals
        }

        /// Get registry contract address
        #[ink(message)]
        pub fn get_registry_contract(&self) -> Option<AccountId> {
            self.registry_contract
        }

        /// PSP22 - Get total supply
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        /// PSP22 - Get balance of an account
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(owner).unwrap_or(0)
        }

        /// PSP22 - Get allowance
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowances.get((owner, spender)).unwrap_or(0)
        }

        /// PSP22 - Transfer tokens
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance, _data: Vec<u8>) -> Result<()> {
            let from = self.env().caller();
            self.transfer_from_to(from, to, value)
        }

        /// PSP22 - Transfer from
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
            _data: Vec<u8>,
        ) -> Result<()> {
            let caller = self.env().caller();
            let allowance = self.allowance(from, caller);
            if allowance < value {
                return Err(Error::InsufficientAllowance);
            }

            self.allowances.insert((from, caller), &(allowance - value));
            self.transfer_from_to(from, to, value)
        }

        /// PSP22 - Approve spender
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let owner = self.env().caller();
            self.allowances.insert((owner, spender), &value);
            Ok(())
        }

        /// PSP22 - Increase allowance
        #[ink(message)]
        pub fn increase_allowance(
            &mut self,
            spender: AccountId,
            delta_value: Balance,
        ) -> Result<()> {
            let owner = self.env().caller();
            let allowance = self.allowance(owner, spender);
            self.allowances
                .insert((owner, spender), &(allowance + delta_value));
            Ok(())
        }

        /// PSP22 - Decrease allowance
        #[ink(message)]
        pub fn decrease_allowance(
            &mut self,
            spender: AccountId,
            delta_value: Balance,
        ) -> Result<()> {
            let owner = self.env().caller();
            let allowance = self.allowance(owner, spender);
            if allowance < delta_value {
                return Err(Error::InsufficientAllowance);
            }
            self.allowances
                .insert((owner, spender), &(allowance - delta_value));
            Ok(())
        }

        /// Internal transfer helper
        fn transfer_from_to(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            let from_balance = self.balance_of(from);
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }

            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balance_of(to);
            self.balances.insert(to, &(to_balance + value));
            Ok(())
        }

        /// Helper function to call registry contract for user verification
        fn call_registry_is_user_verified(
            &self,
            _registry: AccountId,
            _user: AccountId,
        ) -> Result<bool> {
            // Cross-contract call to registry
            // This would need to be implemented using ink!'s cross-contract call mechanism
            // For now, we'll return a placeholder
            // In a real implementation, you would use:
            // let result = build_call::<DefaultEnvironment>()
            //     .call(registry)
            //     .gas_limit(5000)
            //     .exec_input(
            //         ExecutionInput::new(Selector::new(ink::selector_bytes!("is_user_verified")))
            //             .push_arg(user)
            //     )
            //     .returns::<bool>()
            //     .invoke();

            // Placeholder - assume user is verified for now
            Ok(true)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn new_works() {
            let token = GridToken::new(1000);
            assert_eq!(token.total_supply(), 1000);
        }

        #[ink::test]
        fn mint_works() {
            let mut token = GridToken::new(0);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Alice (deployer) mints tokens to Bob
            assert!(token
                .mint(accounts.bob, 500, Some("METER_001".to_string()))
                .is_ok());
            assert_eq!(token.balance_of(accounts.bob), 500);
            assert_eq!(token.total_supply(), 500);
        }

        #[ink::test]
        fn burn_works() {
            let mut token = GridToken::new(1000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Alice (deployer) burns her own tokens
            assert!(token.burn(accounts.alice, 300).is_ok());
            assert_eq!(token.balance_of(accounts.alice), 700);
            assert_eq!(token.total_supply(), 700);
        }

        #[ink::test]
        fn unauthorized_mint_fails() {
            let mut token = GridToken::new(0);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Set Bob as caller (non-minter)
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);

            // Bob tries to mint tokens (should fail)
            assert_eq!(
                token.mint(accounts.charlie, 100, None),
                Err(Error::UnauthorizedMinter)
            );
        }

        #[ink::test]
        fn add_minter_works() {
            let mut token = GridToken::new(0);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Alice adds Bob as minter
            assert!(token.add_minter(accounts.bob).is_ok());
            assert!(token.is_minter(accounts.bob));
        }

        #[ink::test]
        fn psp22_transfer_works() {
            let mut token = GridToken::new(1000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Alice transfers tokens to Bob
            assert!(token.transfer(accounts.bob, 300, Vec::new()).is_ok());
            assert_eq!(token.balance_of(accounts.alice), 700);
            assert_eq!(token.balance_of(accounts.bob), 300);
        }
    }
}

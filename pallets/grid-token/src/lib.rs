#![cfg_attr(not(feature = "std"), no_std)]

/// GridToken (GRID) pallet for P2P Energy Trading platform
/// This pallet implements a PSP22-like fungible token for energy trading
/// 1 kWh of generated solar energy = 1 GRID token
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use codec::{Decode, Encode};
    use frame_support::{
        dispatch::{DispatchError, DispatchResult},
        pallet_prelude::*,
        traits::{Get, StorageVersion},
    };
    use frame_system::pallet_prelude::*;
    use scale_info::TypeInfo;
    use sp_runtime::traits::{AtLeast32BitUnsigned, CheckedAdd, CheckedSub, Saturating, Zero};
    use sp_std::{collections::btree_set::BTreeSet, vec::Vec};

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_registry::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// The balance type for this pallet
        type Balance: Parameter
            + Member
            + AtLeast32BitUnsigned
            + Default
            + Copy
            + MaybeSerializeDeserialize
            + MaxEncodedLen
            + TypeInfo;

        /// Maximum number of authorized minters
        #[pallet::constant]
        type MaxMinters: Get<u32>;
    }

    /// Total supply of GRID tokens
    #[pallet::storage]
    #[pallet::getter(fn total_supply)]
    pub type TotalSupply<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

    /// Token balances for each account
    #[pallet::storage]
    #[pallet::getter(fn balances)]
    pub type Balances<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, T::Balance, ValueQuery>;

    /// Allowances for spending tokens on behalf of other accounts
    #[pallet::storage]
    #[pallet::getter(fn allowances)]
    pub type Allowances<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId, // owner
        Blake2_128Concat,
        T::AccountId, // spender
        T::Balance,
        ValueQuery,
    >;

    /// Authorized token minters (AMI Integration Service, Oracle Client)
    #[pallet::storage]
    #[pallet::getter(fn minters)]
    pub type Minters<T: Config> = StorageValue<_, BTreeSet<T::AccountId>, ValueQuery>;

    /// Authorized token burners
    #[pallet::storage]
    #[pallet::getter(fn burners)]
    pub type Burners<T: Config> = StorageValue<_, BTreeSet<T::AccountId>, ValueQuery>;

    /// Genesis configuration
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        /// Initial token balances
        pub balances: Vec<(T::AccountId, T::Balance)>,
        /// Initial minters
        pub minters: Vec<T::AccountId>,
        /// Initial burners
        pub burners: Vec<T::AccountId>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                balances: Default::default(),
                minters: Default::default(),
                burners: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            let mut total_supply = T::Balance::zero();

            for (account, balance) in &self.balances {
                Balances::<T>::insert(account, balance);
                total_supply = total_supply.saturating_add(*balance);
            }
            TotalSupply::<T>::put(total_supply);

            let mut minter_set = BTreeSet::new();
            for minter in &self.minters {
                minter_set.insert(minter.clone());
            }
            Minters::<T>::put(minter_set);

            let mut burner_set = BTreeSet::new();
            for burner in &self.burners {
                burner_set.insert(burner.clone());
            }
            Burners::<T>::put(burner_set);
        }
    }

    /// Events emitted by this pallet
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Tokens transferred between accounts
        Transfer {
            from: T::AccountId,
            to: T::AccountId,
            amount: T::Balance,
        },
        /// Approval granted for spending tokens
        Approval {
            owner: T::AccountId,
            spender: T::AccountId,
            amount: T::Balance,
        },
        /// Tokens minted to an account
        Minted {
            to: T::AccountId,
            amount: T::Balance,
            meter_id: Option<Vec<u8>>,
        },
        /// Tokens burned from an account
        Burned {
            from: T::AccountId,
            amount: T::Balance,
        },
        /// Minter added to authorized list
        MinterAdded { minter: T::AccountId },
        /// Minter removed from authorized list
        MinterRemoved { minter: T::AccountId },
        /// Burner added to authorized list
        BurnerAdded { burner: T::AccountId },
        /// Burner removed from authorized list
        BurnerRemoved { burner: T::AccountId },
    }

    /// Errors that can occur in this pallet
    #[pallet::error]
    pub enum Error<T> {
        /// Insufficient balance for the operation
        InsufficientBalance,
        /// Insufficient allowance for the operation
        InsufficientAllowance,
        /// Account is not authorized to mint tokens
        UnauthorizedMinter,
        /// Account is not authorized to burn tokens
        UnauthorizedBurner,
        /// Only admins can manage minters and burners
        NotAdmin,
        /// Arithmetic overflow occurred
        Overflow,
        /// Cannot transfer to the same account
        SelfTransfer,
        /// Minter already exists
        MinterAlreadyExists,
        /// Minter not found
        MinterNotFound,
        /// Burner already exists
        BurnerAlreadyExists,
        /// Burner not found
        BurnerNotFound,
        /// User is not verified in the registry
        UserNotVerified,
    }

    /// Dispatchable functions
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Transfer tokens to another account
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn transfer(
            origin: OriginFor<T>,
            to: T::AccountId,
            amount: T::Balance,
        ) -> DispatchResult {
            let from = ensure_signed(origin)?;
            Self::do_transfer(&from, &to, amount)?;
            Ok(())
        }

        /// Transfer tokens from one account to another using allowance
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn transfer_from(
            origin: OriginFor<T>,
            from: T::AccountId,
            to: T::AccountId,
            amount: T::Balance,
        ) -> DispatchResult {
            let spender = ensure_signed(origin)?;

            // Check allowance
            let allowance = Allowances::<T>::get(&from, &spender);
            ensure!(allowance >= amount, Error::<T>::InsufficientAllowance);

            // Perform transfer
            Self::do_transfer(&from, &to, amount)?;

            // Update allowance
            let new_allowance = allowance.saturating_sub(amount);
            if new_allowance == T::Balance::zero() {
                Allowances::<T>::remove(&from, &spender);
            } else {
                Allowances::<T>::insert(&from, &spender, new_allowance);
            }

            Ok(())
        }

        /// Approve another account to spend tokens on your behalf
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn approve(
            origin: OriginFor<T>,
            spender: T::AccountId,
            amount: T::Balance,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            if amount == T::Balance::zero() {
                Allowances::<T>::remove(&owner, &spender);
            } else {
                Allowances::<T>::insert(&owner, &spender, amount);
            }

            Self::deposit_event(Event::Approval {
                owner,
                spender,
                amount,
            });

            Ok(())
        }

        /// Mint tokens representing solar energy generation
        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn mint(
            origin: OriginFor<T>,
            to: T::AccountId,
            amount: T::Balance,
            meter_id: Option<Vec<u8>>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_minter(&who), Error::<T>::UnauthorizedMinter);
            ensure!(
                pallet_registry::Pallet::<T>::is_user_verified(&to),
                Error::<T>::UserNotVerified
            );

            // Mint tokens
            Balances::<T>::try_mutate(&to, |balance| {
                *balance = balance.checked_add(&amount).ok_or(Error::<T>::Overflow)?;
                Ok::<(), Error<T>>(())
            })?;

            TotalSupply::<T>::try_mutate(|supply| {
                *supply = supply.checked_add(&amount).ok_or(Error::<T>::Overflow)?;
                Ok::<(), Error<T>>(())
            })?;

            Self::deposit_event(Event::Minted {
                to,
                amount,
                meter_id,
            });

            Ok(())
        }

        /// Burn tokens representing energy consumption
        #[pallet::call_index(4)]
        #[pallet::weight(10_000)]
        pub fn burn(
            origin: OriginFor<T>,
            from: T::AccountId,
            amount: T::Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_burner(&who), Error::<T>::UnauthorizedBurner);

            // Burn tokens
            Balances::<T>::try_mutate(&from, |balance| {
                ensure!(*balance >= amount, Error::<T>::InsufficientBalance);
                *balance = balance.saturating_sub(amount);
                Ok::<(), Error<T>>(())
            })?;

            TotalSupply::<T>::mutate(|supply| {
                *supply = supply.saturating_sub(amount);
            });

            Self::deposit_event(Event::Burned { from, amount });

            Ok(())
        }

        /// Add a new authorized minter (admin only)
        #[pallet::call_index(5)]
        #[pallet::weight(10_000)]
        pub fn add_minter(origin: OriginFor<T>, minter: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(
                pallet_registry::Pallet::<T>::is_admin(&who),
                Error::<T>::NotAdmin
            );

            Minters::<T>::try_mutate(|minters| {
                ensure!(!minters.contains(&minter), Error::<T>::MinterAlreadyExists);
                minters.insert(minter.clone());
                Ok(())
            })?;

            Self::deposit_event(Event::MinterAdded { minter });
            Ok(())
        }

        /// Remove an authorized minter (admin only)
        #[pallet::call_index(6)]
        #[pallet::weight(10_000)]
        pub fn remove_minter(origin: OriginFor<T>, minter: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(
                pallet_registry::Pallet::<T>::is_admin(&who),
                Error::<T>::NotAdmin
            );

            Minters::<T>::try_mutate(|minters| {
                ensure!(minters.contains(&minter), Error::<T>::MinterNotFound);
                minters.remove(&minter);
                Ok(())
            })?;

            Self::deposit_event(Event::MinterRemoved { minter });
            Ok(())
        }

        /// Add a new authorized burner (admin only)
        #[pallet::call_index(7)]
        #[pallet::weight(10_000)]
        pub fn add_burner(origin: OriginFor<T>, burner: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(
                pallet_registry::Pallet::<T>::is_admin(&who),
                Error::<T>::NotAdmin
            );

            Burners::<T>::try_mutate(|burners| {
                ensure!(!burners.contains(&burner), Error::<T>::BurnerAlreadyExists);
                burners.insert(burner.clone());
                Ok(())
            })?;

            Self::deposit_event(Event::BurnerAdded { burner });
            Ok(())
        }

        /// Remove an authorized burner (admin only)
        #[pallet::call_index(8)]
        #[pallet::weight(10_000)]
        pub fn remove_burner(origin: OriginFor<T>, burner: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(
                pallet_registry::Pallet::<T>::is_admin(&who),
                Error::<T>::NotAdmin
            );

            Burners::<T>::try_mutate(|burners| {
                ensure!(burners.contains(&burner), Error::<T>::BurnerNotFound);
                burners.remove(&burner);
                Ok(())
            })?;

            Self::deposit_event(Event::BurnerRemoved { burner });
            Ok(())
        }
    }

    /// Helper functions
    impl<T: Config> Pallet<T> {
        /// Internal transfer function
        fn do_transfer(
            from: &T::AccountId,
            to: &T::AccountId,
            amount: T::Balance,
        ) -> DispatchResult {
            ensure!(from != to, Error::<T>::SelfTransfer);
            ensure!(amount > T::Balance::zero(), Error::<T>::InsufficientBalance);

            // Check sender balance
            let from_balance = Balances::<T>::get(from);
            ensure!(from_balance >= amount, Error::<T>::InsufficientBalance);

            // Update balances
            Balances::<T>::insert(from, from_balance.saturating_sub(amount));
            Balances::<T>::mutate(to, |balance| {
                *balance = balance.saturating_add(amount);
            });

            Self::deposit_event(Event::Transfer {
                from: from.clone(),
                to: to.clone(),
                amount,
            });

            Ok(())
        }

        /// Check if an account is an authorized minter
        pub fn is_minter(account: &T::AccountId) -> bool {
            Minters::<T>::get().contains(account)
        }

        /// Check if an account is an authorized burner
        pub fn is_burner(account: &T::AccountId) -> bool {
            Burners::<T>::get().contains(account)
        }

        /// Get the balance of an account
        pub fn balance_of(account: &T::AccountId) -> T::Balance {
            Balances::<T>::get(account)
        }

        /// Get the allowance between owner and spender
        pub fn allowance(owner: &T::AccountId, spender: &T::AccountId) -> T::Balance {
            Allowances::<T>::get(owner, spender)
        }

        /// Get the total supply of tokens
        pub fn get_total_supply() -> T::Balance {
            TotalSupply::<T>::get()
        }
    }
}

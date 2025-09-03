#![cfg_attr(not(feature = "std"), no_std)]

/// Oracle Client pallet for P2P Energy Trading platform
/// This pallet acts as a secure bridge between the blockchain and external oracle networks
/// for automated operations like market clearing and energy data verification.
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
    use pallet_timestamp as timestamp;
    use scale_info::TypeInfo;
    use sp_runtime::traits::{AtLeast32BitUnsigned, Saturating, Zero};
    use sp_std::{collections::btree_set::BTreeSet, vec::Vec};

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config:
        frame_system::Config
        + pallet_registry::Config
        + pallet_grid_token::Config
        + pallet_trading::Config
        + timestamp::Config
    {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// The balance type for oracle operations
        type Balance: Parameter
            + Member
            + AtLeast32BitUnsigned
            + Default
            + Copy
            + MaybeSerializeDeserialize
            + MaxEncodedLen
            + TypeInfo;

        /// Energy amount type (kWh in smallest unit)
        type EnergyAmount: Parameter
            + Member
            + AtLeast32BitUnsigned
            + Default
            + Copy
            + MaybeSerializeDeserialize
            + MaxEncodedLen
            + TypeInfo;

        /// Maximum number of pending oracle requests
        #[pallet::constant]
        type MaxOracleRequests: Get<u32>;

        /// Oracle request timeout in blocks
        #[pallet::constant]
        type OracleTimeout: Get<u32>;
    }

    /// Oracle request ID type
    pub type RequestId = u64;

    /// Meter data structure from oracle
    #[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
    pub struct MeterData {
        /// Meter identifier
        pub meter_id: Vec<u8>,
        /// Energy generation amount (kWh)
        pub energy_generated: u64,
        /// Energy consumption amount (kWh)
        pub energy_consumed: u64,
        /// Timestamp of the reading
        pub timestamp: u64,
        /// Digital signature from the meter
        pub signature: Vec<u8>,
    }

    /// Oracle request types
    #[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
    pub enum RequestType {
        /// Request energy data from AMI Head-End API
        EnergyData { meter_id: Vec<u8> },
        /// Request market clearing check
        MarketClearing,
    }

    /// Oracle request structure
    #[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
    pub struct OracleRequest<AccountId, Moment> {
        /// Request ID
        pub id: RequestId,
        /// Requester account
        pub requester: AccountId,
        /// Type of request
        pub request_type: RequestType,
        /// Request timestamp
        pub requested_at: Moment,
        /// Request status
        pub status: RequestStatus,
        /// Block number when request was made
        pub block_number: u32,
    }

    /// Request status enumeration
    #[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
    pub enum RequestStatus {
        /// Request is pending oracle response
        Pending,
        /// Request has been fulfilled
        Fulfilled,
        /// Request has expired
        Expired,
        /// Request failed
        Failed,
    }

    /// Next oracle request ID
    #[pallet::storage]
    #[pallet::getter(fn next_request_id)]
    pub type NextRequestId<T: Config> = StorageValue<_, RequestId, ValueQuery>;

    /// Pending oracle requests
    #[pallet::storage]
    #[pallet::getter(fn oracle_requests)]
    pub type OracleRequests<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        RequestId,
        OracleRequest<T::AccountId, T::Moment>,
        OptionQuery,
    >;

    /// Authorized oracle operators
    #[pallet::storage]
    #[pallet::getter(fn oracle_operators)]
    pub type OracleOperators<T: Config> = StorageValue<_, BTreeSet<T::AccountId>, ValueQuery>;

    /// Oracle funding balance for paying oracle fees
    #[pallet::storage]
    #[pallet::getter(fn oracle_balance)]
    pub type OracleBalance<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

    /// Last market clearing check timestamp
    #[pallet::storage]
    #[pallet::getter(fn last_market_check)]
    pub type LastMarketCheck<T: Config> = StorageValue<_, T::Moment, ValueQuery>;

    /// Automatic market clearing enabled flag
    #[pallet::storage]
    #[pallet::getter(fn auto_market_clearing)]
    pub type AutoMarketClearing<T: Config> = StorageValue<_, bool, ValueQuery>;

    /// Genesis configuration
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        /// Initial oracle operators
        pub oracle_operators: Vec<T::AccountId>,
        /// Initial oracle balance
        pub oracle_balance: T::Balance,
        /// Enable automatic market clearing
        pub auto_market_clearing: bool,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                oracle_operators: Default::default(),
                oracle_balance: Default::default(),
                auto_market_clearing: true,
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            let mut operator_set = BTreeSet::new();
            for operator in &self.oracle_operators {
                operator_set.insert(operator.clone());
            }
            OracleOperators::<T>::put(operator_set);
            OracleBalance::<T>::put(self.oracle_balance);
            AutoMarketClearing::<T>::put(self.auto_market_clearing);
        }
    }

    /// Events emitted by this pallet
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Oracle request created
        OracleRequestCreated {
            request_id: RequestId,
            requester: T::AccountId,
            request_type: RequestType,
        },
        /// Energy data fulfilled by oracle
        EnergyDataFulfilled {
            request_id: RequestId,
            meter_id: Vec<u8>,
            energy_generated: u64,
            energy_consumed: u64,
            tokens_minted: T::Balance,
        },
        /// Market clearing check performed
        MarketClearingChecked {
            request_id: RequestId,
            clearing_needed: bool,
        },
        /// Automated market clearing triggered
        AutoMarketClearingTriggered { timestamp: T::Moment },
        /// Oracle request expired
        OracleRequestExpired { request_id: RequestId },
        /// Oracle operator added
        OracleOperatorAdded { operator: T::AccountId },
        /// Oracle operator removed
        OracleOperatorRemoved { operator: T::AccountId },
        /// Oracle balance funded
        OracleFunded {
            amount: T::Balance,
            new_balance: T::Balance,
        },
    }

    /// Errors that can occur in this pallet
    #[pallet::error]
    pub enum Error<T> {
        /// Request not found
        RequestNotFound,
        /// Only authorized oracle operators can fulfill requests
        NotOracleOperator,
        /// Only admins can manage oracle operators
        NotAdmin,
        /// Request has already been fulfilled
        RequestAlreadyFulfilled,
        /// Request has expired
        RequestExpired,
        /// Invalid meter data signature
        InvalidSignature,
        /// Meter not found in registry
        MeterNotFound,
        /// User not verified for the meter
        UserNotVerified,
        /// Insufficient oracle balance
        InsufficientOracleBalance,
        /// Too many pending requests
        TooManyPendingRequests,
        /// Oracle operator already exists
        OperatorAlreadyExists,
        /// Oracle operator not found
        OperatorNotFound,
        /// Arithmetic overflow
        Overflow,
    }

    /// Dispatchable functions
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Request energy data from oracle for a specific meter
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn request_energy_data(origin: OriginFor<T>, meter_id: Vec<u8>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Verify the requester has permission (admin or meter owner)
            let meter_owner = pallet_registry::Pallet::<T>::get_meter_owner(&meter_id)
                .ok_or(Error::<T>::MeterNotFound)?;

            ensure!(
                pallet_registry::Pallet::<T>::is_admin(&who) || who == meter_owner,
                Error::<T>::UserNotVerified
            );

            // Check oracle balance
            ensure!(
                OracleBalance::<T>::get() > T::Balance::zero(),
                Error::<T>::InsufficientOracleBalance
            );

            let request_id = NextRequestId::<T>::get();
            let current_time = timestamp::Pallet::<T>::get();
            let current_block = frame_system::Pallet::<T>::block_number();

            let request = OracleRequest {
                id: request_id,
                requester: who.clone(),
                request_type: RequestType::EnergyData {
                    meter_id: meter_id.clone(),
                },
                requested_at: current_time,
                status: RequestStatus::Pending,
                block_number: current_block.saturated_into::<u32>(),
            };

            OracleRequests::<T>::insert(request_id, &request);
            NextRequestId::<T>::mutate(|id| *id = id.saturating_add(1));

            Self::deposit_event(Event::OracleRequestCreated {
                request_id,
                requester: who,
                request_type: RequestType::EnergyData { meter_id },
            });

            Ok(())
        }

        /// Fulfill energy data request (oracle operator only)
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn fulfill_energy_data(
            origin: OriginFor<T>,
            request_id: RequestId,
            meter_data: MeterData,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(
                Self::is_oracle_operator(&who),
                Error::<T>::NotOracleOperator
            );

            let mut request =
                OracleRequests::<T>::get(request_id).ok_or(Error::<T>::RequestNotFound)?;

            ensure!(
                matches!(request.status, RequestStatus::Pending),
                Error::<T>::RequestAlreadyFulfilled
            );

            // Check if request has expired
            let current_block = frame_system::Pallet::<T>::block_number().saturated_into::<u32>();
            let timeout_blocks = T::OracleTimeout::get();

            if current_block > request.block_number.saturating_add(timeout_blocks) {
                request.status = RequestStatus::Expired;
                OracleRequests::<T>::insert(request_id, &request);

                Self::deposit_event(Event::OracleRequestExpired { request_id });
                return Err(Error::<T>::RequestExpired.into());
            }

            // Verify the meter exists and get its owner
            let meter_owner = pallet_registry::Pallet::<T>::get_meter_owner(&meter_data.meter_id)
                .ok_or(Error::<T>::MeterNotFound)?;

            // In a real implementation, you would verify the signature here
            // For now, we'll assume the signature is valid

            // Mint tokens for energy generation
            let mut tokens_minted = T::Balance::zero();
            if meter_data.energy_generated > 0 {
                let energy_balance =
                    T::Balance::from(meter_data.energy_generated.saturated_into::<u32>());
                pallet_grid_token::Pallet::<T>::mint(
                    frame_system::RawOrigin::Signed(Self::account_id()).into(),
                    meter_owner,
                    energy_balance,
                    Some(meter_data.meter_id.clone()),
                )?;
                tokens_minted = energy_balance;
            }

            // Update request status
            request.status = RequestStatus::Fulfilled;
            OracleRequests::<T>::insert(request_id, &request);

            Self::deposit_event(Event::EnergyDataFulfilled {
                request_id,
                meter_id: meter_data.meter_id,
                energy_generated: meter_data.energy_generated,
                energy_consumed: meter_data.energy_consumed,
                tokens_minted,
            });

            Ok(())
        }

        /// Check if market clearing is needed (Chainlink Keepers interface)
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn check_upkeep(origin: OriginFor<T>) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            let clearing_needed = pallet_trading::Pallet::<T>::needs_market_clearing();
            let current_time = timestamp::Pallet::<T>::get();

            if clearing_needed && AutoMarketClearing::<T>::get() {
                // Trigger automatic market clearing
                pallet_trading::Pallet::<T>::match_orders(
                    frame_system::RawOrigin::Signed(Self::account_id()).into(),
                )?;

                Self::deposit_event(Event::AutoMarketClearingTriggered {
                    timestamp: current_time,
                });
            }

            LastMarketCheck::<T>::put(current_time);

            let request_id = NextRequestId::<T>::get();
            Self::deposit_event(Event::MarketClearingChecked {
                request_id,
                clearing_needed,
            });

            Ok(())
        }

        /// Perform automated upkeep (Chainlink Keepers interface)
        #[pallet::call_index(3)]
        #[pallet::weight(50_000)]
        pub fn perform_upkeep(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(
                Self::is_oracle_operator(&who),
                Error::<T>::NotOracleOperator
            );

            if pallet_trading::Pallet::<T>::needs_market_clearing() {
                pallet_trading::Pallet::<T>::match_orders(
                    frame_system::RawOrigin::Signed(Self::account_id()).into(),
                )?;

                Self::deposit_event(Event::AutoMarketClearingTriggered {
                    timestamp: timestamp::Pallet::<T>::get(),
                });
            }

            Ok(())
        }

        /// Fund oracle operations
        #[pallet::call_index(4)]
        #[pallet::weight(10_000)]
        pub fn fund_oracle_operations(origin: OriginFor<T>, amount: T::Balance) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(
                pallet_registry::Pallet::<T>::is_admin(&who),
                Error::<T>::NotAdmin
            );

            OracleBalance::<T>::mutate(|balance| {
                *balance = balance.saturating_add(amount);
            });

            let new_balance = OracleBalance::<T>::get();

            Self::deposit_event(Event::OracleFunded {
                amount,
                new_balance,
            });

            Ok(())
        }

        /// Add oracle operator (admin only)
        #[pallet::call_index(5)]
        #[pallet::weight(10_000)]
        pub fn add_oracle_operator(origin: OriginFor<T>, operator: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(
                pallet_registry::Pallet::<T>::is_admin(&who),
                Error::<T>::NotAdmin
            );

            OracleOperators::<T>::try_mutate(|operators| {
                ensure!(
                    !operators.contains(&operator),
                    Error::<T>::OperatorAlreadyExists
                );
                operators.insert(operator.clone());
                Ok(())
            })?;

            Self::deposit_event(Event::OracleOperatorAdded { operator });
            Ok(())
        }

        /// Remove oracle operator (admin only)
        #[pallet::call_index(6)]
        #[pallet::weight(10_000)]
        pub fn remove_oracle_operator(
            origin: OriginFor<T>,
            operator: T::AccountId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(
                pallet_registry::Pallet::<T>::is_admin(&who),
                Error::<T>::NotAdmin
            );

            OracleOperators::<T>::try_mutate(|operators| {
                ensure!(operators.contains(&operator), Error::<T>::OperatorNotFound);
                operators.remove(&operator);
                Ok(())
            })?;

            Self::deposit_event(Event::OracleOperatorRemoved { operator });
            Ok(())
        }

        /// Enable or disable automatic market clearing
        #[pallet::call_index(7)]
        #[pallet::weight(10_000)]
        pub fn set_auto_market_clearing(origin: OriginFor<T>, enabled: bool) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(
                pallet_registry::Pallet::<T>::is_admin(&who),
                Error::<T>::NotAdmin
            );

            AutoMarketClearing::<T>::put(enabled);
            Ok(())
        }
    }

    /// Hooks for automated operations
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_block_number: T::BlockNumber) -> Weight {
            // Clean up expired requests
            Self::cleanup_expired_requests();

            // Check if automatic market clearing is needed
            if AutoMarketClearing::<T>::get() {
                if pallet_trading::Pallet::<T>::needs_market_clearing() {
                    let _ = pallet_trading::Pallet::<T>::match_orders(
                        frame_system::RawOrigin::Signed(Self::account_id()).into(),
                    );
                }
            }

            Weight::from_parts(10_000, 0)
        }
    }

    /// Helper functions
    impl<T: Config> Pallet<T> {
        /// Check if an account is an authorized oracle operator
        pub fn is_oracle_operator(account: &T::AccountId) -> bool {
            OracleOperators::<T>::get().contains(account)
        }

        /// Get the oracle client pallet's account ID
        pub fn account_id() -> T::AccountId {
            // In a real implementation, this would be derived from the pallet's module ID
            // For now, we'll use a placeholder that needs to be properly implemented
            frame_system::Pallet::<T>::account_id()
        }

        /// Clean up expired oracle requests
        fn cleanup_expired_requests() {
            let current_block = frame_system::Pallet::<T>::block_number().saturated_into::<u32>();
            let timeout_blocks = T::OracleTimeout::get();

            for (request_id, mut request) in OracleRequests::<T>::iter() {
                if matches!(request.status, RequestStatus::Pending) {
                    if current_block > request.block_number.saturating_add(timeout_blocks) {
                        request.status = RequestStatus::Expired;
                        OracleRequests::<T>::insert(request_id, &request);

                        Self::deposit_event(Event::OracleRequestExpired { request_id });
                    }
                }
            }
        }

        /// Get pending requests count
        pub fn pending_requests_count() -> u32 {
            OracleRequests::<T>::iter()
                .filter(|(_, request)| matches!(request.status, RequestStatus::Pending))
                .count() as u32
        }

        /// Check if upkeep is needed (for Chainlink Keepers)
        pub fn upkeep_needed() -> bool {
            pallet_trading::Pallet::<T>::needs_market_clearing() && AutoMarketClearing::<T>::get()
        }
    }
}

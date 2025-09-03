#![cfg_attr(not(feature = "std"), no_std)]

/// Trading pallet for P2P Energy Trading platform
/// This pallet manages the periodic order book, matches buy and sell orders,
/// and settles trades for solar energy trading.
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
        + timestamp::Config
    {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// The balance type for energy amounts and prices
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

        /// Market epoch length in milliseconds (e.g., 900000 for 15 minutes)
        #[pallet::constant]
        type MarketEpochLength: Get<Self::Moment>;

        /// Maximum number of orders per user per epoch
        #[pallet::constant]
        type MaxOrdersPerUser: Get<u32>;

        /// Maximum number of orders per epoch
        #[pallet::constant]
        type MaxOrdersPerEpoch: Get<u32>;
    }

    /// Order ID type
    pub type OrderId = u64;

    /// Market order structure
    #[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
    pub struct Order<AccountId, Balance, EnergyAmount, Moment> {
        /// Unique order ID
        pub id: OrderId,
        /// Order creator
        pub user: AccountId,
        /// Type of order
        pub order_type: OrderType,
        /// Amount of energy (kWh)
        pub energy_amount: EnergyAmount,
        /// Price per unit of energy
        pub price_per_unit: Balance,
        /// Total price (energy_amount * price_per_unit)
        pub total_price: Balance,
        /// Order creation timestamp
        pub created_at: Moment,
        /// Order status
        pub status: OrderStatus,
        /// Amount already filled
        pub filled_amount: EnergyAmount,
    }

    /// Order type enumeration
    #[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
    pub enum OrderType {
        /// Sell order (prosumer selling energy)
        Sell,
        /// Buy order (consumer buying energy)
        Buy,
    }

    /// Order status enumeration
    #[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
    pub enum OrderStatus {
        /// Order is active and can be matched
        Active,
        /// Order is partially filled
        PartiallyFilled,
        /// Order is completely filled
        Filled,
        /// Order was cancelled
        Cancelled,
        /// Order expired
        Expired,
    }

    /// Trade execution record
    #[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
    pub struct Trade<AccountId, Balance, EnergyAmount, Moment> {
        /// Sell order ID
        pub sell_order_id: OrderId,
        /// Buy order ID
        pub buy_order_id: OrderId,
        /// Seller account
        pub seller: AccountId,
        /// Buyer account
        pub buyer: AccountId,
        /// Amount of energy traded
        pub energy_amount: EnergyAmount,
        /// Price per unit
        pub price_per_unit: Balance,
        /// Total trade value
        pub total_value: Balance,
        /// Trade execution timestamp
        pub executed_at: Moment,
    }

    /// Next order ID counter
    #[pallet::storage]
    #[pallet::getter(fn next_order_id)]
    pub type NextOrderId<T: Config> = StorageValue<_, OrderId, ValueQuery>;

    /// Current market epoch start time
    #[pallet::storage]
    #[pallet::getter(fn current_epoch_start)]
    pub type CurrentEpochStart<T: Config> = StorageValue<_, T::Moment, ValueQuery>;

    /// Active sell orders for current epoch
    #[pallet::storage]
    #[pallet::getter(fn sell_orders)]
    pub type SellOrders<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        OrderId,
        Order<T::AccountId, T::Balance, T::EnergyAmount, T::Moment>,
        OptionQuery,
    >;

    /// Active buy orders for current epoch
    #[pallet::storage]
    #[pallet::getter(fn buy_orders)]
    pub type BuyOrders<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        OrderId,
        Order<T::AccountId, T::Balance, T::EnergyAmount, T::Moment>,
        OptionQuery,
    >;

    /// Orders by user for current epoch
    #[pallet::storage]
    #[pallet::getter(fn user_orders)]
    pub type UserOrders<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<OrderId, T::MaxOrdersPerUser>,
        ValueQuery,
    >;

    /// Authorized market makers (Oracle Client, AMI Integration Service)
    #[pallet::storage]
    #[pallet::getter(fn market_makers)]
    pub type MarketMakers<T: Config> = StorageValue<_, BTreeSet<T::AccountId>, ValueQuery>;

    /// Trade history
    #[pallet::storage]
    #[pallet::getter(fn trades)]
    pub type Trades<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u64, // trade ID
        Trade<T::AccountId, T::Balance, T::EnergyAmount, T::Moment>,
        OptionQuery,
    >;

    /// Next trade ID
    #[pallet::storage]
    #[pallet::getter(fn next_trade_id)]
    pub type NextTradeId<T: Config> = StorageValue<_, u64, ValueQuery>;

    /// Genesis configuration
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        /// Initial market makers
        pub market_makers: Vec<T::AccountId>,
        /// Initial epoch start time
        pub epoch_start: T::Moment,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                market_makers: Default::default(),
                epoch_start: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            let mut market_maker_set = BTreeSet::new();
            for market_maker in &self.market_makers {
                market_maker_set.insert(market_maker.clone());
            }
            MarketMakers::<T>::put(market_maker_set);
            CurrentEpochStart::<T>::put(self.epoch_start);
        }
    }

    /// Events emitted by this pallet
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Sell order created
        SellOrderCreated {
            order_id: OrderId,
            user: T::AccountId,
            energy_amount: T::EnergyAmount,
            price_per_unit: T::Balance,
        },
        /// Buy order created
        BuyOrderCreated {
            order_id: OrderId,
            user: T::AccountId,
            energy_amount: T::EnergyAmount,
            max_price: T::Balance,
        },
        /// Order cancelled
        OrderCancelled {
            order_id: OrderId,
            user: T::AccountId,
        },
        /// Orders matched and trade executed
        TradeExecuted {
            trade_id: u64,
            sell_order_id: OrderId,
            buy_order_id: OrderId,
            seller: T::AccountId,
            buyer: T::AccountId,
            energy_amount: T::EnergyAmount,
            price_per_unit: T::Balance,
            total_value: T::Balance,
        },
        /// Market epoch cleared
        MarketCleared {
            epoch_start: T::Moment,
            trades_count: u32,
        },
        /// Market maker added
        MarketMakerAdded { market_maker: T::AccountId },
        /// Market maker removed
        MarketMakerRemoved { market_maker: T::AccountId },
    }

    /// Errors that can occur in this pallet
    #[pallet::error]
    pub enum Error<T> {
        /// User is not verified in the registry
        UserNotVerified,
        /// Only prosumers can create sell orders
        NotProsumer,
        /// Insufficient token allowance for buy order
        InsufficientAllowance,
        /// Order not found
        OrderNotFound,
        /// Cannot cancel order that is not yours
        NotOrderOwner,
        /// Order is not in active status
        OrderNotActive,
        /// Not authorized to perform market operations
        NotMarketMaker,
        /// Only admins can manage market makers
        NotAdmin,
        /// Market maker already exists
        MarketMakerAlreadyExists,
        /// Market maker not found
        MarketMakerNotFound,
        /// Maximum orders per user exceeded
        TooManyOrders,
        /// Invalid price (must be greater than zero)
        InvalidPrice,
        /// Invalid energy amount (must be greater than zero)
        InvalidEnergyAmount,
        /// Arithmetic overflow
        Overflow,
        /// Order has already been filled
        OrderAlreadyFilled,
    }

    /// Dispatchable functions
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a sell order for solar energy
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn create_sell_order(
            origin: OriginFor<T>,
            energy_amount: T::EnergyAmount,
            price_per_unit: T::Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Verify user is registered and is a prosumer
            ensure!(
                pallet_registry::Pallet::<T>::is_user_verified(&who),
                Error::<T>::UserNotVerified
            );
            ensure!(
                pallet_registry::Pallet::<T>::is_prosumer(&who),
                Error::<T>::NotProsumer
            );

            // Validate inputs
            ensure!(
                energy_amount > T::EnergyAmount::zero(),
                Error::<T>::InvalidEnergyAmount
            );
            ensure!(
                price_per_unit > T::Balance::zero(),
                Error::<T>::InvalidPrice
            );

            // Check user order limit
            let user_order_count = UserOrders::<T>::get(&who).len();
            ensure!(
                user_order_count < T::MaxOrdersPerUser::get() as usize,
                Error::<T>::TooManyOrders
            );

            let order_id = NextOrderId::<T>::get();
            let total_price = Self::calculate_total_price(energy_amount, price_per_unit)?;
            let current_time = timestamp::Pallet::<T>::get();

            let order = Order {
                id: order_id,
                user: who.clone(),
                order_type: OrderType::Sell,
                energy_amount,
                price_per_unit,
                total_price,
                created_at: current_time,
                status: OrderStatus::Active,
                filled_amount: T::EnergyAmount::zero(),
            };

            SellOrders::<T>::insert(order_id, &order);
            UserOrders::<T>::try_mutate(&who, |orders| {
                orders
                    .try_push(order_id)
                    .map_err(|_| Error::<T>::TooManyOrders)
            })?;

            NextOrderId::<T>::mutate(|id| *id = id.saturating_add(1));

            Self::deposit_event(Event::SellOrderCreated {
                order_id,
                user: who,
                energy_amount,
                price_per_unit,
            });

            Ok(())
        }

        /// Create a buy order for solar energy
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn create_buy_order(
            origin: OriginFor<T>,
            energy_amount: T::EnergyAmount,
            max_price: T::Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Verify user is registered
            ensure!(
                pallet_registry::Pallet::<T>::is_user_verified(&who),
                Error::<T>::UserNotVerified
            );

            // Validate inputs
            ensure!(
                energy_amount > T::EnergyAmount::zero(),
                Error::<T>::InvalidEnergyAmount
            );
            ensure!(max_price > T::Balance::zero(), Error::<T>::InvalidPrice);

            // Check user order limit
            let user_order_count = UserOrders::<T>::get(&who).len();
            ensure!(
                user_order_count < T::MaxOrdersPerUser::get() as usize,
                Error::<T>::TooManyOrders
            );

            let order_id = NextOrderId::<T>::get();
            let total_price = Self::calculate_total_price(energy_amount, max_price)?;
            let current_time = timestamp::Pallet::<T>::get();

            // Check that user has approved sufficient tokens for the trading contract
            let allowance = pallet_grid_token::Pallet::<T>::allowance(&who, &Self::account_id());
            ensure!(allowance >= total_price, Error::<T>::InsufficientAllowance);

            let order = Order {
                id: order_id,
                user: who.clone(),
                order_type: OrderType::Buy,
                energy_amount,
                price_per_unit: max_price,
                total_price,
                created_at: current_time,
                status: OrderStatus::Active,
                filled_amount: T::EnergyAmount::zero(),
            };

            BuyOrders::<T>::insert(order_id, &order);
            UserOrders::<T>::try_mutate(&who, |orders| {
                orders
                    .try_push(order_id)
                    .map_err(|_| Error::<T>::TooManyOrders)
            })?;

            NextOrderId::<T>::mutate(|id| *id = id.saturating_add(1));

            Self::deposit_event(Event::BuyOrderCreated {
                order_id,
                user: who,
                energy_amount,
                max_price,
            });

            Ok(())
        }

        /// Cancel an active order
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn cancel_order(origin: OriginFor<T>, order_id: OrderId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Try to find the order in sell orders first, then buy orders
            let mut order = SellOrders::<T>::get(order_id)
                .or_else(|| BuyOrders::<T>::get(order_id))
                .ok_or(Error::<T>::OrderNotFound)?;

            ensure!(order.user == who, Error::<T>::NotOrderOwner);
            ensure!(
                matches!(
                    order.status,
                    OrderStatus::Active | OrderStatus::PartiallyFilled
                ),
                Error::<T>::OrderNotActive
            );

            order.status = OrderStatus::Cancelled;

            // Update the order in the appropriate storage
            match order.order_type {
                OrderType::Sell => SellOrders::<T>::insert(order_id, &order),
                OrderType::Buy => BuyOrders::<T>::insert(order_id, &order),
            }

            // Remove from user orders
            UserOrders::<T>::mutate(&who, |orders| {
                orders.retain(|&id| id != order_id);
            });

            Self::deposit_event(Event::OrderCancelled {
                order_id,
                user: who,
            });

            Ok(())
        }

        /// Match orders and clear the market (market maker only)
        #[pallet::call_index(3)]
        #[pallet::weight(100_000)]
        pub fn match_orders(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_market_maker(&who), Error::<T>::NotMarketMaker);

            let current_time = timestamp::Pallet::<T>::get();
            let mut trades_count = 0u32;

            // Get all active sell and buy orders
            let sell_orders: Vec<_> = SellOrders::<T>::iter()
                .filter_map(|(id, order)| {
                    if matches!(
                        order.status,
                        OrderStatus::Active | OrderStatus::PartiallyFilled
                    ) {
                        Some((id, order))
                    } else {
                        None
                    }
                })
                .collect();

            let buy_orders: Vec<_> = BuyOrders::<T>::iter()
                .filter_map(|(id, order)| {
                    if matches!(
                        order.status,
                        OrderStatus::Active | OrderStatus::PartiallyFilled
                    ) {
                        Some((id, order))
                    } else {
                        None
                    }
                })
                .collect();

            // Simple price-time priority matching
            for (sell_id, mut sell_order) in sell_orders {
                for (buy_id, mut buy_order) in &buy_orders {
                    if sell_order.price_per_unit <= buy_order.price_per_unit {
                        let remaining_sell = sell_order
                            .energy_amount
                            .saturating_sub(sell_order.filled_amount);
                        let remaining_buy = buy_order
                            .energy_amount
                            .saturating_sub(buy_order.filled_amount);

                        if remaining_sell > T::EnergyAmount::zero()
                            && remaining_buy > T::EnergyAmount::zero()
                        {
                            let trade_amount = remaining_sell.min(remaining_buy);
                            let trade_price = sell_order.price_per_unit; // Use seller's price

                            // Execute the trade
                            if Self::execute_trade(
                                sell_id,
                                buy_id,
                                &mut sell_order,
                                &mut buy_order.clone(),
                                trade_amount,
                                trade_price,
                                current_time,
                            )
                            .is_ok()
                            {
                                trades_count = trades_count.saturating_add(1);
                            }
                        }
                    }
                }
            }

            // Clear expired orders and update epoch
            Self::clear_expired_orders(current_time);
            CurrentEpochStart::<T>::put(current_time);

            Self::deposit_event(Event::MarketCleared {
                epoch_start: current_time,
                trades_count,
            });

            Ok(())
        }

        /// Add market maker (admin only)
        #[pallet::call_index(4)]
        #[pallet::weight(10_000)]
        pub fn add_market_maker(
            origin: OriginFor<T>,
            market_maker: T::AccountId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(
                pallet_registry::Pallet::<T>::is_admin(&who),
                Error::<T>::NotAdmin
            );

            MarketMakers::<T>::try_mutate(|makers| {
                ensure!(
                    !makers.contains(&market_maker),
                    Error::<T>::MarketMakerAlreadyExists
                );
                makers.insert(market_maker.clone());
                Ok(())
            })?;

            Self::deposit_event(Event::MarketMakerAdded { market_maker });
            Ok(())
        }

        /// Remove market maker (admin only)
        #[pallet::call_index(5)]
        #[pallet::weight(10_000)]
        pub fn remove_market_maker(
            origin: OriginFor<T>,
            market_maker: T::AccountId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(
                pallet_registry::Pallet::<T>::is_admin(&who),
                Error::<T>::NotAdmin
            );

            MarketMakers::<T>::try_mutate(|makers| {
                ensure!(
                    makers.contains(&market_maker),
                    Error::<T>::MarketMakerNotFound
                );
                makers.remove(&market_maker);
                Ok(())
            })?;

            Self::deposit_event(Event::MarketMakerRemoved { market_maker });
            Ok(())
        }
    }

    /// Helper functions
    impl<T: Config> Pallet<T> {
        /// Calculate total price with overflow protection
        fn calculate_total_price(
            energy_amount: T::EnergyAmount,
            price_per_unit: T::Balance,
        ) -> Result<T::Balance, Error<T>> {
            // Convert energy amount to balance type for calculation
            let energy_balance = T::Balance::from(energy_amount.saturated_into::<u32>());
            energy_balance
                .checked_mul(&price_per_unit)
                .ok_or(Error::<T>::Overflow)
        }

        /// Execute a trade between matched orders
        fn execute_trade(
            sell_order_id: OrderId,
            buy_order_id: OrderId,
            sell_order: &mut Order<T::AccountId, T::Balance, T::EnergyAmount, T::Moment>,
            buy_order: &mut Order<T::AccountId, T::Balance, T::EnergyAmount, T::Moment>,
            trade_amount: T::EnergyAmount,
            price_per_unit: T::Balance,
            timestamp: T::Moment,
        ) -> DispatchResult {
            let total_value = Self::calculate_total_price(trade_amount, price_per_unit)?;

            // Execute token transfer using the grid token pallet
            pallet_grid_token::Pallet::<T>::transfer_from(
                frame_system::RawOrigin::Signed(Self::account_id()).into(),
                buy_order.user.clone(),
                sell_order.user.clone(),
                total_value,
            )?;

            // Update order filled amounts
            sell_order.filled_amount = sell_order.filled_amount.saturating_add(trade_amount);
            buy_order.filled_amount = buy_order.filled_amount.saturating_add(trade_amount);

            // Update order statuses
            if sell_order.filled_amount >= sell_order.energy_amount {
                sell_order.status = OrderStatus::Filled;
            } else {
                sell_order.status = OrderStatus::PartiallyFilled;
            }

            if buy_order.filled_amount >= buy_order.energy_amount {
                buy_order.status = OrderStatus::Filled;
            } else {
                buy_order.status = OrderStatus::PartiallyFilled;
            }

            // Update orders in storage
            SellOrders::<T>::insert(sell_order_id, sell_order);
            BuyOrders::<T>::insert(buy_order_id, buy_order);

            // Record the trade
            let trade_id = NextTradeId::<T>::get();
            let trade = Trade {
                sell_order_id,
                buy_order_id,
                seller: sell_order.user.clone(),
                buyer: buy_order.user.clone(),
                energy_amount: trade_amount,
                price_per_unit,
                total_value,
                executed_at: timestamp,
            };

            Trades::<T>::insert(trade_id, &trade);
            NextTradeId::<T>::mutate(|id| *id = id.saturating_add(1));

            Self::deposit_event(Event::TradeExecuted {
                trade_id,
                sell_order_id,
                buy_order_id,
                seller: sell_order.user.clone(),
                buyer: buy_order.user.clone(),
                energy_amount: trade_amount,
                price_per_unit,
                total_value,
            });

            Ok(())
        }

        /// Clear expired orders
        fn clear_expired_orders(current_time: T::Moment) {
            let epoch_length = T::MarketEpochLength::get();
            let epoch_start = CurrentEpochStart::<T>::get();
            let epoch_end = epoch_start.saturating_add(epoch_length);

            if current_time >= epoch_end {
                // Mark unfilled orders as expired and clear user orders
                for (order_id, mut order) in SellOrders::<T>::iter() {
                    if matches!(
                        order.status,
                        OrderStatus::Active | OrderStatus::PartiallyFilled
                    ) {
                        order.status = OrderStatus::Expired;
                        SellOrders::<T>::insert(order_id, &order);
                    }
                }

                for (order_id, mut order) in BuyOrders::<T>::iter() {
                    if matches!(
                        order.status,
                        OrderStatus::Active | OrderStatus::PartiallyFilled
                    ) {
                        order.status = OrderStatus::Expired;
                        BuyOrders::<T>::insert(order_id, &order);
                    }
                }

                // Clear user orders for the new epoch
                UserOrders::<T>::remove_all(None);
            }
        }

        /// Check if an account is an authorized market maker
        pub fn is_market_maker(account: &T::AccountId) -> bool {
            MarketMakers::<T>::get().contains(account)
        }

        /// Get the trading pallet's account ID
        pub fn account_id() -> T::AccountId {
            // In a real implementation, this would be derived from the pallet's module ID
            // For now, we'll use a placeholder that needs to be properly implemented
            frame_system::Pallet::<T>::account_id()
        }

        /// Get active orders for a user
        pub fn get_user_orders(user: &T::AccountId) -> Vec<OrderId> {
            UserOrders::<T>::get(user).into_inner()
        }

        /// Check if market clearing is needed
        pub fn needs_market_clearing() -> bool {
            let current_time = timestamp::Pallet::<T>::get();
            let epoch_start = CurrentEpochStart::<T>::get();
            let epoch_length = T::MarketEpochLength::get();

            current_time >= epoch_start.saturating_add(epoch_length)
        }
    }
}

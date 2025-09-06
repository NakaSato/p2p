#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// Trading smart contract for P2P Energy Trading platform
/// This contract manages the periodic order book, matches buy and sell orders,
/// and settles trades for solar energy trading.

#[ink::contract]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::arithmetic_side_effects)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::empty_line_after_outer_attr)]
mod trading {

    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    use scale::{Decode, Encode};

    /// Order ID type
    pub type OrderId = u64;

    /// Energy amount type (kWh in smallest unit)
    pub type EnergyAmount = u64;

    /// Market order structure
    #[derive(Clone, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo))]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct Order {
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
        pub created_at: u64,
        /// Order status
        pub status: OrderStatus,
        /// Amount already filled
        pub filled_amount: EnergyAmount,
    }

    /// Order type enumeration
    #[derive(Clone, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo))]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum OrderType {
        /// Sell order (prosumer selling energy)
        Sell,
        /// Buy order (consumer buying energy)
        Buy,
    }

    /// Order status enumeration
    #[derive(Clone, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo))]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
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
    #[derive(Clone, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo))]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct Trade {
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
        pub executed_at: u64,
    }

    /// Contract storage
    #[ink(storage)]
    pub struct Trading {
        /// Registry contract address for identity verification
        registry_contract: Option<AccountId>,
        /// Token contract address for payment settlement
        token_contract: Option<AccountId>,
        /// Market epoch length in milliseconds (e.g., 900000 for 15 minutes)
        market_epoch_length: u64,
        /// Current market epoch start time
        current_epoch_start: u64,
        /// Next order ID counter
        next_order_id: OrderId,
        /// Active sell orders
        sell_orders: Mapping<OrderId, Order>,
        /// Active buy orders
        buy_orders: Mapping<OrderId, Order>,
        /// Orders by user for current epoch
        user_orders: Mapping<AccountId, Vec<OrderId>>,
        /// Authorized market makers (Oracle Client, AMI Integration Service)
        market_makers: Mapping<AccountId, ()>,
        /// Trade history
        trades: Mapping<u64, Trade>,
        /// Next trade ID
        next_trade_id: u64,
        /// Maximum orders per user per epoch
        max_orders_per_user: u32,
    }

    /// Events emitted by this contract
    #[ink(event)]
    pub struct SellOrderCreated {
        #[ink(topic)]
        order_id: OrderId,
        #[ink(topic)]
        user: AccountId,
        energy_amount: EnergyAmount,
        price_per_unit: Balance,
    }

    #[ink(event)]
    pub struct BuyOrderCreated {
        #[ink(topic)]
        order_id: OrderId,
        #[ink(topic)]
        user: AccountId,
        energy_amount: EnergyAmount,
        max_price: Balance,
    }

    #[ink(event)]
    pub struct OrderCancelled {
        #[ink(topic)]
        order_id: OrderId,
        #[ink(topic)]
        user: AccountId,
    }

    #[ink(event)]
    pub struct TradeExecuted {
        #[ink(topic)]
        trade_id: u64,
        sell_order_id: OrderId,
        buy_order_id: OrderId,
        #[ink(topic)]
        seller: AccountId,
        #[ink(topic)]
        buyer: AccountId,
        energy_amount: EnergyAmount,
        price_per_unit: Balance,
        total_value: Balance,
    }

    #[ink(event)]
    pub struct MarketCleared {
        epoch_start: u64,
        trades_count: u32,
    }

    #[ink(event)]
    pub struct MarketMakerAdded {
        #[ink(topic)]
        market_maker: AccountId,
    }

    #[ink(event)]
    pub struct MarketMakerRemoved {
        #[ink(topic)]
        market_maker: AccountId,
    }

    #[ink(event)]
    pub struct ContractsConfigured {
        registry_contract: AccountId,
        token_contract: AccountId,
    }

    /// Errors that can occur in this contract
    #[derive(Encode, Decode)]
    #[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq, scale_info::TypeInfo))]
    pub enum Error {
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
        /// Only the owner can manage settings
        NotOwner,
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
        /// Registry contract not set
        RegistryNotSet,
        /// Token contract not set
        TokenNotSet,
        /// Cross-contract call failed
        CrossContractCallFailed,
    }

    /// Contract result type
    pub type Result<T> = core::result::Result<T, Error>;

    impl Trading {
        /// Constructor that initializes the trading contract
        #[ink(constructor)]
        pub fn new(market_epoch_length: u64) -> Self {
            let caller = Self::env().caller();
            let mut market_makers = Mapping::default();
            market_makers.insert(caller, &());

            Self {
                registry_contract: None,
                token_contract: None,
                market_epoch_length,
                current_epoch_start: Self::env().block_timestamp(),
                next_order_id: 1,
                sell_orders: Mapping::default(),
                buy_orders: Mapping::default(),
                user_orders: Mapping::default(),
                market_makers,
                trades: Mapping::default(),
                next_trade_id: 1,
                max_orders_per_user: 100,
            }
        }

        /// Constructor with configuration
        #[ink(constructor)]
        pub fn new_with_config(
            market_epoch_length: u64,
            max_orders_per_user: u32,
            initial_market_makers: Vec<AccountId>,
        ) -> Self {
            let caller = Self::env().caller();
            let mut market_makers = Mapping::default();

            // Add initial market makers
            for market_maker in initial_market_makers.iter() {
                market_makers.insert(market_maker, &());
            }
            // Add deployer as market maker if not already included
            market_makers.insert(caller, &());

            Self {
                registry_contract: None,
                token_contract: None,
                market_epoch_length,
                current_epoch_start: Self::env().block_timestamp(),
                next_order_id: 1,
                sell_orders: Mapping::default(),
                buy_orders: Mapping::default(),
                user_orders: Mapping::default(),
                market_makers,
                trades: Mapping::default(),
                next_trade_id: 1,
                max_orders_per_user,
            }
        }

        /// Set contract addresses for registry and token
        #[ink(message)]
        pub fn set_contracts(
            &mut self,
            registry_contract: AccountId,
            token_contract: AccountId,
        ) -> Result<()> {
            // Only owner can set contracts
            // In production, you'd have proper access control here
            self.registry_contract = Some(registry_contract);
            self.token_contract = Some(token_contract);

            self.env().emit_event(ContractsConfigured {
                registry_contract,
                token_contract,
            });

            Ok(())
        }

        /// Create a sell order for solar energy
        #[ink(message)]
        pub fn create_sell_order(
            &mut self,
            energy_amount: EnergyAmount,
            price_per_unit: Balance,
        ) -> Result<()> {
            let caller = self.env().caller();

            // Validate inputs
            if energy_amount == 0 {
                return Err(Error::InvalidEnergyAmount);
            }
            if price_per_unit == 0 {
                return Err(Error::InvalidPrice);
            }

            // Verify user is registered and is a prosumer (would need registry contract call)
            self.verify_prosumer(&caller)?;

            // Check user order limit
            let user_order_count = self
                .user_orders
                .get(caller)
                .map(|orders| orders.len())
                .unwrap_or(0);
            if user_order_count >= self.max_orders_per_user as usize {
                return Err(Error::TooManyOrders);
            }

            let order_id = self.next_order_id;
            let total_price = self.calculate_total_price(energy_amount, price_per_unit)?;
            let current_time = self.env().block_timestamp();

            let order = Order {
                id: order_id,
                user: caller,
                order_type: OrderType::Sell,
                energy_amount,
                price_per_unit,
                total_price,
                created_at: current_time,
                status: OrderStatus::Active,
                filled_amount: 0,
            };

            self.sell_orders.insert(order_id, &order);
            self.add_user_order(caller, order_id);
            self.next_order_id += 1;

            self.env().emit_event(SellOrderCreated {
                order_id,
                user: caller,
                energy_amount,
                price_per_unit,
            });

            Ok(())
        }

        /// Create a buy order for solar energy
        #[ink(message)]
        pub fn create_buy_order(
            &mut self,
            energy_amount: EnergyAmount,
            max_price: Balance,
        ) -> Result<()> {
            let caller = self.env().caller();

            // Validate inputs
            if energy_amount == 0 {
                return Err(Error::InvalidEnergyAmount);
            }
            if max_price == 0 {
                return Err(Error::InvalidPrice);
            }

            // Verify user is registered (would need registry contract call)
            self.verify_user(&caller)?;

            // Check user order limit
            let user_order_count = self
                .user_orders
                .get(caller)
                .map(|orders| orders.len())
                .unwrap_or(0);
            if user_order_count >= self.max_orders_per_user as usize {
                return Err(Error::TooManyOrders);
            }

            let order_id = self.next_order_id;
            let total_price = self.calculate_total_price(energy_amount, max_price)?;
            let current_time = self.env().block_timestamp();

            // Check that user has approved sufficient tokens for the trading contract
            // This would require a cross-contract call to the token contract
            self.verify_allowance(&caller, total_price)?;

            let order = Order {
                id: order_id,
                user: caller,
                order_type: OrderType::Buy,
                energy_amount,
                price_per_unit: max_price,
                total_price,
                created_at: current_time,
                status: OrderStatus::Active,
                filled_amount: 0,
            };

            self.buy_orders.insert(order_id, &order);
            self.add_user_order(caller, order_id);
            self.next_order_id += 1;

            self.env().emit_event(BuyOrderCreated {
                order_id,
                user: caller,
                energy_amount,
                max_price,
            });

            Ok(())
        }

        /// Cancel an active order
        #[ink(message)]
        pub fn cancel_order(&mut self, order_id: OrderId) -> Result<()> {
            let caller = self.env().caller();

            // Try to find the order in sell orders first, then buy orders
            let mut order = self
                .sell_orders
                .get(order_id)
                .or_else(|| self.buy_orders.get(order_id))
                .ok_or(Error::OrderNotFound)?;

            if order.user != caller {
                return Err(Error::NotOrderOwner);
            }

            if !matches!(
                order.status,
                OrderStatus::Active | OrderStatus::PartiallyFilled
            ) {
                return Err(Error::OrderNotActive);
            }

            order.status = OrderStatus::Cancelled;

            // Update the order in the appropriate storage
            match order.order_type {
                OrderType::Sell => {
                    self.sell_orders.insert(order_id, &order);
                }
                OrderType::Buy => {
                    self.buy_orders.insert(order_id, &order);
                }
            };

            // Remove from user orders
            self.remove_user_order(caller, order_id);

            self.env().emit_event(OrderCancelled {
                order_id,
                user: caller,
            });

            Ok(())
        }

        /// Match orders and clear the market (market maker only)
        #[ink(message)]
        pub fn match_orders(&mut self) -> Result<()> {
            let caller = self.env().caller();
            if !self.market_makers.contains(caller) {
                return Err(Error::NotMarketMaker);
            }

            let current_time = self.env().block_timestamp();
            let mut trades_count = 0u32;

            // Simple matching algorithm - in production, this would be more sophisticated
            let sell_order_ids: Vec<OrderId> = (1..self.next_order_id).collect();
            let buy_order_ids: Vec<OrderId> = (1..self.next_order_id).collect();

            for sell_order_id in sell_order_ids {
                if let Some(mut sell_order) = self.sell_orders.get(sell_order_id) {
                    if !matches!(
                        sell_order.status,
                        OrderStatus::Active | OrderStatus::PartiallyFilled
                    ) {
                        continue;
                    }

                    for buy_order_id in &buy_order_ids {
                        if let Some(mut buy_order) = self.buy_orders.get(*buy_order_id) {
                            if !matches!(
                                buy_order.status,
                                OrderStatus::Active | OrderStatus::PartiallyFilled
                            ) {
                                continue;
                            }

                            if sell_order.price_per_unit <= buy_order.price_per_unit {
                                let remaining_sell =
                                    sell_order.energy_amount - sell_order.filled_amount;
                                let remaining_buy =
                                    buy_order.energy_amount - buy_order.filled_amount;

                                if remaining_sell > 0 && remaining_buy > 0 {
                                    let trade_amount = remaining_sell.min(remaining_buy);
                                    let trade_price = sell_order.price_per_unit; // Use seller's price

                                    // Execute the trade
                                    if self
                                        .execute_trade(
                                            sell_order_id,
                                            *buy_order_id,
                                            &mut sell_order,
                                            &mut buy_order,
                                            trade_amount,
                                            trade_price,
                                            current_time,
                                        )
                                        .is_ok()
                                    {
                                        trades_count += 1;
                                    }

                                    // Update orders
                                    self.sell_orders.insert(sell_order_id, &sell_order);
                                    self.buy_orders.insert(*buy_order_id, &buy_order);
                                }
                            }
                        }
                    }
                }
            }

            // Clear expired orders and update epoch
            self.clear_expired_orders(current_time);
            self.current_epoch_start = current_time;

            self.env().emit_event(MarketCleared {
                epoch_start: current_time,
                trades_count,
            });

            Ok(())
        }

        /// Add market maker
        #[ink(message)]
        pub fn add_market_maker(&mut self, market_maker: AccountId) -> Result<()> {
            // Only existing market makers can add new ones
            let caller = self.env().caller();
            if !self.market_makers.contains(caller) {
                return Err(Error::NotOwner);
            }

            if self.market_makers.contains(market_maker) {
                return Err(Error::MarketMakerAlreadyExists);
            }

            self.market_makers.insert(market_maker, &());

            self.env().emit_event(MarketMakerAdded { market_maker });
            Ok(())
        }

        /// Remove market maker
        #[ink(message)]
        pub fn remove_market_maker(&mut self, market_maker: AccountId) -> Result<()> {
            let caller = self.env().caller();
            if !self.market_makers.contains(caller) {
                return Err(Error::NotOwner);
            }

            if !self.market_makers.contains(market_maker) {
                return Err(Error::MarketMakerNotFound);
            }

            self.market_makers.remove(market_maker);

            self.env().emit_event(MarketMakerRemoved { market_maker });
            Ok(())
        }

        /// Get order information
        #[ink(message)]
        pub fn get_order(&self, order_id: OrderId) -> Option<Order> {
            self.sell_orders
                .get(order_id)
                .or_else(|| self.buy_orders.get(order_id))
        }

        /// Get trade information
        #[ink(message)]
        pub fn get_trade(&self, trade_id: u64) -> Option<Trade> {
            self.trades.get(trade_id)
        }

        /// Get user orders
        #[ink(message)]
        pub fn get_user_orders(&self, user: AccountId) -> Vec<OrderId> {
            self.user_orders.get(user).unwrap_or_default()
        }

        /// Check if market clearing is needed
        #[ink(message)]
        pub fn needs_market_clearing(&self) -> bool {
            let current_time = self.env().block_timestamp();
            current_time >= self.current_epoch_start + self.market_epoch_length
        }

        /// Check if account is market maker
        #[ink(message)]
        pub fn is_market_maker(&self, account: AccountId) -> bool {
            self.market_makers.contains(account)
        }

        /// Get market configuration
        #[ink(message)]
        pub fn get_market_config(&self) -> (u64, u64, u32) {
            (
                self.market_epoch_length,
                self.current_epoch_start,
                self.max_orders_per_user,
            )
        }

        /// Helper functions

        fn calculate_total_price(
            &self,
            energy_amount: EnergyAmount,
            price_per_unit: Balance,
        ) -> Result<Balance> {
            let energy_balance = energy_amount as Balance;
            energy_balance
                .checked_mul(price_per_unit)
                .ok_or(Error::Overflow)
        }

        fn execute_trade(
            &mut self,
            sell_order_id: OrderId,
            buy_order_id: OrderId,
            sell_order: &mut Order,
            buy_order: &mut Order,
            trade_amount: EnergyAmount,
            price_per_unit: Balance,
            timestamp: u64,
        ) -> Result<()> {
            let total_value = self.calculate_total_price(trade_amount, price_per_unit)?;

            // Execute token transfer (would need cross-contract call to token contract)
            // For now, we'll assume the transfer succeeds

            // Update order filled amounts
            sell_order.filled_amount += trade_amount;
            buy_order.filled_amount += trade_amount;

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

            // Record the trade
            let trade_id = self.next_trade_id;
            let trade = Trade {
                sell_order_id,
                buy_order_id,
                seller: sell_order.user,
                buyer: buy_order.user,
                energy_amount: trade_amount,
                price_per_unit,
                total_value,
                executed_at: timestamp,
            };

            self.trades.insert(trade_id, &trade);
            self.next_trade_id += 1;

            self.env().emit_event(TradeExecuted {
                trade_id,
                sell_order_id,
                buy_order_id,
                seller: sell_order.user,
                buyer: buy_order.user,
                energy_amount: trade_amount,
                price_per_unit,
                total_value,
            });

            Ok(())
        }

        fn clear_expired_orders(&mut self, current_time: u64) {
            let epoch_end = self.current_epoch_start + self.market_epoch_length;

            if current_time >= epoch_end {
                // In a real implementation, you'd iterate through all orders and mark expired ones
                // This is a simplified version
                // Clear user orders for the new epoch
                // self.user_orders would need to be cleared, but ink! doesn't have clear_all
            }
        }

        fn add_user_order(&mut self, user: AccountId, order_id: OrderId) {
            let mut user_orders = self.user_orders.get(user).unwrap_or_default();
            user_orders.push(order_id);
            self.user_orders.insert(user, &user_orders);
        }

        fn remove_user_order(&mut self, user: AccountId, order_id: OrderId) {
            let mut user_orders = self.user_orders.get(user).unwrap_or_default();
            user_orders.retain(|&id| id != order_id);
            self.user_orders.insert(user, &user_orders);
        }

        fn verify_user(&self, _user: &AccountId) -> Result<()> {
            // Would make cross-contract call to registry
            // For now, assume user is verified
            Ok(())
        }

        fn verify_prosumer(&self, _user: &AccountId) -> Result<()> {
            // Would make cross-contract call to registry
            // For now, assume user is prosumer
            Ok(())
        }

        fn verify_allowance(&self, _user: &AccountId, _amount: Balance) -> Result<()> {
            // Would make cross-contract call to token contract
            // For now, assume allowance is sufficient
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn new_works() {
            let trading = Trading::new(900_000); // 15 minutes
            assert_eq!(trading.market_epoch_length, 900_000);
            assert_eq!(trading.next_order_id, 1);
        }

        #[ink::test]
        fn create_sell_order_works() {
            let mut trading = Trading::new(900_000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Alice creates sell order
            assert!(trading
                .create_sell_order(100, 150_000_000_000_000_000_000u128) // 100 kWh at 150 GRID
                .is_ok());

            let order = trading.get_order(1).unwrap();
            assert_eq!(order.user, accounts.alice);
            assert_eq!(order.energy_amount, 100);
            assert_eq!(order.order_type, OrderType::Sell);
        }

        #[ink::test]
        fn create_buy_order_works() {
            let mut trading = Trading::new(900_000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Alice creates buy order
            assert!(trading
                .create_buy_order(50, 160_000_000_000_000_000_000u128) // 50 kWh at max 160 GRID
                .is_ok());

            let order = trading.get_order(1).unwrap();
            assert_eq!(order.user, accounts.alice);
            assert_eq!(order.energy_amount, 50);
            assert_eq!(order.order_type, OrderType::Buy);
        }

        #[ink::test]
        fn cancel_order_works() {
            let mut trading = Trading::new(900_000);

            // Create and cancel order
            trading
                .create_sell_order(100, 150_000_000_000_000_000_000u128)
                .unwrap();
            assert!(trading.cancel_order(1).is_ok());

            let order = trading.get_order(1).unwrap();
            assert_eq!(order.status, OrderStatus::Cancelled);
        }

        #[ink::test]
        fn add_market_maker_works() {
            let mut trading = Trading::new(900_000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Alice adds Bob as market maker
            assert!(trading.add_market_maker(accounts.bob).is_ok());
            assert!(trading.is_market_maker(accounts.bob));
        }
    }
}

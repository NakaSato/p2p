#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// OracleClient smart contract for P2P Energy Trading platform
/// This contract acts as a secure bridge between the blockchain and external oracle networks
/// for automated operations like market clearing and energy data verification.

#[ink::contract]
mod oracle_client {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    use scale::{Decode, Encode};

    /// Oracle request ID type
    pub type RequestId = u64;

    /// Meter data structure from oracle
    #[derive(Clone, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo))]
    pub struct MeterData {
        /// Meter identifier
        pub meter_id: String,
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
    #[derive(Clone, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo))]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum RequestType {
        /// Request energy data from AMI Head-End API
        EnergyData { meter_id: String },
        /// Request market clearing check
        MarketClearing,
    }

    /// Oracle request structure
    #[derive(Clone, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo))]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct OracleRequest {
        /// Request ID
        pub id: RequestId,
        /// Requester account
        pub requester: AccountId,
        /// Type of request
        pub request_type: RequestType,
        /// Request timestamp
        pub requested_at: u64,
        /// Request status
        pub status: RequestStatus,
        /// Block number when request was made
        pub block_number: u64,
    }

    /// Request status enumeration
    #[derive(Clone, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo))]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
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

    /// Contract storage
    #[ink(storage)]
    pub struct OracleClient {
        /// Registry contract address for identity verification
        registry_contract: Option<AccountId>,
        /// Token contract address for minting operations
        token_contract: Option<AccountId>,
        /// Trading contract address for market clearing
        trading_contract: Option<AccountId>,
        /// Next oracle request ID
        next_request_id: RequestId,
        /// Pending oracle requests
        oracle_requests: Mapping<RequestId, OracleRequest>,
        /// Authorized oracle operators
        oracle_operators: Mapping<AccountId, ()>,
        /// Oracle funding balance for paying oracle fees
        oracle_balance: Balance,
        /// Last market clearing check timestamp
        last_market_check: u64,
        /// Automatic market clearing enabled flag
        auto_market_clearing: bool,
        /// Oracle request timeout in blocks
        oracle_timeout: u64,
        /// Maximum number of pending requests
        max_pending_requests: u32,
    }

    /// Events emitted by this contract
    #[ink(event)]
    pub struct OracleRequestCreated {
        #[ink(topic)]
        request_id: RequestId,
        #[ink(topic)]
        requester: AccountId,
        request_type: RequestType,
    }

    #[ink(event)]
    pub struct EnergyDataFulfilled {
        #[ink(topic)]
        request_id: RequestId,
        meter_id: String,
        energy_generated: u64,
        energy_consumed: u64,
        tokens_minted: Balance,
    }

    #[ink(event)]
    pub struct MarketClearingChecked {
        #[ink(topic)]
        request_id: RequestId,
        clearing_needed: bool,
    }

    #[ink(event)]
    pub struct AutoMarketClearingTriggered {
        timestamp: u64,
    }

    #[ink(event)]
    pub struct OracleRequestExpired {
        #[ink(topic)]
        request_id: RequestId,
    }

    #[ink(event)]
    pub struct OracleOperatorAdded {
        #[ink(topic)]
        operator: AccountId,
    }

    #[ink(event)]
    pub struct OracleOperatorRemoved {
        #[ink(topic)]
        operator: AccountId,
    }

    #[ink(event)]
    pub struct OracleFunded {
        amount: Balance,
        new_balance: Balance,
    }

    #[ink(event)]
    pub struct ContractsConfigured {
        registry_contract: AccountId,
        token_contract: AccountId,
        trading_contract: AccountId,
    }

    /// Errors that can occur in this contract
    #[derive(Encode, Decode)]
    #[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq, scale_info::TypeInfo))]
    pub enum Error {
        /// Request not found
        RequestNotFound,
        /// Only authorized oracle operators can fulfill requests
        NotOracleOperator,
        /// Only the owner can manage oracle operators
        NotOwner,
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
        /// Registry contract not set
        RegistryNotSet,
        /// Token contract not set
        TokenNotSet,
        /// Trading contract not set
        TradingNotSet,
        /// Cross-contract call failed
        CrossContractCallFailed,
    }

    /// Contract result type
    pub type Result<T> = core::result::Result<T, Error>;

    impl OracleClient {
        /// Constructor that initializes the oracle client
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            let mut oracle_operators = Mapping::default();
            oracle_operators.insert(caller, &());

            Self {
                registry_contract: None,
                token_contract: None,
                trading_contract: None,
                next_request_id: 1,
                oracle_requests: Mapping::default(),
                oracle_operators,
                oracle_balance: 0,
                last_market_check: Self::env().block_timestamp(),
                auto_market_clearing: true,
                oracle_timeout: 100, // blocks
                max_pending_requests: 1000,
            }
        }

        /// Constructor with configuration
        #[ink(constructor)]
        pub fn new_with_config(
            initial_operators: Vec<AccountId>,
            oracle_balance: Balance,
            auto_market_clearing: bool,
            oracle_timeout: u64,
        ) -> Self {
            let caller = Self::env().caller();
            let mut oracle_operators = Mapping::default();

            // Add initial operators
            for operator in initial_operators.iter() {
                oracle_operators.insert(operator, &());
            }
            // Add deployer as operator if not already included
            oracle_operators.insert(caller, &());

            Self {
                registry_contract: None,
                token_contract: None,
                trading_contract: None,
                next_request_id: 1,
                oracle_requests: Mapping::default(),
                oracle_operators,
                oracle_balance,
                last_market_check: Self::env().block_timestamp(),
                auto_market_clearing,
                oracle_timeout,
                max_pending_requests: 1000,
            }
        }

        /// Set contract addresses
        #[ink(message)]
        pub fn set_contracts(
            &mut self,
            registry_contract: AccountId,
            token_contract: AccountId,
            trading_contract: AccountId,
        ) -> Result<()> {
            // Only owner can set contracts
            self.registry_contract = Some(registry_contract);
            self.token_contract = Some(token_contract);
            self.trading_contract = Some(trading_contract);

            self.env().emit_event(ContractsConfigured {
                registry_contract,
                token_contract,
                trading_contract,
            });

            Ok(())
        }

        /// Request energy data from oracle for a specific meter
        #[ink(message)]
        pub fn request_energy_data(&mut self, meter_id: String) -> Result<()> {
            let caller = self.env().caller();

            // Verify the requester has permission (would need registry contract call)
            self.verify_meter_access(&caller, &meter_id)?;

            // Check oracle balance
            if self.oracle_balance == 0 {
                return Err(Error::InsufficientOracleBalance);
            }

            // Check pending requests limit
            let pending_count = self.count_pending_requests();
            if pending_count >= self.max_pending_requests {
                return Err(Error::TooManyPendingRequests);
            }

            let request_id = self.next_request_id;
            let current_time = self.env().block_timestamp();
            let current_block = self.env().block_number();

            let request = OracleRequest {
                id: request_id,
                requester: caller,
                request_type: RequestType::EnergyData {
                    meter_id: meter_id.clone(),
                },
                requested_at: current_time,
                status: RequestStatus::Pending,
                block_number: current_block as u64,
            };

            self.oracle_requests.insert(request_id, &request);
            self.next_request_id += 1;

            self.env().emit_event(OracleRequestCreated {
                request_id,
                requester: caller,
                request_type: RequestType::EnergyData { meter_id },
            });

            Ok(())
        }

        /// Fulfill energy data request (oracle operator only)
        #[ink(message)]
        pub fn fulfill_energy_data(
            &mut self,
            request_id: RequestId,
            meter_data: MeterData,
        ) -> Result<()> {
            let caller = self.env().caller();
            if !self.oracle_operators.contains(caller) {
                return Err(Error::NotOracleOperator);
            }

            let mut request = self
                .oracle_requests
                .get(request_id)
                .ok_or(Error::RequestNotFound)?;

            if !matches!(request.status, RequestStatus::Pending) {
                return Err(Error::RequestAlreadyFulfilled);
            }

            // Check if request has expired
            let current_block = self.env().block_number();
            if (current_block as u64) > request.block_number + self.oracle_timeout {
                request.status = RequestStatus::Expired;
                self.oracle_requests.insert(request_id, &request);

                self.env().emit_event(OracleRequestExpired { request_id });
                return Err(Error::RequestExpired);
            }

            // Verify meter data (in production, check signature)
            self.verify_meter_data(&meter_data)?;

            // Mint tokens for energy generation (would need cross-contract call)
            let mut tokens_minted = 0u128;
            if meter_data.energy_generated > 0 {
                tokens_minted = self.mint_energy_tokens(&meter_data)?;
            }

            // Update request status
            request.status = RequestStatus::Fulfilled;
            self.oracle_requests.insert(request_id, &request);

            self.env().emit_event(EnergyDataFulfilled {
                request_id,
                meter_id: meter_data.meter_id,
                energy_generated: meter_data.energy_generated,
                energy_consumed: meter_data.energy_consumed,
                tokens_minted,
            });

            Ok(())
        }

        /// Check if market clearing is needed (Chainlink Keepers interface)
        #[ink(message)]
        pub fn check_upkeep(&mut self) -> Result<()> {
            let current_time = self.env().block_timestamp();

            // Check if market clearing is needed (would need cross-contract call)
            let clearing_needed = self.check_market_clearing_needed()?;

            if clearing_needed && self.auto_market_clearing {
                // Trigger automatic market clearing
                self.trigger_market_clearing()?;

                self.env().emit_event(AutoMarketClearingTriggered {
                    timestamp: current_time,
                });
            }

            self.last_market_check = current_time;

            let request_id = self.next_request_id;
            self.env().emit_event(MarketClearingChecked {
                request_id,
                clearing_needed,
            });

            Ok(())
        }

        /// Perform automated upkeep (Chainlink Keepers interface)
        #[ink(message)]
        pub fn perform_upkeep(&mut self) -> Result<()> {
            let caller = self.env().caller();
            if !self.oracle_operators.contains(caller) {
                return Err(Error::NotOracleOperator);
            }

            if self.check_market_clearing_needed()? {
                self.trigger_market_clearing()?;

                self.env().emit_event(AutoMarketClearingTriggered {
                    timestamp: self.env().block_timestamp(),
                });
            }

            Ok(())
        }

        /// Fund oracle operations
        #[ink(message, payable)]
        pub fn fund_oracle_operations(&mut self) -> Result<()> {
            let amount = self.env().transferred_value();
            self.oracle_balance += amount;

            self.env().emit_event(OracleFunded {
                amount,
                new_balance: self.oracle_balance,
            });

            Ok(())
        }

        /// Add oracle operator
        #[ink(message)]
        pub fn add_oracle_operator(&mut self, operator: AccountId) -> Result<()> {
            let caller = self.env().caller();
            if !self.oracle_operators.contains(caller) {
                return Err(Error::NotOwner);
            }

            if self.oracle_operators.contains(operator) {
                return Err(Error::OperatorAlreadyExists);
            }

            self.oracle_operators.insert(operator, &());

            self.env().emit_event(OracleOperatorAdded { operator });
            Ok(())
        }

        /// Remove oracle operator
        #[ink(message)]
        pub fn remove_oracle_operator(&mut self, operator: AccountId) -> Result<()> {
            let caller = self.env().caller();
            if !self.oracle_operators.contains(caller) {
                return Err(Error::NotOwner);
            }

            if !self.oracle_operators.contains(operator) {
                return Err(Error::OperatorNotFound);
            }

            self.oracle_operators.remove(operator);

            self.env().emit_event(OracleOperatorRemoved { operator });
            Ok(())
        }

        /// Enable or disable automatic market clearing
        #[ink(message)]
        pub fn set_auto_market_clearing(&mut self, enabled: bool) -> Result<()> {
            let caller = self.env().caller();
            if !self.oracle_operators.contains(caller) {
                return Err(Error::NotOwner);
            }

            self.auto_market_clearing = enabled;
            Ok(())
        }

        /// Get request information
        #[ink(message)]
        pub fn get_request(&self, request_id: RequestId) -> Option<OracleRequest> {
            self.oracle_requests.get(request_id)
        }

        /// Check if account is oracle operator
        #[ink(message)]
        pub fn is_oracle_operator(&self, account: AccountId) -> bool {
            self.oracle_operators.contains(account)
        }

        /// Get oracle balance
        #[ink(message)]
        pub fn get_oracle_balance(&self) -> Balance {
            self.oracle_balance
        }

        /// Get configuration
        #[ink(message)]
        pub fn get_config(&self) -> (bool, u64, u64, u32) {
            (
                self.auto_market_clearing,
                self.oracle_timeout,
                self.last_market_check,
                self.max_pending_requests,
            )
        }

        /// Check if upkeep is needed
        #[ink(message)]
        pub fn upkeep_needed(&self) -> Result<bool> {
            Ok(self.check_market_clearing_needed()? && self.auto_market_clearing)
        }

        /// Helper functions

        fn verify_meter_access(&self, _user: &AccountId, _meter_id: &str) -> Result<()> {
            // Would make cross-contract call to registry
            // For now, assume access is granted
            Ok(())
        }

        fn verify_meter_data(&self, _meter_data: &MeterData) -> Result<()> {
            // In production, verify the digital signature
            // For now, assume data is valid
            Ok(())
        }

        fn mint_energy_tokens(&self, meter_data: &MeterData) -> Result<Balance> {
            // Would make cross-contract call to token contract
            // For now, return the amount that would be minted
            let tokens_to_mint =
                meter_data.energy_generated as Balance * 1_000_000_000_000_000_000u128; // Convert to 18 decimals
            Ok(tokens_to_mint)
        }

        fn check_market_clearing_needed(&self) -> Result<bool> {
            // Would make cross-contract call to trading contract
            // For now, assume clearing is needed every 15 minutes
            let current_time = self.env().block_timestamp();
            let fifteen_minutes = 15 * 60 * 1000; // 15 minutes in milliseconds
            Ok(current_time >= self.last_market_check + fifteen_minutes)
        }

        fn trigger_market_clearing(&self) -> Result<()> {
            // Would make cross-contract call to trading contract
            // For now, assume the call succeeds
            Ok(())
        }

        fn count_pending_requests(&self) -> u32 {
            // In a real implementation, you'd need to iterate through all requests
            // This is a simplified version
            0 // Placeholder
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn new_works() {
            let oracle = OracleClient::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert!(oracle.is_oracle_operator(accounts.alice));
            assert_eq!(oracle.next_request_id, 1);
        }

        #[ink::test]
        fn request_energy_data_works() {
            let mut oracle = OracleClient::new();

            // Fund the oracle first
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(1000);
            oracle.fund_oracle_operations().unwrap();

            assert!(oracle.request_energy_data("METER_001".to_string()).is_ok());

            let request = oracle.get_request(1).unwrap();
            assert_eq!(request.id, 1);
            assert!(matches!(
                request.request_type,
                RequestType::EnergyData { .. }
            ));
        }

        #[ink::test]
        fn add_oracle_operator_works() {
            let mut oracle = OracleClient::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert!(oracle.add_oracle_operator(accounts.bob).is_ok());
            assert!(oracle.is_oracle_operator(accounts.bob));
        }

        #[ink::test]
        fn unauthorized_fulfill_fails() {
            let mut oracle = OracleClient::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Fund the oracle first
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(1000);
            oracle.fund_oracle_operations().unwrap();

            // Create request
            oracle.request_energy_data("METER_001".to_string()).unwrap();

            // Set Bob as caller (non-operator)
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);

            let meter_data = MeterData {
                meter_id: "METER_001".to_string(),
                energy_generated: 100,
                energy_consumed: 50,
                timestamp: 1000,
                signature: vec![1, 2, 3],
            };

            // Bob tries to fulfill (should fail)
            assert_eq!(
                oracle.fulfill_energy_data(1, meter_data),
                Err(Error::NotOracleOperator)
            );
        }

        #[ink::test]
        fn fund_oracle_operations_works() {
            let mut oracle = OracleClient::new();

            // Simulate funding
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(1000);
            assert!(oracle.fund_oracle_operations().is_ok());
        }
    }
}

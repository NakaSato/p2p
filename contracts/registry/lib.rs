#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// Registry smart contract for P2P Energy Trading platform
/// This contract manages the registration and verification of all participants
/// and their associated smart meters within the university campus.

#[ink::contract]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::new_without_default)]
mod registry {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    use scale::{Decode, Encode};

    /// User information stored in the registry
    #[derive(Clone, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo))]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct UserInfo {
        /// User type: Prosumer (can generate and consume) or Consumer (only consume)
        pub user_type: UserType,
        /// Campus location identifier
        pub location: String,
        /// User status
        pub status: UserStatus,
        /// Registration timestamp
        pub registered_at: u64,
    }

    /// Types of users in the system
    #[derive(Clone, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo))]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum UserType {
        /// Can both generate and consume energy
        Prosumer,
        /// Can only consume energy
        Consumer,
    }

    /// User status in the system
    #[derive(Clone, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo))]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum UserStatus {
        /// User is active and can participate in trading
        Active,
        /// User is temporarily suspended
        Suspended,
        /// User is permanently deactivated
        Deactivated,
    }

    /// Meter identifier type
    pub type MeterId = String;

    /// Contract storage
    #[ink(storage)]
    pub struct Registry {
        /// REC Regulators
        rec_regulators: Mapping<AccountId, ()>,
        /// Registered users
        users: Mapping<AccountId, UserInfo>,
        /// Mapping meter IDs to their owners
        meter_owners: Mapping<MeterId, AccountId>,
        /// Mapping users to their assigned meters
        user_meters: Mapping<AccountId, Vec<MeterId>>,
        /// Maximum number of meters per user
        max_meters_per_user: u32,
    }

    /// Events emitted by this contract
    #[ink(event)]
    pub struct RecRegulatorAdded {
        #[ink(topic)]
        rec_regulator: AccountId,
    }

    #[ink(event)]
    pub struct RecRegulatorRemoved {
        #[ink(topic)]
        rec_regulator: AccountId,
    }

    #[ink(event)]
    pub struct UserRegistered {
        #[ink(topic)]
        user: AccountId,
        user_type: UserType,
        location: String,
    }

    #[ink(event)]
    pub struct UserStatusUpdated {
        #[ink(topic)]
        user: AccountId,
        old_status: UserStatus,
        new_status: UserStatus,
    }

    #[ink(event)]
    pub struct MeterAssigned {
        #[ink(topic)]
        meter_id: MeterId,
        #[ink(topic)]
        owner: AccountId,
    }

    #[ink(event)]
    pub struct MeterUnassigned {
        #[ink(topic)]
        meter_id: MeterId,
        #[ink(topic)]
        former_owner: AccountId,
    }

    /// Errors that can occur in this contract
    #[derive(Encode, Decode)]
    #[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq, scale_info::TypeInfo))]
    pub enum Error {
        /// The account is not a REC regulator
        NotRecRegulator,
        /// The user is already registered
        UserAlreadyExists,
        /// The user is not registered
        UserNotFound,
        /// The meter is already assigned
        MeterAlreadyAssigned,
        /// The meter is not assigned to any user
        MeterNotFound,
        /// User has reached maximum number of meters
        TooManyMeters,
        /// Cannot remove the last REC regulator
        CannotRemoveLastRecRegulator,
        /// REC regulator already exists
        RecRegulatorAlreadyExists,
        /// REC regulator not found
        RecRegulatorNotFound,
    }

    /// Contract result type
    pub type Result<T> = core::result::Result<T, Error>;

    impl Registry {
        /// Constructor that initializes the registry with the deployer as REC regulator
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            let mut rec_regulators = Mapping::default();
            rec_regulators.insert(caller, &());

            Self {
                rec_regulators,
                users: Mapping::default(),
                meter_owners: Mapping::default(),
                user_meters: Mapping::default(),
                max_meters_per_user: 10,
            }
        }

        /// Constructor with initial configuration
        #[ink(constructor)]
        pub fn new_with_config(initial_rec_regulators: Vec<AccountId>, max_meters_per_user: u32) -> Self {
            let mut rec_regulators = Mapping::default();
            let caller = Self::env().caller();

            // Add initial REC regulators
            for rec_regulator in initial_rec_regulators.iter() {
                rec_regulators.insert(rec_regulator, &());
            }

            // Add deployer as REC regulator if not already included
            rec_regulators.insert(caller, &());

            Self {
                rec_regulators,
                users: Mapping::default(),
                meter_owners: Mapping::default(),
                user_meters: Mapping::default(),
                max_meters_per_user,
            }
        }

        /// Add a new REC regulator to the system
        #[ink(message)]
        pub fn add_rec_regulator(&mut self, new_rec_regulator: AccountId) -> Result<()> {
            let caller = self.env().caller();
            self.ensure_rec_regulator(&caller)?;

            if self.rec_regulators.contains(new_rec_regulator) {
                return Err(Error::RecRegulatorAlreadyExists);
            }

            self.rec_regulators.insert(new_rec_regulator, &());
            self.env().emit_event(RecRegulatorAdded { rec_regulator: new_rec_regulator });

            Ok(())
        }

        /// Remove a REC regulator from the system
        #[ink(message)]
        pub fn remove_rec_regulator(&mut self, rec_regulator_to_remove: AccountId) -> Result<()> {
            let caller = self.env().caller();
            self.ensure_rec_regulator(&caller)?;

            if !self.rec_regulators.contains(rec_regulator_to_remove) {
                return Err(Error::RecRegulatorNotFound);
            }

            // Count remaining REC regulators
            let rec_regulator_count = self.count_rec_regulators();
            if rec_regulator_count <= 1 {
                return Err(Error::CannotRemoveLastRecRegulator);
            }

            self.rec_regulators.remove(rec_regulator_to_remove);
            self.env().emit_event(RecRegulatorRemoved {
                rec_regulator: rec_regulator_to_remove,
            });

            Ok(())
        }

        /// Register a new user in the system
        #[ink(message)]
        pub fn register_user(
            &mut self,
            user_account: AccountId,
            user_type: UserType,
            location: String,
        ) -> Result<()> {
            let caller = self.env().caller();
            self.ensure_rec_regulator(&caller)?;

            if self.users.contains(user_account) {
                return Err(Error::UserAlreadyExists);
            }

            let user_info = UserInfo {
                user_type: user_type.clone(),
                location: location.clone(),
                status: UserStatus::Active,
                registered_at: self.env().block_timestamp(),
            };

            self.users.insert(user_account, &user_info);
            self.user_meters
                .insert(user_account, &Vec::<MeterId>::new());

            self.env().emit_event(UserRegistered {
                user: user_account,
                user_type,
                location,
            });

            Ok(())
        }

        /// Update user status
        #[ink(message)]
        pub fn update_user_status(
            &mut self,
            user_account: AccountId,
            new_status: UserStatus,
        ) -> Result<()> {
            let caller = self.env().caller();
            self.ensure_rec_regulator(&caller)?;

            let mut user_info = self.users.get(user_account).ok_or(Error::UserNotFound)?;
            let old_status = user_info.status.clone();
            user_info.status = new_status.clone();

            self.users.insert(user_account, &user_info);

            self.env().emit_event(UserStatusUpdated {
                user: user_account,
                old_status,
                new_status,
            });

            Ok(())
        }

        /// Assign a smart meter to a user
        #[ink(message)]
        pub fn assign_meter(&mut self, meter_id: MeterId, owner: AccountId) -> Result<()> {
            let caller = self.env().caller();
            self.ensure_rec_regulator(&caller)?;

            // Check if user exists
            if !self.users.contains(owner) {
                return Err(Error::UserNotFound);
            }

            // Check if meter is already assigned
            if self.meter_owners.contains(&meter_id) {
                return Err(Error::MeterAlreadyAssigned);
            }

            // Check user's meter limit
            let mut user_meters = self.user_meters.get(owner).unwrap_or_default();
            if user_meters.len() >= self.max_meters_per_user as usize {
                return Err(Error::TooManyMeters);
            }

            // Assign meter
            self.meter_owners.insert(&meter_id, &owner);
            user_meters.push(meter_id.clone());
            self.user_meters.insert(owner, &user_meters);

            self.env().emit_event(MeterAssigned { meter_id, owner });

            Ok(())
        }

        /// Unassign a smart meter from a user
        #[ink(message)]
        pub fn unassign_meter(&mut self, meter_id: MeterId) -> Result<()> {
            let caller = self.env().caller();
            self.ensure_rec_regulator(&caller)?;

            let former_owner = self
                .meter_owners
                .get(&meter_id)
                .ok_or(Error::MeterNotFound)?;

            // Remove from meter owners
            self.meter_owners.remove(&meter_id);

            // Remove from user's meter list
            let mut user_meters = self.user_meters.get(former_owner).unwrap_or_default();
            user_meters.retain(|m| m != &meter_id);
            self.user_meters.insert(former_owner, &user_meters);

            self.env().emit_event(MeterUnassigned {
                meter_id,
                former_owner,
            });

            Ok(())
        }

        /// Check if an account is a REC regulator
        #[ink(message)]
        pub fn is_rec_regulator(&self, account: AccountId) -> bool {
            self.rec_regulators.contains(account)
        }

        /// Check if a user is verified (registered and active)
        #[ink(message)]
        pub fn is_user_verified(&self, user_account: AccountId) -> bool {
            if let Some(user_info) = self.users.get(user_account) {
                matches!(user_info.status, UserStatus::Active)
            } else {
                false
            }
        }

        /// Get the owner of a meter
        #[ink(message)]
        pub fn get_meter_owner(&self, meter_id: MeterId) -> Option<AccountId> {
            self.meter_owners.get(&meter_id)
        }

        /// Check if user is a prosumer
        #[ink(message)]
        pub fn is_prosumer(&self, user_account: AccountId) -> bool {
            if let Some(user_info) = self.users.get(user_account) {
                matches!(user_info.user_type, UserType::Prosumer)
            } else {
                false
            }
        }

        /// Get all meters owned by a user
        #[ink(message)]
        pub fn get_user_meters(&self, user_account: AccountId) -> Vec<MeterId> {
            self.user_meters.get(user_account).unwrap_or_default()
        }

        /// Get user information
        #[ink(message)]
        pub fn get_user_info(&self, user_account: AccountId) -> Option<UserInfo> {
            self.users.get(user_account)
        }

        /// Get maximum meters per user setting
        #[ink(message)]
        pub fn get_max_meters_per_user(&self) -> u32 {
            self.max_meters_per_user
        }

        /// Update maximum meters per user (REC regulator only)
        #[ink(message)]
        pub fn set_max_meters_per_user(&mut self, max_meters: u32) -> Result<()> {
            let caller = self.env().caller();
            self.ensure_rec_regulator(&caller)?;

            self.max_meters_per_user = max_meters;
            Ok(())
        }

        /// Helper function to ensure caller is REC regulator
        fn ensure_rec_regulator(&self, account: &AccountId) -> Result<()> {
            if self.rec_regulators.contains(account) {
                Ok(())
            } else {
                Err(Error::NotRecRegulator)
            }
        }

        /// Count total number of REC regulators
        fn count_rec_regulators(&self) -> u32 {
            // Note: In a real implementation, you'd need to iterate through all REC regulators
            // This is a simplified version for the example
            // You might want to maintain a separate counter for efficiency
            1 // Placeholder - in real implementation, iterate through mapping
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn new_works() {
            let registry = Registry::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Deployer should be REC regulator
            assert!(registry.is_rec_regulator(accounts.alice));
        }

        #[ink::test]
        fn add_rec_regulator_works() {
            let mut registry = Registry::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Alice adds Bob as REC regulator
            assert!(registry.add_rec_regulator(accounts.bob).is_ok());
            assert!(registry.is_rec_regulator(accounts.bob));
        }

        #[ink::test]
        fn register_user_works() {
            let mut registry = Registry::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Register Bob as prosumer
            assert!(registry
                .register_user(accounts.bob, UserType::Prosumer, "Building A".to_string())
                .is_ok());

            assert!(registry.is_user_verified(accounts.bob));
            assert!(registry.is_prosumer(accounts.bob));
        }

        #[ink::test]
        fn assign_meter_works() {
            let mut registry = Registry::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // First register user
            registry
                .register_user(accounts.bob, UserType::Prosumer, "Building A".to_string())
                .unwrap();

            // Assign meter
            let meter_id = "METER_001".to_string();
            assert!(registry
                .assign_meter(meter_id.clone(), accounts.bob)
                .is_ok());
            assert_eq!(
                registry.get_meter_owner(meter_id.clone()),
                Some(accounts.bob)
            );

            let user_meters = registry.get_user_meters(accounts.bob);
            assert_eq!(user_meters, vec![meter_id]);
        }

        #[ink::test]
        fn non_rec_regulator_cannot_register_user() {
            let mut registry = Registry::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Set Bob as caller (non-REC regulator)
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);

            // Bob tries to register Charlie (should fail)
            assert_eq!(
                registry.register_user(
                    accounts.charlie,
                    UserType::Consumer,
                    "Building B".to_string()
                ),
                Err(Error::NotRecRegulator)
            );
        }

        #[ink::test]
        fn cannot_assign_meter_to_unregistered_user() {
            let mut registry = Registry::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Try to assign meter to unregistered user
            assert_eq!(
                registry.assign_meter("METER_001".to_string(), accounts.bob),
                Err(Error::UserNotFound)
            );
        }
    }
}

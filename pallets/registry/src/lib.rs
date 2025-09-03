#![cfg_attr(not(feature = "std"), no_std)]

/// Registry pallet for P2P Energy Trading platform
/// This pallet manages the registration and verification of all participants
/// and their associated smart meters within the university campus.
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
    use sp_std::{collections::btree_set::BTreeSet, vec::Vec};

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Maximum number of meters per user
        #[pallet::constant]
        type MaxMetersPerUser: Get<u32>;
    }

    /// User information stored in the registry
    #[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
    pub struct UserInfo {
        /// User type: Prosumer (can generate and consume) or Consumer (only consume)
        pub user_type: UserType,
        /// Campus location identifier
        pub location: Vec<u8>,
        /// User status
        pub status: UserStatus,
        /// Registration timestamp
        pub registered_at: u64,
    }

    /// Types of users in the system
    #[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
    pub enum UserType {
        /// Can both generate and consume energy
        Prosumer,
        /// Can only consume energy
        Consumer,
    }

    /// User status in the system
    #[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
    pub enum UserStatus {
        /// User is active and can participate in trading
        Active,
        /// User is temporarily suspended
        Suspended,
        /// User is permanently deactivated
        Deactivated,
    }

    /// Meter identifier type
    pub type MeterId = Vec<u8>;

    /// Storage for system administrators
    #[pallet::storage]
    #[pallet::getter(fn admins)]
    pub type Admins<T: Config> = StorageValue<_, BTreeSet<T::AccountId>, ValueQuery>;

    /// Storage for registered users
    #[pallet::storage]
    #[pallet::getter(fn users)]
    pub type Users<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, UserInfo, OptionQuery>;

    /// Storage mapping meter IDs to their owners
    #[pallet::storage]
    #[pallet::getter(fn meter_owners)]
    pub type MeterOwners<T: Config> =
        StorageMap<_, Blake2_128Concat, MeterId, T::AccountId, OptionQuery>;

    /// Storage mapping users to their assigned meters
    #[pallet::storage]
    #[pallet::getter(fn user_meters)]
    pub type UserMeters<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<MeterId, T::MaxMetersPerUser>,
        ValueQuery,
    >;

    /// Genesis configuration
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        /// Initial administrators
        pub admins: Vec<T::AccountId>,
        /// Initial users
        pub users: Vec<(T::AccountId, UserInfo)>,
        /// Initial meter assignments
        pub meters: Vec<(MeterId, T::AccountId)>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                admins: Default::default(),
                users: Default::default(),
                meters: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            let mut admin_set = BTreeSet::new();
            for admin in &self.admins {
                admin_set.insert(admin.clone());
            }
            Admins::<T>::put(admin_set);

            for (account, user_info) in &self.users {
                Users::<T>::insert(account, user_info);
            }

            for (meter_id, owner) in &self.meters {
                MeterOwners::<T>::insert(meter_id, owner);
                UserMeters::<T>::try_mutate(owner, |meters| meters.try_push(meter_id.clone()))
                    .expect("Failed to assign meter in genesis");
            }
        }
    }

    /// Events emitted by this pallet
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Admin added to the system
        AdminAdded { admin: T::AccountId },
        /// Admin removed from the system
        AdminRemoved { admin: T::AccountId },
        /// User registered in the system
        UserRegistered {
            user: T::AccountId,
            user_type: UserType,
            location: Vec<u8>,
        },
        /// User status updated
        UserStatusUpdated {
            user: T::AccountId,
            old_status: UserStatus,
            new_status: UserStatus,
        },
        /// Meter assigned to user
        MeterAssigned {
            meter_id: MeterId,
            owner: T::AccountId,
        },
        /// Meter unassigned from user
        MeterUnassigned {
            meter_id: MeterId,
            former_owner: T::AccountId,
        },
    }

    /// Errors that can occur in this pallet
    #[pallet::error]
    pub enum Error<T> {
        /// The account is not an administrator
        NotAdmin,
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
        /// Cannot remove the last admin
        CannotRemoveLastAdmin,
        /// Admin already exists
        AdminAlreadyExists,
        /// Admin not found
        AdminNotFound,
    }

    /// Dispatchable functions
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Add a new administrator to the system
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn add_admin(origin: OriginFor<T>, new_admin: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_admin(&who), Error::<T>::NotAdmin);

            Admins::<T>::try_mutate(|admins| {
                ensure!(!admins.contains(&new_admin), Error::<T>::AdminAlreadyExists);
                admins.insert(new_admin.clone());
                Ok(())
            })?;

            Self::deposit_event(Event::AdminAdded { admin: new_admin });
            Ok(())
        }

        /// Remove an administrator from the system
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn remove_admin(origin: OriginFor<T>, admin_to_remove: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_admin(&who), Error::<T>::NotAdmin);

            Admins::<T>::try_mutate(|admins| {
                ensure!(admins.len() > 1, Error::<T>::CannotRemoveLastAdmin);
                ensure!(admins.contains(&admin_to_remove), Error::<T>::AdminNotFound);
                admins.remove(&admin_to_remove);
                Ok(())
            })?;

            Self::deposit_event(Event::AdminRemoved {
                admin: admin_to_remove,
            });
            Ok(())
        }

        /// Register a new user in the system
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn register_user(
            origin: OriginFor<T>,
            user_account: T::AccountId,
            user_type: UserType,
            location: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_admin(&who), Error::<T>::NotAdmin);
            ensure!(
                !Users::<T>::contains_key(&user_account),
                Error::<T>::UserAlreadyExists
            );

            let user_info = UserInfo {
                user_type: user_type.clone(),
                location: location.clone(),
                status: UserStatus::Active,
                registered_at: Self::current_timestamp(),
            };

            Users::<T>::insert(&user_account, &user_info);

            Self::deposit_event(Event::UserRegistered {
                user: user_account,
                user_type,
                location,
            });

            Ok(())
        }

        /// Update user status
        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn update_user_status(
            origin: OriginFor<T>,
            user_account: T::AccountId,
            new_status: UserStatus,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_admin(&who), Error::<T>::NotAdmin);

            Users::<T>::try_mutate(&user_account, |user_info| {
                let user_info = user_info.as_mut().ok_or(Error::<T>::UserNotFound)?;
                let old_status = user_info.status.clone();
                user_info.status = new_status.clone();

                Self::deposit_event(Event::UserStatusUpdated {
                    user: user_account.clone(),
                    old_status,
                    new_status,
                });

                Ok(())
            })
        }

        /// Assign a smart meter to a user
        #[pallet::call_index(4)]
        #[pallet::weight(10_000)]
        pub fn assign_meter(
            origin: OriginFor<T>,
            meter_id: MeterId,
            owner: T::AccountId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_admin(&who), Error::<T>::NotAdmin);
            ensure!(Users::<T>::contains_key(&owner), Error::<T>::UserNotFound);
            ensure!(
                !MeterOwners::<T>::contains_key(&meter_id),
                Error::<T>::MeterAlreadyAssigned
            );

            UserMeters::<T>::try_mutate(&owner, |meters| {
                meters
                    .try_push(meter_id.clone())
                    .map_err(|_| Error::<T>::TooManyMeters)
            })?;

            MeterOwners::<T>::insert(&meter_id, &owner);

            Self::deposit_event(Event::MeterAssigned { meter_id, owner });
            Ok(())
        }

        /// Unassign a smart meter from a user
        #[pallet::call_index(5)]
        #[pallet::weight(10_000)]
        pub fn unassign_meter(origin: OriginFor<T>, meter_id: MeterId) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_admin(&who), Error::<T>::NotAdmin);

            let former_owner = MeterOwners::<T>::get(&meter_id).ok_or(Error::<T>::MeterNotFound)?;

            UserMeters::<T>::mutate(&former_owner, |meters| {
                meters.retain(|m| m != &meter_id);
            });

            MeterOwners::<T>::remove(&meter_id);

            Self::deposit_event(Event::MeterUnassigned {
                meter_id,
                former_owner,
            });
            Ok(())
        }
    }

    /// Helper functions
    impl<T: Config> Pallet<T> {
        /// Check if an account is an administrator
        pub fn is_admin(account: &T::AccountId) -> bool {
            Admins::<T>::get().contains(account)
        }

        /// Check if a user is verified (registered and active)
        pub fn is_user_verified(user_account: &T::AccountId) -> bool {
            if let Some(user_info) = Users::<T>::get(user_account) {
                matches!(user_info.status, UserStatus::Active)
            } else {
                false
            }
        }

        /// Get the owner of a meter
        pub fn get_meter_owner(meter_id: &MeterId) -> Option<T::AccountId> {
            MeterOwners::<T>::get(meter_id)
        }

        /// Get current timestamp (placeholder - in real implementation would use pallet_timestamp)
        fn current_timestamp() -> u64 {
            // In a real implementation, this would use pallet_timestamp
            // For now, we'll use a placeholder
            0u64
        }

        /// Check if user is a prosumer
        pub fn is_prosumer(user_account: &T::AccountId) -> bool {
            if let Some(user_info) = Users::<T>::get(user_account) {
                matches!(user_info.user_type, UserType::Prosumer)
            } else {
                false
            }
        }

        /// Get all meters owned by a user
        pub fn get_user_meters(user_account: &T::AccountId) -> Vec<MeterId> {
            UserMeters::<T>::get(user_account).into_inner()
        }
    }

    #[cfg(test)]
    mod mock;

    #[cfg(test)]
    mod tests;
}

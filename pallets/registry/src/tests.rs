use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, traits::Get};
use pallet_registry::{UserInfo, UserStatus, UserType};

#[test]
fn test_add_admin_works() {
    new_test_ext().execute_with(|| {
        let alice = 1u64;
        let bob = 2u64;

        // Alice should be able to add Bob as admin
        assert_ok!(Registry::add_admin(RuntimeOrigin::signed(alice), bob));

        // Check that Bob is now an admin
        assert!(Registry::is_admin(&bob));

        // Check event was emitted
        System::assert_last_event(Event::AdminAdded { admin: bob }.into());
    });
}

#[test]
fn test_add_admin_fails_for_non_admin() {
    new_test_ext().execute_with(|| {
        let alice = 1u64;
        let bob = 2u64;
        let charlie = 3u64;

        // Bob (non-admin) should not be able to add Charlie as admin
        assert_noop!(
            Registry::add_admin(RuntimeOrigin::signed(bob), charlie),
            Error::<Test>::NotAdmin
        );

        // Charlie should not be an admin
        assert!(!Registry::is_admin(&charlie));
    });
}

#[test]
fn test_remove_admin_works() {
    new_test_ext().execute_with(|| {
        let alice = 1u64;
        let bob = 2u64;

        // First add Bob as admin
        assert_ok!(Registry::add_admin(RuntimeOrigin::signed(alice), bob));
        assert!(Registry::is_admin(&bob));

        // Alice should be able to remove Bob as admin
        assert_ok!(Registry::remove_admin(RuntimeOrigin::signed(alice), bob));

        // Bob should no longer be an admin
        assert!(!Registry::is_admin(&bob));

        // Check event was emitted
        System::assert_last_event(Event::AdminRemoved { admin: bob }.into());
    });
}

#[test]
fn test_cannot_remove_last_admin() {
    new_test_ext().execute_with(|| {
        let alice = 1u64;

        // Alice is the only admin, should not be able to remove herself
        assert_noop!(
            Registry::remove_admin(RuntimeOrigin::signed(alice), alice),
            Error::<Test>::CannotRemoveLastAdmin
        );

        // Alice should still be an admin
        assert!(Registry::is_admin(&alice));
    });
}

#[test]
fn test_register_user_works() {
    new_test_ext().execute_with(|| {
        let alice = 1u64; // admin
        let bob = 2u64; // user to register

        assert_ok!(Registry::register_user(
            RuntimeOrigin::signed(alice),
            bob,
            UserType::Prosumer,
            b"Building A".to_vec()
        ));

        // Check user is registered
        let user_info = Registry::users(bob).unwrap();
        assert_eq!(user_info.user_type, UserType::Prosumer);
        assert_eq!(user_info.location, b"Building A".to_vec());
        assert_eq!(user_info.status, UserStatus::Active);

        // Check user is verified
        assert!(Registry::is_user_verified(&bob));
        assert!(Registry::is_prosumer(&bob));

        // Check event was emitted
        System::assert_last_event(
            Event::UserRegistered {
                user: bob,
                user_type: UserType::Prosumer,
                location: b"Building A".to_vec(),
            }
            .into(),
        );
    });
}

#[test]
fn test_register_user_fails_for_non_admin() {
    new_test_ext().execute_with(|| {
        let bob = 2u64; // non-admin
        let charlie = 3u64; // user to register

        assert_noop!(
            Registry::register_user(
                RuntimeOrigin::signed(bob),
                charlie,
                UserType::Consumer,
                b"Building B".to_vec()
            ),
            Error::<Test>::NotAdmin
        );

        // Charlie should not be registered
        assert!(!Registry::is_user_verified(&charlie));
    });
}

#[test]
fn test_register_existing_user_fails() {
    new_test_ext().execute_with(|| {
        let alice = 1u64; // admin
        let bob = 2u64; // user

        // Register Bob first time
        assert_ok!(Registry::register_user(
            RuntimeOrigin::signed(alice),
            bob,
            UserType::Prosumer,
            b"Building A".to_vec()
        ));

        // Try to register Bob again
        assert_noop!(
            Registry::register_user(
                RuntimeOrigin::signed(alice),
                bob,
                UserType::Consumer,
                b"Building B".to_vec()
            ),
            Error::<Test>::UserAlreadyExists
        );
    });
}

#[test]
fn test_update_user_status_works() {
    new_test_ext().execute_with(|| {
        let alice = 1u64; // admin
        let bob = 2u64; // user

        // Register Bob first
        assert_ok!(Registry::register_user(
            RuntimeOrigin::signed(alice),
            bob,
            UserType::Consumer,
            b"Building B".to_vec()
        ));

        // Update Bob's status to suspended
        assert_ok!(Registry::update_user_status(
            RuntimeOrigin::signed(alice),
            bob,
            UserStatus::Suspended
        ));

        // Check status was updated
        let user_info = Registry::users(bob).unwrap();
        assert_eq!(user_info.status, UserStatus::Suspended);

        // User should no longer be verified
        assert!(!Registry::is_user_verified(&bob));

        // Check event was emitted
        System::assert_last_event(
            Event::UserStatusUpdated {
                user: bob,
                old_status: UserStatus::Active,
                new_status: UserStatus::Suspended,
            }
            .into(),
        );
    });
}

#[test]
fn test_assign_meter_works() {
    new_test_ext().execute_with(|| {
        let alice = 1u64; // admin
        let bob = 2u64; // user
        let meter_id = b"METER_001".to_vec();

        // Register Bob first
        assert_ok!(Registry::register_user(
            RuntimeOrigin::signed(alice),
            bob,
            UserType::Prosumer,
            b"Building A".to_vec()
        ));

        // Assign meter to Bob
        assert_ok!(Registry::assign_meter(
            RuntimeOrigin::signed(alice),
            meter_id.clone(),
            bob
        ));

        // Check meter is assigned
        assert_eq!(Registry::get_meter_owner(&meter_id), Some(bob));
        let user_meters = Registry::get_user_meters(&bob);
        assert_eq!(user_meters, vec![meter_id.clone()]);

        // Check event was emitted
        System::assert_last_event(
            Event::MeterAssigned {
                meter_id,
                owner: bob,
            }
            .into(),
        );
    });
}

#[test]
fn test_assign_meter_fails_for_non_admin() {
    new_test_ext().execute_with(|| {
        let alice = 1u64; // admin
        let bob = 2u64; // non-admin user
        let meter_id = b"METER_001".to_vec();

        // Register Bob first
        assert_ok!(Registry::register_user(
            RuntimeOrigin::signed(alice),
            bob,
            UserType::Prosumer,
            b"Building A".to_vec()
        ));

        // Bob tries to assign meter to himself (should fail)
        assert_noop!(
            Registry::assign_meter(RuntimeOrigin::signed(bob), meter_id.clone(), bob),
            Error::<Test>::NotAdmin
        );

        // Meter should not be assigned
        assert_eq!(Registry::get_meter_owner(&meter_id), None);
    });
}

#[test]
fn test_assign_meter_to_non_registered_user_fails() {
    new_test_ext().execute_with(|| {
        let alice = 1u64; // admin
        let bob = 2u64; // unregistered user
        let meter_id = b"METER_001".to_vec();

        // Try to assign meter to unregistered user
        assert_noop!(
            Registry::assign_meter(RuntimeOrigin::signed(alice), meter_id.clone(), bob),
            Error::<Test>::UserNotFound
        );

        // Meter should not be assigned
        assert_eq!(Registry::get_meter_owner(&meter_id), None);
    });
}

#[test]
fn test_assign_already_assigned_meter_fails() {
    new_test_ext().execute_with(|| {
        let alice = 1u64; // admin
        let bob = 2u64; // user 1
        let charlie = 3u64; // user 2
        let meter_id = b"METER_001".to_vec();

        // Register both users
        assert_ok!(Registry::register_user(
            RuntimeOrigin::signed(alice),
            bob,
            UserType::Prosumer,
            b"Building A".to_vec()
        ));
        assert_ok!(Registry::register_user(
            RuntimeOrigin::signed(alice),
            charlie,
            UserType::Consumer,
            b"Building B".to_vec()
        ));

        // Assign meter to Bob
        assert_ok!(Registry::assign_meter(
            RuntimeOrigin::signed(alice),
            meter_id.clone(),
            bob
        ));

        // Try to assign same meter to Charlie (should fail)
        assert_noop!(
            Registry::assign_meter(RuntimeOrigin::signed(alice), meter_id.clone(), charlie),
            Error::<Test>::MeterAlreadyAssigned
        );

        // Meter should still belong to Bob
        assert_eq!(Registry::get_meter_owner(&meter_id), Some(bob));
    });
}

#[test]
fn test_unassign_meter_works() {
    new_test_ext().execute_with(|| {
        let alice = 1u64; // admin
        let bob = 2u64; // user
        let meter_id = b"METER_001".to_vec();

        // Register Bob and assign meter
        assert_ok!(Registry::register_user(
            RuntimeOrigin::signed(alice),
            bob,
            UserType::Prosumer,
            b"Building A".to_vec()
        ));
        assert_ok!(Registry::assign_meter(
            RuntimeOrigin::signed(alice),
            meter_id.clone(),
            bob
        ));

        // Unassign meter
        assert_ok!(Registry::unassign_meter(
            RuntimeOrigin::signed(alice),
            meter_id.clone()
        ));

        // Check meter is unassigned
        assert_eq!(Registry::get_meter_owner(&meter_id), None);
        let user_meters = Registry::get_user_meters(&bob);
        assert!(user_meters.is_empty());

        // Check event was emitted
        System::assert_last_event(
            Event::MeterUnassigned {
                meter_id,
                former_owner: bob,
            }
            .into(),
        );
    });
}

#[test]
fn test_unassign_non_existent_meter_fails() {
    new_test_ext().execute_with(|| {
        let alice = 1u64; // admin
        let meter_id = b"METER_001".to_vec();

        // Try to unassign non-existent meter
        assert_noop!(
            Registry::unassign_meter(RuntimeOrigin::signed(alice), meter_id),
            Error::<Test>::MeterNotFound
        );
    });
}

#[test]
fn test_user_type_detection_works() {
    new_test_ext().execute_with(|| {
        let alice = 1u64; // admin
        let bob = 2u64; // prosumer
        let charlie = 3u64; // consumer

        // Register prosumer
        assert_ok!(Registry::register_user(
            RuntimeOrigin::signed(alice),
            bob,
            UserType::Prosumer,
            b"Building A".to_vec()
        ));

        // Register consumer
        assert_ok!(Registry::register_user(
            RuntimeOrigin::signed(alice),
            charlie,
            UserType::Consumer,
            b"Building B".to_vec()
        ));

        // Check user types
        assert!(Registry::is_prosumer(&bob));
        assert!(!Registry::is_prosumer(&charlie));
    });
}

#[test]
fn test_max_meters_per_user_limit() {
    new_test_ext().execute_with(|| {
        let alice = 1u64; // admin
        let bob = 2u64; // user

        // Register Bob
        assert_ok!(Registry::register_user(
            RuntimeOrigin::signed(alice),
            bob,
            UserType::Prosumer,
            b"Building A".to_vec()
        ));

        // Assign maximum number of meters (MaxMetersPerUser = 5 in mock)
        for i in 0..5 {
            let meter_id = format!("METER_{:03}", i).into_bytes();
            assert_ok!(Registry::assign_meter(
                RuntimeOrigin::signed(alice),
                meter_id,
                bob
            ));
        }

        // Try to assign one more meter (should fail)
        let meter_id = b"METER_006".to_vec();
        assert_noop!(
            Registry::assign_meter(RuntimeOrigin::signed(alice), meter_id, bob),
            Error::<Test>::TooManyMeters
        );
    });
}

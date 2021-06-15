use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use sp_io::hashing::{blake2_128, keccak_256};



const A: u64 = 1;
const B: u64 = 2;


#[test]
fn add_to_order() {
	new_test_ext().execute_with(|| {
        let price: u128 = 5;
		assert_ok!(AcuityAtomicSwap::add_to_order(Origin::signed(A), price, 50));

        assert_eq!(Balances::free_balance(A), 50);
        assert_eq!(Balances::free_balance(AcuityAtomicSwap::fund_account_id()), 50);

        let order_id: [u8; 16] = blake2_128(&[A.to_ne_bytes().to_vec(), price.to_ne_bytes().to_vec()].concat());

        assert_eq!(AcuityAtomicSwap::order_id_value(order_id), 50);

	});
}

#[test]
fn remove_from_order_control_order_to_small() {
	new_test_ext().execute_with(|| {
        let price: u128 = 5;
        assert_ok!(AcuityAtomicSwap::add_to_order(Origin::signed(A), price, 50));
        assert_ok!(AcuityAtomicSwap::add_to_order(Origin::signed(A), price, 50));
	});
}

#[test]
fn remove_from_order_fail_order_to_small() {
	new_test_ext().execute_with(|| {
        let price: u128 = 5;
        assert_ok!(AcuityAtomicSwap::add_to_order(Origin::signed(A), price, 50));
		assert_noop!(
            AcuityAtomicSwap::remove_from_order(Origin::signed(A), price, 51),
			Error::<Test>::OrderTooSmall
		);
	});
}

#[test]
fn remove_from_order() {
	new_test_ext().execute_with(|| {
        let price: u128 = 5;
		assert_ok!(AcuityAtomicSwap::add_to_order(Origin::signed(A), price, 50));

        assert_eq!(Balances::free_balance(A), 50);
        assert_eq!(Balances::free_balance(AcuityAtomicSwap::fund_account_id()), 50);

        let order_id: [u8; 16] = blake2_128(&[A.to_ne_bytes().to_vec(), price.to_ne_bytes().to_vec()].concat());

        assert_eq!(AcuityAtomicSwap::order_id_value(order_id), 50);

        assert_ok!(AcuityAtomicSwap::remove_from_order(Origin::signed(A), price, 50));
        assert_eq!(Balances::free_balance(A), 100);
        assert_eq!(Balances::free_balance(AcuityAtomicSwap::fund_account_id()), 0);
        assert_eq!(AcuityAtomicSwap::order_id_value(order_id), 0);

	});
}

#[test]
fn lock_sell_order_control_too_small() {
    new_test_ext().execute_with(|| {
        let price: u128 = 5;
        assert_ok!(AcuityAtomicSwap::add_to_order(Origin::signed(A), price, 50));
        assert_ok!(AcuityAtomicSwap::lock_sell(Origin::signed(A), price, [0; 32], 0, 50));
	});
}

#[test]
fn lock_sell_order_fail_too_small() {
    new_test_ext().execute_with(|| {
        let price: u128 = 5;
        assert_ok!(AcuityAtomicSwap::add_to_order(Origin::signed(A), price, 50));
		assert_noop!(
            AcuityAtomicSwap::lock_sell(Origin::signed(A), price, [0; 32], 0, 60),
			Error::<Test>::OrderTooSmall
		);
	});
}

#[test]
fn lock_sell_order_control_already_in_use() {
    new_test_ext().execute_with(|| {
        let price: u128 = 5;
        assert_ok!(AcuityAtomicSwap::add_to_order(Origin::signed(A), price, 50));
        assert_ok!(AcuityAtomicSwap::lock_sell(Origin::signed(A), price, [0; 32], 0, 10));
        assert_ok!(AcuityAtomicSwap::lock_sell(Origin::signed(A), price, [1; 32], 0, 10));
	});
}

#[test]
fn lock_sell_order_fail_already_in_use() {
    new_test_ext().execute_with(|| {
        let price: u128 = 5;
        assert_ok!(AcuityAtomicSwap::add_to_order(Origin::signed(A), price, 50));
        assert_ok!(AcuityAtomicSwap::lock_sell(Origin::signed(A), price, [0; 32], 0, 10));
		assert_noop!(
            AcuityAtomicSwap::lock_sell(Origin::signed(A), price, [0; 32], 0, 10),
			Error::<Test>::HashedSecretAlreadyInUse
		);
	});
}

#[test]
fn lock_sell() {
	new_test_ext().execute_with(|| {
        let price: u128 = 5;
		assert_ok!(AcuityAtomicSwap::add_to_order(Origin::signed(A), price, 50));

        assert_eq!(Balances::free_balance(A), 50);
        assert_eq!(Balances::free_balance(AcuityAtomicSwap::fund_account_id()), 50);

        let order_id: [u8; 16] = blake2_128(&[A.to_ne_bytes().to_vec(), price.to_ne_bytes().to_vec()].concat());

        assert_eq!(AcuityAtomicSwap::order_id_value(order_id), 50);

        assert_ok!(AcuityAtomicSwap::lock_sell(Origin::signed(A), price, [0; 32], 0, 10));
        assert_eq!(AcuityAtomicSwap::order_id_value(order_id), 40);

        let lock = AcuityAtomicSwap::sell_lock([0; 32]);
        assert_eq!(lock.order_id, order_id);
        assert_eq!(lock.value, 10);
        assert_eq!(lock.timeout, 0);

	});
}

#[test]
fn unlock_sell_order_control_timed_out() {
    new_test_ext().execute_with(|| {
        let price: u128 = 5;
        let secret = [0; 32];
        let hashed_secret: [u8; 32] = keccak_256(&secret);
        let _now = <pallet_timestamp::Pallet<Test>>::get();
		assert_ok!(AcuityAtomicSwap::add_to_order(Origin::signed(A), price, 50));
        assert_ok!(AcuityAtomicSwap::lock_sell(Origin::signed(A), price, hashed_secret, _now + 1000, 10));
        assert_ok!(AcuityAtomicSwap::unlock_sell(Origin::signed(B), secret));
	});
}

#[test]
fn unlock_sell_order_fail_timed_out() {
    new_test_ext().execute_with(|| {
        let price: u128 = 5;
        let secret = [0; 32];
        let hashed_secret: [u8; 32] = keccak_256(&secret);
        let _now = <pallet_timestamp::Pallet<Test>>::get();
		assert_ok!(AcuityAtomicSwap::add_to_order(Origin::signed(A), price, 50));
        assert_ok!(AcuityAtomicSwap::lock_sell(Origin::signed(A), price, hashed_secret, _now, 10));
		assert_noop!(
            AcuityAtomicSwap::unlock_sell(Origin::signed(B), secret),
			Error::<Test>::LockTimedOut
		);
	});
}

#[test]
fn unlock_sell() {
	new_test_ext().execute_with(|| {
        let price: u128 = 5;
        let secret = [0; 32];
        let hashed_secret: [u8; 32] = keccak_256(&secret);
        let _now = <pallet_timestamp::Pallet<Test>>::get();
		assert_ok!(AcuityAtomicSwap::add_to_order(Origin::signed(A), price, 50));

        assert_eq!(Balances::free_balance(A), 50);
        assert_eq!(Balances::free_balance(B), 200);
        assert_eq!(Balances::free_balance(AcuityAtomicSwap::fund_account_id()), 50);

        let order_id: [u8; 16] = blake2_128(&[A.to_ne_bytes().to_vec(), price.to_ne_bytes().to_vec()].concat());

        assert_eq!(AcuityAtomicSwap::order_id_value(order_id), 50);

        assert_ok!(AcuityAtomicSwap::lock_sell(Origin::signed(A), price, hashed_secret, _now + 1000, 10));
        assert_eq!(AcuityAtomicSwap::order_id_value(order_id), 40);

        let lock = AcuityAtomicSwap::sell_lock(hashed_secret);
        assert_eq!(lock.order_id, order_id);
        assert_eq!(lock.value, 10);
        assert_eq!(lock.timeout, _now + 1000);

        assert_ok!(AcuityAtomicSwap::unlock_sell(Origin::signed(B), secret));
        assert_eq!(AcuityAtomicSwap::order_id_value(order_id), 40);

        let lock = AcuityAtomicSwap::sell_lock(hashed_secret);
        assert_eq!(lock.value, 0);
        assert_eq!(lock.timeout, 0);

        assert_eq!(Balances::free_balance(A), 50);
        assert_eq!(Balances::free_balance(B), 210);
        assert_eq!(Balances::free_balance(AcuityAtomicSwap::fund_account_id()), 40);

	});
}

#[test]
fn timeout_sell_order_control_wrong_order_id() {
    new_test_ext().execute_with(|| {
        let price: u128 = 5;
        let secret = [0; 32];
        let hashed_secret: [u8; 32] = keccak_256(&secret);
        let _now = <pallet_timestamp::Pallet<Test>>::get();
		assert_ok!(AcuityAtomicSwap::add_to_order(Origin::signed(A), price, 50));
        assert_ok!(AcuityAtomicSwap::lock_sell(Origin::signed(A), price, hashed_secret, _now, 10));
        assert_ok!(AcuityAtomicSwap::timeout_sell(Origin::signed(A), price, hashed_secret));
	});
}

#[test]
fn timeout_sell_order_fail_wrong_order_id() {
    new_test_ext().execute_with(|| {
        let price: u128 = 5;
        let secret = [0; 32];
        let hashed_secret: [u8; 32] = keccak_256(&secret);
        let _now = <pallet_timestamp::Pallet<Test>>::get();
		assert_ok!(AcuityAtomicSwap::add_to_order(Origin::signed(A), price, 50));
        assert_ok!(AcuityAtomicSwap::lock_sell(Origin::signed(A), price, hashed_secret, _now, 10));
		assert_noop!(
            AcuityAtomicSwap::timeout_sell(Origin::signed(A), price - 1, hashed_secret),
			Error::<Test>::WrongOrderId
		);
	});
}

#[test]
fn timeout_sell_order_control_not_timed_out() {
    new_test_ext().execute_with(|| {
        let price: u128 = 5;
        let secret = [0; 32];
        let hashed_secret: [u8; 32] = keccak_256(&secret);
        let _now = <pallet_timestamp::Pallet<Test>>::get();
		assert_ok!(AcuityAtomicSwap::add_to_order(Origin::signed(A), price, 50));
        assert_ok!(AcuityAtomicSwap::lock_sell(Origin::signed(A), price, hashed_secret, _now, 10));
        assert_ok!(AcuityAtomicSwap::timeout_sell(Origin::signed(A), price, hashed_secret));
	});
}

#[test]
fn timeout_sell_order_fail_not_timed_out() {
    new_test_ext().execute_with(|| {
        let price: u128 = 5;
        let secret = [0; 32];
        let hashed_secret: [u8; 32] = keccak_256(&secret);
        let _now = <pallet_timestamp::Pallet<Test>>::get();
		assert_ok!(AcuityAtomicSwap::add_to_order(Origin::signed(A), price, 50));
        assert_ok!(AcuityAtomicSwap::lock_sell(Origin::signed(A), price, hashed_secret, _now + 1000, 10));
		assert_noop!(
            AcuityAtomicSwap::timeout_sell(Origin::signed(A), price, hashed_secret),
			Error::<Test>::LockNotTimedOut
		);
	});
}

#[test]
fn timeout_sell() {
	new_test_ext().execute_with(|| {
        let price: u128 = 5;
        let secret = [0; 32];
        let hashed_secret: [u8; 32] = keccak_256(&secret);
        let _now = <pallet_timestamp::Pallet<Test>>::get();
		assert_ok!(AcuityAtomicSwap::add_to_order(Origin::signed(A), price, 50));

        assert_eq!(Balances::free_balance(A), 50);
        assert_eq!(Balances::free_balance(B), 200);
        assert_eq!(Balances::free_balance(AcuityAtomicSwap::fund_account_id()), 50);

        let order_id: [u8; 16] = blake2_128(&[A.to_ne_bytes().to_vec(), price.to_ne_bytes().to_vec()].concat());

        assert_eq!(AcuityAtomicSwap::order_id_value(order_id), 50);

        assert_ok!(AcuityAtomicSwap::lock_sell(Origin::signed(A), price, hashed_secret, _now, 10));
        assert_eq!(AcuityAtomicSwap::order_id_value(order_id), 40);

        let lock = AcuityAtomicSwap::sell_lock(hashed_secret);
        assert_eq!(lock.order_id, order_id);
        assert_eq!(lock.value, 10);
        assert_eq!(lock.timeout, _now);

        assert_ok!(AcuityAtomicSwap::timeout_sell(Origin::signed(A), price, hashed_secret));
        assert_eq!(AcuityAtomicSwap::order_id_value(order_id), 50);

        let lock = AcuityAtomicSwap::sell_lock(hashed_secret);
        assert_eq!(lock.value, 0);
        assert_eq!(lock.timeout, 0);

        assert_eq!(Balances::free_balance(A), 50);
        assert_eq!(Balances::free_balance(B), 200);
        assert_eq!(Balances::free_balance(AcuityAtomicSwap::fund_account_id()), 50);

	});
}

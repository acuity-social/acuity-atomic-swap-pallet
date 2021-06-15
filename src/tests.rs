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

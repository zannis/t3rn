use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        assert_ok!(TestModule::do_something(
            Origin::signed(1),
            String::from("42").into_bytes()
        ));
        // Read pallet storage and assert an expected result.
        assert_eq!(
            TestModule::something(),
            Some(String::from("42").into_bytes())
        );
        assert_ok!(TestModule::do_something(
            Origin::signed(1),
            String::from("43").into_bytes()
        ));
    });
}

#[test]
fn correct_error_for_none_value() {
    new_test_ext().execute_with(|| {
        // Ensure the expected error is thrown when no value is present.
        assert_noop!(
            TestModule::cause_error(Origin::signed(1)),
            Error::<Test>::NoneValue
        );
    });
}

#[test]
fn add_member_to_the_pool() {
    new_test_ext().execute_with(|| {
        assert_ok!(TestModule::add_member(Origin::signed(1)));
        assert_ok!(TestModule::add_member(Origin::signed(2)));
        assert_eq!(TestModule::members(), Some(vec![1 as u64, 2]).unwrap());

        // assert_eq!(TestModule::members(), vec![Origin::signed(1)]);
    })
}

#[test]
fn remove_member_from_the_pool() {
    new_test_ext().execute_with(|| {
        assert_ok!(TestModule::add_member(Origin::signed(1)));
        assert_ok!(TestModule::add_member(Origin::signed(2)));
        assert_eq!(TestModule::members(), Some(vec![1 as u64, 2]).unwrap());
        assert_ok!(TestModule::remove_member(Origin::signed(2)));
        assert_eq!(TestModule::members(), Some(vec![1 as u64]).unwrap());
    })
}

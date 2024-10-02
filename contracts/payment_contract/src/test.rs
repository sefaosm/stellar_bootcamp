// contracts\payment_contract\src\test.rs
// Test file does not work properly at the moment. Need to figure out how to get it to work.
#![allow(warnings)]

use soroban_sdk::Env;
use soroban_sdk::{contract, Symbol, Vec, String};
use soroban_sdk::testutils::Address;

use crate::PaymentContract;

#[test]
fn test() {
    let env = Env::default();

    let buyer = <soroban_sdk::Address as Address>::generate(&env);
    let seller = <soroban_sdk::Address as Address>::generate(&env);
    
    // seller.require_auth();
    // buyer.require_auth();

    // let amount = 100; // This is changable

    // PaymentContract::transfer(env.clone(), buyer.clone(), seller.clone(), amount);
    // assert_eq!(PaymentContract::get_balance(env.clone(), buyer.clone()), amount);
    // assert_eq!(PaymentContract::get_balance(env.clone(), seller.clone()), 0);
}
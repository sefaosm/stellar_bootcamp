#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, log, Address, Env, IntoVal, String, Symbol, Vec};

#[contract]
/// A contract for making payments of assets from one account to another.
///
/// This contract provides two methods for making payments. The first, `pay`, is
/// for making a single payment from one account to another. The second, `pay_multiple`,
/// is for making multiple payments to different accounts.
pub struct PaymentContract;

#[contracttype]
pub struct Payment {
    from: Address,
    to: Address,
    asset_code: Symbol,
    amount: u64,
}

#[contractimpl]
// Implementations of the methods of the `PaymentContract` contract.
impl PaymentContract {
    // XLM Transfer
    pub fn transfer(env: Env, from: Address, to: Address, amount: u64) {
        let asset_code = Symbol::new(&env, "XLM");
        env.invoke_contract::<()>(
            &from,
            &Symbol::new(&env, "pay"),
            (&from, to, asset_code, amount).into_val(&env),
        );
    }

    // Paying method with message
    pub fn transfer_with_message(env: Env, from: Address, to: Address, amount: u64, message: String) {
        let asset_code = Symbol::new(&env, "XLM");
        // XLM transferring
        env.invoke_contract::<()>(
            &from,
            &Symbol::new(&env, "pay"),
            (&from, to, asset_code, amount).into_val(&env),
        );

        // Logging the message
        log!(&env, "Message: {}", message);
    }

    // Balance Inquiry
    pub fn get_balance(env: Env, account: Address) -> u64 {
        let asset_code = Symbol::new(&env, "XLM");
        let balance: u64 = env.invoke_contract(
            &account,
            &Symbol::new(&env, "get_balance"),
            (&account, asset_code).into_val(&env),
        );
        balance
    }

    /// Schedule a payment to be made 'num_payments' times. Each payment will
    /// be for 'amount' units of the asset with code 'XLM'. The payment will be
    /// made from 'from' to 'to'.
    pub fn schedule_payment(env: Env, from: Address, to: Address, amount: u64, num_payments: u32) {
        let asset_code = Symbol::new(&env, "XLM");

        // Do this process 'num_payments' times.
        for _ in 0..num_payments {
            env.invoke_contract::<()>(
                &from,
                &Symbol::new(&env, "pay"),
                (from.clone(), to.clone(), asset_code.clone(), amount).into_val(&env),
            );
        }
    }

    // Paying to multiple recipients
    pub fn transfer_to_multiple(env: Env, from: Address, recipients: Vec<Address>, amount: u64) {
        let asset_code = Symbol::new(&env, "XLM");
        for recipient in recipients.iter() {
            env.invoke_contract::<()>(
                &from,
                &Symbol::new(&env, "pay"),
                (from.clone(), recipient.clone(), asset_code.clone(), amount).into_val(&env),
            );
        }
    }
}

mod test;
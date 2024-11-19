use ic_cdk_macros::*;
use std::collections::HashMap;

#[derive(Default)]
struct Wallet {
    balances: HashMap<String, u64>, // Maps token names to their balances
}

static mut WALLET: Option<Wallet> = None;

// Initialize the canister and set up the wallet
#[init]
fn init() {
    ic_cdk::println!("Wallet initialized successfully!");
    unsafe {
        WALLET = Some(Wallet::default());
    }
}

// Main function for testing or initialization in development
// fn main() {
    // ic_cdk::run();
// }

// Function to send tokens to another address
#[update]
fn send_tokens(to: String, amount: u64) -> String {
    let caller = ic_cdk::caller().to_string();
    unsafe {
        let wallet = WALLET.as_mut().unwrap();

        // Handle the caller's balance
        let balance = wallet.balances.get_mut(&caller);
        if let Some(bal) = balance {
            if *bal < amount {
                return format!(
                    "Insufficient balance: You have {} tokens but tried to send {} tokens.",
                    bal, amount
                );
            }
            *bal -= amount;
        } else {
            return "Caller has no balance.".to_string();
        }

        // Handle the recipient's balance
        let recipient_balance = wallet.balances.entry(to.clone()).or_insert(0);
        *recipient_balance += amount;

        ic_cdk::println!(
            "{} tokens sent from {} to {}.",
            amount,
            caller,
            to
        );

        format!(
            "Successfully sent {} tokens to {}. Your new balance is {}.",
            amount,
            to,
            wallet.balances.get(&caller).unwrap_or(&0)
        )
    }
}


// Function to receive tokens (can be used by another contract or user)
#[update]
fn receive_tokens(from: String, amount: u64) -> String {
    let caller = ic_cdk::caller().to_string();
    unsafe {
        let wallet = WALLET.as_mut().unwrap();

        // Update the caller's balance
        let balance = wallet.balances.entry(caller.clone()).or_insert(0);
        *balance += amount;

        ic_cdk::println!(
            "{} tokens received from {} to {}.",
            amount,
            from,
            caller
        );

        format!(
            "Successfully received {} tokens from {}. Your new balance is {}.",
            amount, from, balance
        )
    }
}

// Function to fetch and display the current token balance of the caller
#[query]
fn get_balance() -> String {
    let caller = ic_cdk::caller().to_string();
    unsafe {
        let wallet = WALLET.as_ref().unwrap();
        let balance = wallet.balances.get(&caller).unwrap_or(&0);
        format!(
            "Your current balance is {} tokens.",
            balance
        )
    }
}

// Function to fetch and display the token balance of a specific user
#[query]
fn get_balance_of(user: String) -> String {
    unsafe {
        let wallet = WALLET.as_ref().unwrap();
        let balance = wallet.balances.get(&user).unwrap_or(&0);
        format!(
            "The current balance of {} is {} tokens.",
            user, balance
        )
    }
}

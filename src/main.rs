mod account;
mod transaction;
mod utils;

use std::collections::HashMap;
use account::Account;
use transaction::transfer;

fn main() {
    let mut accounts = HashMap::new();

    accounts.insert(1, Account::new(1));
    accounts.insert(2, Account::new(2));

    accounts.get_mut(&1).unwrap().deposit(500.0);

    match transfer(&mut accounts, 1, 2, 200.0) {
        Ok(()) => println!("Transfer successful!"),
        Err(e) => println!("Transfer failed: {}", e),
    }
}

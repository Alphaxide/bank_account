use std::collections::HashMap;
use crate::account::{Account, AccountStatus};
use crate::utils::log_transaction;

pub fn transfer(accounts: &mut HashMap<u32, Account>, from_id: u32, to_id: u32, amount: f64) -> Result<(), String> {
    
    let from_account_status = accounts.get(&from_id)
        .ok_or("Sender account not found".to_string())?
        .status()
        .clone();

    if let AccountStatus::Closed = from_account_status {
        return Err("Sender account is closed".to_string());
    }

    
    let to_account_status = accounts.get(&to_id)
        .ok_or("Receiver account not found".to_string())?
        .status()
        .clone();

    if let AccountStatus::Closed = to_account_status {
        return Err("Receiver account is closed".to_string());
    }

    
    let mut from_account = accounts.remove(&from_id).ok_or("Sender account not found".to_string())?;
    let mut to_account = accounts.remove(&to_id).ok_or("Receiver account not found".to_string())?;

    
    from_account.withdraw(amount)?;
    to_account.deposit(amount);

    
    accounts.insert(from_id, from_account);
    accounts.insert(to_id, to_account);

    
    log_transaction(from_id, to_id, amount).map_err(|e| e.to_string())?;
    Ok(())
}

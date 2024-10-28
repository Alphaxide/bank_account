#[derive(Debug)]
pub enum AccountStatus {
    Active,
    Closed,
}

#[derive(Debug)]
pub struct Account {
    id: u32,
    balance: f64,
    status: AccountStatus,
}

impl Account {
    pub fn new(id: u32) -> Self {
        Account {
            id,
            balance: 0.0,
            status: AccountStatus::Active,
        }
    }

    pub fn close(&mut self) {
        self.status = AccountStatus::Closed;
    }

    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if self.balance >= amount {
            self.balance -= amount;
            Ok(())
        } else {
            Err("Insufficient funds".to_string())
        }
    }

    pub fn status(&self) -> &AccountStatus {
        &self.status
    }
}

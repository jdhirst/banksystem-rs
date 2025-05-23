use serde::{Serialize, Deserialize};
use super::{Transaction, TransactionType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: u64,
    pub customer_id: u64,
    pub name: String,
    pub account_type: String,
    pub balance: f64,
    pub history: Vec<Transaction>,
}

impl Account {
    pub fn new(id: u64, customer_id: u64, name: String, account_type: String) -> Self {
        Self {
            id,
            customer_id,
            name,
            account_type,
            balance: 0.0,
            history: vec![],
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        self.history.push(Transaction::new(
            TransactionType::Deposit,
            amount,
        ));
    }

    pub fn withdraw(&mut self, amount: f64) -> bool {
        if self.balance >= amount {
            self.balance -= amount;
            self.history.push(Transaction::new(
                TransactionType::Withdrawal,
                amount,
            ));
            true
        } else {
            false
        }
    }

    pub fn transfer(&mut self, amount: f64, to: &mut Account) -> bool {
        if self.withdraw(amount) {
            to.deposit(amount);
            self.history.push(Transaction::new(
                TransactionType::Transfer { to_account: to.id },
                amount,
            ));
            true
        } else {
            false
        }
    }

    pub fn get_history(&self) -> &[Transaction] {
        &self.history
    }
}

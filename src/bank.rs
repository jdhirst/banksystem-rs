use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Transfer { to_account: u64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub tx_type: TransactionType,
    pub amount: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: u64,
    pub name: String,
    pub balance: f64,
    pub history: Vec<Transaction>,
}

impl Account {
    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        self.history.push(Transaction {
            tx_type: TransactionType::Deposit,
            amount,
            timestamp: Utc::now(),
        });
    }
    pub fn withdraw(&mut self, amount: f64) -> bool {
        if self.balance >= amount {
            self.balance -= amount;
            self.history.push(Transaction {
                tx_type: TransactionType::Withdrawal,
                amount,
                timestamp: Utc::now(),
            });
            true
        } else {
            false
        }
    }
    pub fn transfer(&mut self, amount: f64, to: &mut Account) -> bool {
        if self.withdraw(amount) {
            to.deposit(amount);
            self.history.push(Transaction {
                tx_type: TransactionType::Transfer { to_account: to.id },
                amount,
                timestamp: Utc::now(),
            });
            true
        } else {
            false
        }
    }
}

#[derive(Default)]
pub struct Bank {
    pub accounts: Vec<Account>,
    pub next_id: u64,
}

impl Bank {
    pub fn new() -> Self {
        Self { accounts: vec![], next_id: 1 }
    }
    pub fn create_account(&mut self, name: String) -> u64 {
        let id = self.next_id;
        self.accounts.push(Account {
            id,
            name,
            balance: 0.0,
            history: vec![],
        });
        self.next_id += 1;
        id
    }
    pub fn get_account_mut(&mut self, id: u64) -> Option<&mut Account> {
        self.accounts.iter_mut().find(|a| a.id == id)
    }
    pub fn get_account(&self, id: u64) -> Option<&Account> {
        self.accounts.iter().find(|a| a.id == id)
    }
    pub fn list_accounts(&self) -> &Vec<Account> {
        &self.accounts
    }
}

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

impl Transaction {
    pub fn new(tx_type: TransactionType, amount: f64) -> Self {
        Self {
            tx_type,
            amount,
            timestamp: Utc::now(),
        }
    }
}

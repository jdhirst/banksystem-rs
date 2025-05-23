mod account;
mod customer;
mod transaction;

pub use account::Account;
pub use customer::Customer;
pub use transaction::{Transaction, TransactionType};

#[derive(Default)]
pub struct Bank {
    pub accounts: Vec<Account>,
    customers: Vec<Customer>,
    next_id: u64,
}

impl Bank {
    pub fn new() -> Self {
        Self {
            accounts: vec![],
            customers: vec![],
            next_id: 1,
        }
    }

    pub fn create_customer(&mut self, name: String, address: String, phone: String, email: String) -> u64 {
        let id = self.next_id;
        self.customers.push(Customer::new(id, name, address, phone, email));
        self.next_id += 1;
        id
    }

    pub fn create_account(&mut self, customer_id: u64, account_type: String) -> u64 {
        let id = self.next_id;
        if let Some(_customer) = self.customers.iter().find(|c| c.id == customer_id) {
            self.accounts.push(Account::new(
                id,
                customer_id,
                format!("{} Account", account_type),
                account_type,
            ));
            self.next_id += 1;
            id
        } else {
            panic!("Customer not found");
        }
    }

    pub fn get_customer(&self, id: u64) -> Option<&Customer> {
        self.customers.iter().find(|c| c.id == id)
    }

    pub fn get_customer_mut(&mut self, id: u64) -> Option<&mut Customer> {
        self.customers.iter_mut().find(|c| c.id == id)
    }

    pub fn get_account(&self, id: u64) -> Option<&Account> {
        self.accounts.iter().find(|a| a.id == id)
    }

    pub fn get_account_mut(&mut self, id: u64) -> Option<&mut Account> {
        self.accounts.iter_mut().find(|a| a.id == id)
    }

    pub fn list_accounts(&self) -> &Vec<Account> {
        &self.accounts
    }

    pub fn list_customer_accounts(&self, customer_id: u64) -> Vec<&Account> {
        self.accounts.iter()
            .filter(|a| a.customer_id == customer_id)
            .collect()
    }

    pub fn list_customers(&self) -> &Vec<Customer> {
        &self.customers
    }
}

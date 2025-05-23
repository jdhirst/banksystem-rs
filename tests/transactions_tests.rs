use banksystem_rs::bank::{Bank, TransactionType};

#[test]
fn test_transaction_history() {
    let mut bank = Bank::new();
    let customer_id = bank.create_customer(
        "Test User".to_string(),
        "Test Address".to_string(),
        "555-0000".to_string(),
        "test@example.com".to_string(),
    );
    let account_id = bank.create_account(customer_id, "Checking".to_string());
    
    let account = bank.get_account_mut(account_id).unwrap();
    account.deposit(100.0);
    account.withdraw(30.0);
    
    let history = account.get_history();
    assert_eq!(history.len(), 2);
    assert_eq!(history[0].transaction_type, TransactionType::Deposit);
    assert_eq!(history[0].amount, 100.0);
    assert_eq!(history[1].transaction_type, TransactionType::Withdrawal);
    assert_eq!(history[1].amount, 30.0);
}

#[test]
fn test_invalid_operations() {
    let mut bank = Bank::new();
    let customer_id = bank.create_customer(
        "Edge Case".to_string(),
        "Edge Address".to_string(),
        "555-9999".to_string(),
        "edge@example.com".to_string(),
    );
    let account_id = bank.create_account(customer_id, "Checking".to_string());
    
    let account = bank.get_account_mut(account_id).unwrap();
    
    // Test negative deposit
    assert_eq!(account.balance, 0.0);
    account.deposit(-50.0);  // Should be ignored or handled
    assert_eq!(account.balance, 0.0);
    
    // Test overdraft
    let withdraw_result = account.withdraw(100.0);
    assert!(!withdraw_result);
    assert_eq!(account.balance, 0.0);
    
    // Test valid operations after invalid ones
    account.deposit(100.0);
    assert_eq!(account.balance, 100.0);
    let withdraw_result = account.withdraw(50.0);
    assert!(withdraw_result);
    assert_eq!(account.balance, 50.0);
}

#[test]
fn test_account_types() {
    let mut bank = Bank::new();
    let customer_id = bank.create_customer(
        "Type Test".to_string(),
        "Type Address".to_string(),
        "555-8888".to_string(),
        "type@example.com".to_string(),
    );
    
    // Test different account types
    let checking_id = bank.create_account(customer_id, "Checking".to_string());
    let savings_id = bank.create_account(customer_id, "Savings".to_string());
    let credit_id = bank.create_account(customer_id, "Credit".to_string());
    
    assert_eq!(bank.get_account(checking_id).unwrap().account_type, "Checking");
    assert_eq!(bank.get_account(savings_id).unwrap().account_type, "Savings");
    assert_eq!(bank.get_account(credit_id).unwrap().account_type, "Credit");
}

#[test]
fn test_transfer_validation() {
    let mut bank = Bank::new();
    let customer_id = bank.create_customer(
        "Transfer Test".to_string(),
        "Transfer Address".to_string(),
        "555-7777".to_string(),
        "transfer@example.com".to_string(),
    );
    
    let account1_id = bank.create_account(customer_id, "Checking".to_string());
    let account2_id = bank.create_account(customer_id, "Savings".to_string());
    
    // Get mutable references to both accounts
    let account1 = bank.get_account_mut(account1_id).unwrap();
    account1.deposit(100.0);
    
    let account2 = bank.get_account_mut(account2_id).unwrap();
    
    // Test transfer with insufficient funds
    let result = account1.transfer(150.0, account2);
    assert!(!result);
    assert_eq!(account1.balance, 100.0);
    assert_eq!(account2.balance, 0.0);
    
    // Test valid transfer
    let result = account1.transfer(50.0, account2);
    assert!(result);
    assert_eq!(account1.balance, 50.0);
    assert_eq!(account2.balance, 50.0);
}

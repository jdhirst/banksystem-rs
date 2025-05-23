use banksystem_rs::bank::Bank;

#[test]
fn test_customer_creation() {
    let mut bank = Bank::new();
    let id = bank.create_customer(
        "John Doe".to_string(),
        "123 Main St".to_string(),
        "555-1234".to_string(),
        "john@example.com".to_string(),
    );
    assert_eq!(id, 1);
    let customer = bank.get_customer(id).unwrap();
    assert_eq!(customer.name, "John Doe");
    assert_eq!(customer.address, "123 Main St");
    assert_eq!(customer.phone, "555-1234");
    assert_eq!(customer.email, "john@example.com");
}

#[test]
fn test_customer_account_relationship() {
    let mut bank = Bank::new();
    let customer_id = bank.create_customer(
        "Jane Smith".to_string(),
        "456 Oak Ave".to_string(),
        "555-5678".to_string(),
        "jane@example.com".to_string(),
    );
    
    // Create multiple accounts for the customer
    let checking_id = bank.create_account(customer_id, "Checking".to_string());
    let savings_id = bank.create_account(customer_id, "Savings".to_string());
    
    // Test account listing
    let customer_accounts = bank.list_customer_accounts(customer_id);
    assert_eq!(customer_accounts.len(), 2);
    assert!(customer_accounts.iter().any(|a| a.account_type == "Checking"));
    assert!(customer_accounts.iter().any(|a| a.account_type == "Savings"));
}

#[test]
fn test_customer_not_found() {
    let bank = Bank::new();
    assert!(bank.get_customer(999).is_none());
    assert!(bank.get_customer_mut(999).is_none());
}

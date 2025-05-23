use banksystem_rs::bank::{Bank, Account};

#[test]
fn test_account_creation() {
    let mut bank = Bank::new();
    let id = bank.create_account("Alice".to_string());
    assert_eq!(id, 1);
    let account = bank.get_account(id).unwrap();
    assert_eq!(account.name, "Alice");
    assert_eq!(account.balance, 0.0);
}

#[test]
fn test_deposit() {
    let mut bank = Bank::new();
    let id = bank.create_account("Bob".to_string());
    let acc = bank.get_account_mut(id).unwrap();
    acc.deposit(100.0);
    assert_eq!(acc.balance, 100.0);
}

#[test]
fn test_withdrawal() {
    let mut bank = Bank::new();
    let id = bank.create_account("Carol".to_string());
    let acc = bank.get_account_mut(id).unwrap();
    acc.deposit(100.0);
    let result = acc.withdraw(40.0);
    assert!(result);
    assert_eq!(acc.balance, 60.0);
    let fail = acc.withdraw(100.0);
    assert!(!fail);
    assert_eq!(acc.balance, 60.0);
}

#[test]
fn test_transfer() {
    let mut bank = Bank::new();
    let id1 = bank.create_account("Dave".to_string());
    let id2 = bank.create_account("Eve".to_string());
    {
        let acc1 = bank.get_account_mut(id1).unwrap();
        acc1.deposit(200.0);
    }
    let (acc1, acc2) = {
        let (left, right) = bank.accounts.split_at_mut(1);
        (&mut left[0], &mut right[0])
    };
    let result = acc1.transfer(50.0, acc2);
    assert!(result);
    assert_eq!(acc1.balance, 150.0);
    assert_eq!(acc2.balance, 50.0);
}

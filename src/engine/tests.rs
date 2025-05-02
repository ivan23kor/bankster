use super::*;
use crate::transaction::{Transaction, TransactionType};

/// Macro to create a [`Transaction`]
///
/// Examples
/// ```no_run
/// // Deposit amd Withdrawal auto-increment tx
/// transact!(1, 1, 1.0, TransactionType::Deposit);
/// transact!(1, 2, 1.0, TransactionType::Withdrawal);
///
/// // Other transaction types do not change tx
/// transact!(1, 3, TransactionType::Dispute);
/// transact!(1, 4, TransactionType::Resolve);
/// transact!(1, 5, TransactionType::Chargeback);
/// ```
macro_rules! transact {
    // a - is for accounts_db
    // t - is for transactions_db
    ($a: expr, $t: expr, $client: expr, $tx: expr, $amount: expr, $type_: expr) => {
        $tx += 1;
        process_transaction(
            &mut $a,
            &mut $t,
            Transaction {
                client: $client,
                tx: $tx,
                amount: Some($amount),
                type_: $type_,
            },
        )
    };
    // a - is for accounts_db
    // t - is for transactions_db
    ($a: expr, $t: expr, $client: expr, $tx: expr, $type_: expr) => {
        process_transaction(
            &mut $a,
            &mut $t,
            Transaction {
                client: $client,
                tx: $tx,
                amount: None,
                type_: $type_,
            },
        )
    };
}

/// Macro to get an [`Account`]
///
/// Examples
/// ```no_run
/// let account = account_of!(accounts_db, 1);
/// ```
macro_rules! account_of {
    ($a: expr, $client: expr) => {
        $a.get(&$client).unwrap()
    };
}

#[test]
fn test_given_example() {
    let mut a = AccountDB::new();
    let mut t = TransactionDB::new();
    let mut tx = 0;
    transact!(a, t, 1, tx, 1.0, TransactionType::Deposit);
    transact!(a, t, 2, tx, 2.0, TransactionType::Deposit);
    transact!(a, t, 1, tx, 2.0, TransactionType::Deposit);
    transact!(a, t, 1, tx, 1.5, TransactionType::Withdrawal);
    transact!(a, t, 2, tx, 3.0, TransactionType::Withdrawal);
    assert_eq!(account_of!(a, 1).available, 1.5);
    assert_eq!(account_of!(a, 1).held, 0.0);
    assert_eq!(account_of!(a, 1).total, 1.5);
    assert_eq!(account_of!(a, 1).locked, false);
    assert_eq!(account_of!(a, 2).available, 2.0);
    assert_eq!(account_of!(a, 2).held, 0.0);
    assert_eq!(account_of!(a, 2).total, 2.0);
    assert_eq!(account_of!(a, 2).locked, false);
}

#[test]
fn test_deposits_and_withdrawals_for_three_clients() {
    let mut a = AccountDB::new();
    let mut t = TransactionDB::new();
    let mut tx = 0;
    transact!(a, t, 1, tx, 3.0, TransactionType::Deposit);
    assert_eq!(account_of!(a, 1).available, 3.0);
    assert_eq!(account_of!(a, 1).total, 3.0);

    transact!(a, t, 1, tx, 3.0, TransactionType::Withdrawal);
    assert_eq!(account_of!(a, 1).available, 0.0);
    assert_eq!(account_of!(a, 1).total, 0.0);

    transact!(a, t, 2, tx, 1.0, TransactionType::Deposit);
    assert_eq!(account_of!(a, 2).available, 1.0);
    assert_eq!(account_of!(a, 2).total, 1.0);

    transact!(a, t, 2, tx, 1.0, TransactionType::Withdrawal);
    assert_eq!(account_of!(a, 2).available, 0.0);
    assert_eq!(account_of!(a, 2).total, 0.0);

    transact!(a, t, 3, tx, 1.0, TransactionType::Deposit);
    assert_eq!(account_of!(a, 3).available, 1.0);
    assert_eq!(account_of!(a, 3).total, 1.0);

    transact!(a, t, 3, tx, 1.0, TransactionType::Withdrawal);
    assert_eq!(account_of!(a, 3).available, 0.0);
    assert_eq!(account_of!(a, 3).total, 0.0);
}

#[test]
fn test_dispute() {
    let mut a = AccountDB::new();
    let mut t = TransactionDB::new();
    let mut tx = 0;

    transact!(a, t, 1, tx, 1.0, TransactionType::Deposit);
    assert_eq!(account_of!(a, 1).available, 1.0);
    assert_eq!(account_of!(a, 1).total, 1.0);

    transact!(a, t, 1, 1, TransactionType::Dispute);
    assert_eq!(account_of!(a, 1).available, 0.0);
    assert_eq!(account_of!(a, 1).held, 1.0);
    assert_eq!(account_of!(a, 1).total, 1.0);
}

#[test]
fn test_resolve() {
    let mut a = AccountDB::new();
    let mut t = TransactionDB::new();
    let mut tx = 0;

    transact!(a, t, 1, tx, 1.0, TransactionType::Deposit);
    transact!(a, t, 1, 1, TransactionType::Dispute);
    transact!(a, t, 1, 1, TransactionType::Resolve);
    assert_eq!(account_of!(a, 1).available, 1.0);
    assert_eq!(account_of!(a, 1).held, 0.0);
    assert_eq!(account_of!(a, 1).total, 1.0);
}

#[test]
fn test_chargeback() {
    let mut a = AccountDB::new();
    let mut t = TransactionDB::new();
    let mut tx = 0;

    transact!(a, t, 1, tx, 1.0, TransactionType::Deposit);
    assert_eq!(account_of!(a, 1).available, 1.0);
    assert_eq!(account_of!(a, 1).total, 1.0);

    transact!(a, t, 1, 1, TransactionType::Dispute);
    assert_eq!(account_of!(a, 1).available, 0.0);
    assert_eq!(account_of!(a, 1).held, 1.0);

    transact!(a, t, 1, 1, TransactionType::Chargeback);
    assert_eq!(account_of!(a, 1).held, 0.0);
    assert_eq!(account_of!(a, 1).total, 0.0);
    assert_eq!(account_of!(a, 1).locked, true);
}

#[test]
fn test_dispute_missing_transaction() {
    let mut a = AccountDB::new();
    let mut t = TransactionDB::new();
    let mut tx = 0;

    transact!(a, t, 1, tx, 42.0, TransactionType::Deposit);
    assert_eq!(account_of!(a, 1).available, 42.0);
    assert_eq!(account_of!(a, 1).total, 42.0);

    transact!(a, t, 1, 2, TransactionType::Dispute);
    assert_eq!(account_of!(a, 1).available, 42.0);
    assert_eq!(account_of!(a, 1).held, 0.0);
    assert_eq!(account_of!(a, 1).total, 42.0);
}

#[test]
fn test_resolve_undisputed_transaction() {
    let mut a = AccountDB::new();
    let mut t = TransactionDB::new();
    let mut tx = 0;

    transact!(a, t, 1, tx, 42.0, TransactionType::Deposit);
    assert_eq!(account_of!(a, 1).available, 42.0);
    assert_eq!(account_of!(a, 1).total, 42.0);

    transact!(a, t, 1, 1, TransactionType::Resolve);
    assert_eq!(account_of!(a, 1).available, 42.0);
    assert_eq!(account_of!(a, 1).held, 0.0);
    assert_eq!(account_of!(a, 1).total, 42.0);
}

#[test]
fn test_chargeback_undisputed_transaction() {
    let mut a = AccountDB::new();
    let mut t = TransactionDB::new();
    let mut tx = 0;

    transact!(a, t, 1, tx, 42.0, TransactionType::Deposit);
    assert_eq!(account_of!(a, 1).available, 42.0);
    assert_eq!(account_of!(a, 1).total, 42.0);

    transact!(a, t, 1, 1, TransactionType::Chargeback);
    assert_eq!(account_of!(a, 1).available, 42.0);
    assert_eq!(account_of!(a, 1).held, 0.0);
    assert_eq!(account_of!(a, 1).total, 42.0);
    assert_eq!(account_of!(a, 1).locked, false);
}

// Are dispute, resolve and chargeback allowed to turn some balances negative?
#[test]
fn test_dispute_with_negative_available_balance() {
    let mut a = AccountDB::new();
    let mut t = TransactionDB::new();
    let mut tx = 0;
    transact!(a, t, 1, tx, 42.0, TransactionType::Deposit);
    assert_eq!(account_of!(a, 1).available, 42.0);
    assert_eq!(account_of!(a, 1).total, 42.0);

    transact!(a, t, 1, tx, 1.0, TransactionType::Withdrawal);
    assert_eq!(account_of!(a, 1).available, 41.0);
    assert_eq!(account_of!(a, 1).total, 41.0);

    transact!(a, t, 1, 1, TransactionType::Dispute);
    assert_eq!(account_of!(a, 1).available, -1.0);
    assert_eq!(account_of!(a, 1).held, 42.0);
    assert_eq!(account_of!(a, 1).total, 41.0);
}

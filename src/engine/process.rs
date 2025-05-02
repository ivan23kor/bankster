use crate::engine::account_db::AccountDB;
use crate::engine::transaction_db::{TransactionDB, TransactionDBEntry};
use crate::{
    account::Account,
    transaction::{ClientID, FundsType, Transaction, TransactionID, TransactionType},
};

/// Processes a single transaction and updates the account and transaction databases
pub fn process_transaction(
    accounts_db: &mut AccountDB,
    transactions_db: &mut TransactionDB,
    transaction: Transaction,
) {
    // Look up existing or start a new account for this client id.
    let account = accounts_db
        .entry(transaction.client)
        .or_insert_with_key(|client: &ClientID| Account::new(*client));

    match transaction.type_ {
        // Increment available and total funds by transaction amount
        TransactionType::Deposit => {
            transactions_db.insert(transaction.tx, TransactionDBEntry::new(transaction.clone()));
            if let Some(amount) = transaction.amount {
                account.available += amount;
                account.total += amount;
            }
        }
        // Decrease available and total funds by transaction amount
        TransactionType::Withdrawal => {
            transactions_db.insert(transaction.tx, TransactionDBEntry::new(transaction.clone()));
            if let Some(amount) = transaction.amount {
                if amount <= account.available {
                    account.available -= amount;
                    account.total -= amount;
                }
            }
        }
        // Only if referenced transaction is found in transactions_db, increase held and decrease available funds
        TransactionType::Dispute => {
            if let Some(amount) = dispute_transaction(transactions_db, transaction.tx) {
                account.held += amount;
                account.available -= amount;
            }
        }
        // Only if referenced transaction is found in transactions_db, and is a dispute, decrease held and increase available funds
        TransactionType::Resolve => {
            if let Some(amount) = resolve_dispute(transactions_db, transaction.tx) {
                account.held -= amount;
                account.available += amount;
            }
        }
        // Only if referenced transaction is found in transactions_db, and is a dispute, decrease held and total funds and freeze this account
        TransactionType::Chargeback => {
            if let Some(amount) = resolve_dispute(transactions_db, transaction.tx) {
                account.held -= amount;
                account.total -= amount;
                account.locked = true;
            }
        }
    }
}

/// Looks up transaction by [`TransactionId`] in "transactions_db".
/// If found, marks that transaction as under dispute and returns amount of that transaction.
///
/// Otherwise, returns [`None`].
fn dispute_transaction(
    transactions_db: &mut TransactionDB,
    tx: TransactionID,
) -> Option<FundsType> {
    transactions_db.get_mut(&tx).and_then(|entry| {
        entry.under_dispute = true;
        entry.transaction.amount
    })
}

/// Looks up transaction under dispute by [`TransactionId`] in "transactions_db"
/// If found, marks that transaction as no longer under dispute and returns amount of that transaction.
///
/// Otherwise, returns [`None`].
fn resolve_dispute(transactions_db: &mut TransactionDB, tx: TransactionID) -> Option<FundsType> {
    let entry = transactions_db.get_mut(&tx)?;
    if entry.under_dispute {
        entry.under_dispute = false;
        entry.transaction.amount
    } else {
        None
    }
}

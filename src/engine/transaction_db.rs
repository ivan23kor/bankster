use std::collections::HashMap;

use crate::transaction::{Transaction, TransactionID};

/// Keeps track of  transaction and whether it is under dispute.
pub struct TransactionDBEntry {
    pub transaction: Transaction,
    pub under_dispute: bool,
}

impl TransactionDBEntry {
    pub fn new(transaction: Transaction) -> Self {
        Self {
            transaction,
            under_dispute: false,
        }
    }
}

/// Keeps track of [`crate::transaction::TransactionType::Deposit`] and [`crate::transaction::TransactionType::Withdrawal`]
/// transactions to be able to look them up, e.g. for dispute resolution.
pub type TransactionDB = HashMap<TransactionID, TransactionDBEntry>;

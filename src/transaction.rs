/// Type definitions for transactions
use serde::Deserialize;
use std::fmt::Debug;

/// Transaction types, serialized as defined in the CSV header
#[derive(Copy, Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

pub type ClientID = u16;
pub type TransactionID = u32;
/// Transaction amount has up to four decimal places
/// f16 has only 5 bits for exponent, making max value of ~65000 too low for representing some transactions
/// f32 has 8 bits for exponent and 23 bits for precision, making it enough for max value and 4 decimal places
pub type FundsType = f32;

/// Transaction record, with serialized fields as defined in the CSV header
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Transaction {
    #[serde(rename = "type")]
    pub type_: TransactionType,
    pub client: ClientID,
    pub tx: TransactionID,
    pub amount: Option<FundsType>,
}

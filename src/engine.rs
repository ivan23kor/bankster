/// This module provides functionality to process transactions.
/// See [`process_transaction`] for more details.
pub mod account_db;
pub mod process;
pub mod transaction_db;

pub use account_db::{AccountDB, write_accounts_db_as_csv};
pub use process::process_transaction;
pub use transaction_db::TransactionDB;

#[cfg(test)]
mod tests;

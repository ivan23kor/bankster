mod cli;
mod streaming_client;

use bankster::engine::{AccountDB, TransactionDB, process_transaction, write_accounts_db_as_csv};
use clap::Parser;

use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

/// TODO: If there are multiple concurrent CSVs, use tokio threads to stream them from files.
/// How to resolve order of transactions in those CSVs?
#[tokio::main]
async fn main() -> Result<(), String> {
    let (tx, mut rx) = mpsc::channel(32);

    // Start streaming transactions from CSV
    tokio::spawn(async move {
        let csv_filepath = cli::Args::parse().csv_filepath;
        let _ = streaming_client::stream_file_to_channel(&csv_filepath, tx).await;
    });

    // Initialize databases
    let accounts_db = Arc::new(Mutex::new(AccountDB::new()));
    let transactions_db = Arc::new(Mutex::new(TransactionDB::new()));
    let accounts_db_clone = accounts_db.clone();

    while let Some(transaction) = rx.recv().await {
        process_transaction(
            &mut accounts_db.lock().unwrap(),
            &mut transactions_db.lock().unwrap(),
            transaction,
        );
    }

    write_accounts_db_as_csv(&accounts_db_clone.lock().unwrap(), &mut std::io::stdout());

    Ok(())
}

// fn main() -> Result<(), std::io::Error> {
//     let csv_filepath = cli::Args::parse().csv_filepath;

//     // Gracefully handle IO errors
//     let file = std::fs::File::open(&csv_filepath)?;

//     // Initialize databases
//     let mut accounts_db = AccountDB::new();
//     let mut transactions_db = TransactionDB::new();

//     // Receive transactions from CSV
//     let transactions = stream_csv(file).filter_map(Result::ok);
//     for transaction in transactions {
//         process_transaction(&mut accounts_db, &mut transactions_db, transaction);
//     }

//     write_accounts_db_as_csv(&accounts_db, &mut std::io::stdout());
//     Ok(())
// }

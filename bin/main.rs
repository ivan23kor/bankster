mod cli;

use bankster::{
    engine::{AccountDB, TransactionDB, process_transaction, write_accounts_db_as_csv},
    stream::stream_csv,
};

// use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
// use std::sync::{Arc, Mutex};
// use tokio::net::{TcpListener, TcpStream};

use clap::Parser;

// type DB = Arc<Mutex<HashMap<u16, Account>>>;

// /// TODO: If there are multiple concurrent CSVs, use tokio threads to stream them from files.
// /// How to resolve order of transactions in those CSVs?
// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let socket = TcpStream::connect("127.0.0.1:6142").await?;
//     let (mut rd, mut wr) = io::split(socket);

//     // Write data in the background
//     tokio::spawn(async move {
//         wr.write_all(b"hello\r\n").await?;
//         wr.write_all(b"world\r\n").await?;

//         // Sometimes, the rust type inferencer needs
//         // a little help
//         Ok::<_, io::Error>(())
//     });

//     let mut buf = vec![0; 128];

//     loop {
//         let n = rd.read(&mut buf).await?;

//         if n == 0 {
//             break;
//         }

//         println!("GOT {:?}", &buf[..n]);
//     }

//     Ok(())
// }

fn main() -> Result<(), std::io::Error> {
    let csv_filepath = cli::Args::parse().csv_filepath;

    // Gracefully handle IO errors
    let file = std::fs::File::open(&csv_filepath)?;

    // Initialize databases
    let mut accounts_db = AccountDB::new();
    let mut transactions_db = TransactionDB::new();

    // Receive transactions from CSV
    let transactions = stream_csv(file).filter_map(Result::ok);
    for transaction in transactions {
        println!("{:?}", transaction);
        process_transaction(&mut accounts_db, &mut transactions_db, transaction);
    }

    write_accounts_db_as_csv(&accounts_db, &mut std::io::stdout());
    Ok(())
}

use bankster::{stream::stream_csv, transaction::Transaction};

/// Stream transactions from CSV file to channel
///
/// Returns early on IO or send error.
pub(crate) async fn stream_file_to_channel(
    filepath: &str,
    channel: tokio::sync::mpsc::Sender<Transaction>,
) -> Result<(), String> {
    let file = std::fs::File::open(filepath).map_err(|e| e.to_string())?;

    // Receive transactions from CSV
    let transactions = stream_csv(file).filter_map(Result::ok);
    for transaction in transactions {
        if let Err(error) = channel.send(transaction).await {
            return Err(error.to_string());
        }
    }

    Ok(())
}

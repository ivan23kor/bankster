/// This module provides functionality to stream CSV file line by line
use crate::transaction::Transaction;

/// Streams csv records line by line
/// Example:
/// ```no_run
/// for record in stream_csv(std::fs::File::open("file.csv").unwrap()) {
///     println!("{:?}", record);
/// }
/// ```
pub fn stream_csv<R>(readable: R) -> csv::DeserializeRecordsIntoIter<R, Transaction>
where
    R: std::io::Read,
{
    csv::ReaderBuilder::new()
        .trim(csv::Trim::All) // Trim whitespace from all fields
        .has_headers(true)
        .from_reader(readable)
        .into_deserialize()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transaction::TransactionType;

    #[test]
    fn test_stream_csv_with_trailing_whitespace() {
        let data = b"   type  , client,   tx  , amount
        deposit,1,1,42
        deposit,2,2,42
 deposit,    3,3,42
withdrawal,1,  4    ,2
    dispute,   2,5  ,  
            resolve,3,6,
        chargeback    ,1,2,";
        let transactions = stream_csv(&data[..]).collect::<Vec<_>>();
        assert_eq!(transactions.len(), 7);
        assert_eq!(
            transactions[0].as_ref().unwrap(),
            &Transaction {
                type_: TransactionType::Deposit,
                client: 1,
                tx: 1,
                amount: Some(42.0)
            }
        );
        assert_eq!(
            transactions[1].as_ref().unwrap(),
            &Transaction {
                type_: TransactionType::Deposit,
                client: 2,
                tx: 2,
                amount: Some(42.0)
            }
        );
        assert_eq!(
            transactions[2].as_ref().unwrap(),
            &Transaction {
                type_: TransactionType::Deposit,
                client: 3,
                tx: 3,
                amount: Some(42.0)
            }
        );
        assert_eq!(
            transactions[3].as_ref().unwrap(),
            &Transaction {
                type_: TransactionType::Withdrawal,
                client: 1,
                tx: 4,
                amount: Some(2.0)
            }
        );
        assert_eq!(
            transactions[4].as_ref().unwrap(),
            &Transaction {
                type_: TransactionType::Dispute,
                client: 2,
                tx: 5,
                amount: None
            }
        );
        assert_eq!(
            transactions[5].as_ref().unwrap(),
            &Transaction {
                type_: TransactionType::Resolve,
                client: 3,
                tx: 6,
                amount: None,
            }
        );
        assert_eq!(
            transactions[6].as_ref().unwrap(),
            &Transaction {
                type_: TransactionType::Chargeback,
                client: 1,
                tx: 2,
                amount: None
            }
        );
    }

    #[test]
    #[should_panic(
        expected = r#"called `Result::unwrap()` on an `Err` value: Error(Deserialize { pos: Some(Position { byte: 38, line: 2, record: 1 }), err: DeserializeError { field: None, kind: Message("missing field `type`") } })"#
    )]
    fn test_malformed_csv() {
        let data = b"   tpe  , c2lient,   t!@##x  , amount
        depositor,1,1,42";

        let transactions = stream_csv(&data[..]).collect::<Vec<_>>();
        assert_eq!(transactions.len(), 1);
        assert_eq!(
            transactions[0].as_ref().unwrap(),
            &Transaction {
                client: 1,
                tx: 1,
                type_: TransactionType::Deposit,
                amount: Some(42.0)
            }
        );
    }
}

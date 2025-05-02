# Usage
```bash
cargo run -- transactions.csv > accounts.csv
```

# Code structure

```
src/
├── account.rs              <- Type representation of a single client account
├── engine
│   ├── account_db.rs       <- Hashmap between client ids and account structure above + output account DB as csv
│   ├── process.rs          <- Transaction processing logic
│   ├── tests.rs            <- Tests for transaction processing logic
│   └── transaction_db.rs   <- Hashmap between transaction ids and transaction structure defined in transaction.rs
├── engine.rs               <- Engine module
├── lib.rs                  <- Library module
├── stream.rs               <- CSV streaming logic and tests
└── transaction.rs          <- Type representation of a single transaction
```

# Input validation
`bankster` uses Rust `enum`s and `struct`s + `serde::Deserialize` for CSV parsing.

Additionally, `csv::ReaderBuilder` uses `csv::Trim::ALL` to trim trailing whitespaces from all fields.

CSV streaming is tested in [stream::tests](src/stream.rs#L22-L110)

Transaction processing is tested in [engine::process::tests](src/engine/process/tests.rs)


# TODO
1. Whenever I was not sure how to handle erroneous cases of Option and Result, I was skipping erroneous cases and handled only successful cases without printing error messages.
2. What if "client" - "tx" for a Resolve or Dispute transaction does not correspond to a previous transaction? For that reason, I was only comparing transaction ids for dispute, resolve and chargeback.

# Resources
1. https://tokio.rs/tokio/tutorial/shared-state
2. docs.rs
3. SO

(c) Ivan Korostelev 2025
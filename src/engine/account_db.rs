use crate::account::Account;
use crate::transaction::ClientID;
use std::collections::HashMap;

/// Keeps track of account funds and locked status
pub type AccountDB = HashMap<ClientID, Account>;

/// Writes the account database in CSV format
pub fn write_accounts_db_as_csv(account_db: &AccountDB, writer: &mut impl std::io::Write) {
    let mut writer = csv::Writer::from_writer(writer);
    for account in account_db.values() {
        let _ = writer.serialize(account);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_as_csv() {
        let mut writer = Vec::new();
        let mut accounts_db = AccountDB::new();
        accounts_db.insert(1, Account::new(1));
        write_accounts_db_as_csv(&accounts_db, &mut writer);

        #[cfg(windows)]
        let newline = "\r\n";
        #[cfg(not(windows))]
        let newline = "\n";

        assert_eq!(
            String::from_utf8(writer).unwrap(),
            format!(
                "client,available,held,total,locked{newline}1,0.0,0.0,0.0,false{newline}",
                newline = newline
            )
        );
    }
}

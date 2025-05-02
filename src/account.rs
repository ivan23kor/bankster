/// Type definition for [`Account`]
use crate::transaction::{ClientID, FundsType};

/// Keeps track of account funds and locked status for a single client.
///
/// Uses [`FundsType`] for numerical values for the reasons explained in its definition ([`FundsType`])
#[derive(Debug, serde::Serialize)]
pub struct Account {
    pub client: ClientID,
    pub available: FundsType,
    pub held: FundsType,
    pub total: FundsType,
    pub locked: bool,
}

impl Account {
    pub fn new(client: ClientID) -> Self {
        Self {
            client,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        }
    }
}

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FlatEntries {
    pub key: String,
    pub value: String,
}

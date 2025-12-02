use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Debug, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct PasswordEntry {
    #[zeroize(skip)] 
    pub id: Option<i64>,
    pub service: String,
    pub username: String,
    pub encrypted_payload: Vec<u8>, 
    pub nonce: Vec<u8>,
}
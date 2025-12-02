use thiserror::Error;

#[derive(Error, Debug)]
pub enum HaspError {
    #[error("Database error: {0}")]
    DbError(#[from] rusqlite::Error),

    #[error("Encryption error: {0}")]
    CryptoError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Entry not found: {0}")]
    NotFound(String),

    #[error("Decryption failed (Wrong password?)")]
    DecryptionFailed,
}
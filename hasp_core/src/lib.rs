pub mod errors;
pub mod models;

pub use errors::HaspError;
pub use models::PasswordEntry;

pub type HaspResult<T> = Result<T, HaspError>;
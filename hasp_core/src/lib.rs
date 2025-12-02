pub mod errors;
pub mod models;
pub mod db;

pub use errors::HaspError;
pub use models::PasswordEntry;
pub use db::HaspDatabase;

pub type HaspResult<T> = Result<T, HaspError>;
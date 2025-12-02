use std::path::Path;
use rusqlite::{Connection, params};
use crate::HaspResult;
use crate::errors::HaspError;
use crate::models::PasswordEntry;


pub struct HaspDatabase {
    conn: Connection,
}

impl HaspDatabase {
    pub fn new<P: AsRef<Path>>(path: P) -> HaspResult<Self> {
        let conn = Connection::open(path).map_err(HaspError::DbError)?;
        
        Ok(Self { conn })
    }

    pub fn init(&self) -> HaspResult<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS passwords (
                id                  INTEGER PRIMARY KEY,
                service             TEXT NOT NULL UNIQUE,
                username            TEXT NOT NULL,
                encrypted_payload   BLOB NOT NULL,
                nonce               BLOB NOT NULL
            )",
            [], 
        )?; 
        
        Ok(())
    }


    pub fn add_entry(&self, entry: &PasswordEntry) -> HaspResult<()> {
        self.conn.execute(
            "INSERT INTO passwords (service, username, encrypted_payload, nonce)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                entry.service, 
                entry.username, 
                entry.encrypted_payload, 
                entry.nonce
            ],
        )?;
        Ok(())
    }


    pub fn get_entry(&self, service: &str) -> HaspResult<PasswordEntry> {
        let entry = self.conn.query_row(
            "SELECT id, service, username, encrypted_payload, nonce 
             FROM passwords WHERE service = ?1",
            params![service],
            |row| {
                Ok(PasswordEntry {
                    id: row.get(0)?,
                    service: row.get(1)?,
                    username: row.get(2)?,
                    encrypted_payload: row.get(3)?,
                    nonce: row.get(4)?,
                })
            }
        ).map_err(|e| {
            match e {
                rusqlite::Error::QueryReturnedNoRows => HaspError::NotFound(service.to_string()),
                _ => HaspError::DbError(e),
            }
        })?;

        Ok(entry)
    }
}
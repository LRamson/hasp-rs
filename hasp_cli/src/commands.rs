use hasp_core::db::HaspDatabase;
use hasp_core::models::PasswordEntry;
use hasp_core::{crypto, HaspResult};
use std::io::{self, Write};

// A helper to ask the user for input
fn prompt(message: &str) -> HaspResult<String> {
    print!("{}", message);
    io::stdout().flush()?; // Ensure the prompt appears before input
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

// A helper to ask for the MASTER password securely
fn prompt_password(message: &str) -> HaspResult<String> {
    print!("{}", message);
    io::stdout().flush()?;
    // rpassword handles the "*****" masking
    let password = rpassword::read_password()
        .map_err(|e| hasp_core::HaspError::IoError(e))?;
    Ok(password)
}

pub fn init() -> HaspResult<()> {
    let db = HaspDatabase::new("passwords.db")?;
    db.init()?;
    println!("Store initialized (passwords.db created).");
    Ok(())
}

pub fn add(service: String, username: String) -> HaspResult<()> {
    // 1. Get the password to be saved
    let password_to_save = prompt_password(&format!("Enter password for [{}]: ", service))?;

    // 2. Get the Master Password to encrypt it
    let master_pass = prompt_password("Enter your MASTER password: ")?;
    
    // 3. Derive the key (Heavy lifting)
    println!("Encrypting...");
    let key = crypto::derive_key(&master_pass)?;

    // 4. Encrypt
    let (encrypted_payload, nonce) = crypto::encrypt(&password_to_save, &key)?;

    // 5. Save to DB
    let entry = PasswordEntry {
        id: None,
        service,
        username,
        encrypted_payload,
        nonce,
    };

    let db = HaspDatabase::new("passwords.db")?;
    db.add_entry(&entry)?;
    
    println!("Password saved successfully! 🔒");
    Ok(())
}

pub fn get(service: String) -> HaspResult<()> {
    let db = HaspDatabase::new("passwords.db")?;
    
    // 1. Find the entry first (fast)
    let entry = db.get_entry(&service)?;

    // 2. If found, ask for Master Password
    let master_pass = prompt_password("Enter your MASTER password to unlock: ")?;
    
    // 3. Derive key
    let key = crypto::derive_key(&master_pass)?;

    // 4. Decrypt
    let password = crypto::decrypt(&entry.encrypted_payload, &entry.nonce, &key)?;

    println!("--------------------------------");
    println!("Service:  {}", entry.service);
    println!("Username: {}", entry.username);
    println!("Password: {}", password);
    println!("--------------------------------");

    Ok(())
}
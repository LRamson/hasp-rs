use crate::errors::HaspError;
use crate::HaspResult;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce
};
use argon2::{
    password_hash::{rand_core::RngCore, PasswordHasher, SaltString},
    Argon2,
};
use zeroize::Zeroize;

pub fn derive_key(password: &str) -> HaspResult<[u8; 32]> {
    let mut key = [0u8; 32];
    
    let salt_str = "somesuperrandomsaltstringformysupersecureapp"; 
    let salt = SaltString::from_b64(salt_str).map_err(|e| HaspError::CryptoError(e.to_string()))?;

    // Setup Argon2 with recommended security parameters
    let argon2 = Argon2::default();
    
    // Hash the password
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)
        .map_err(|e| HaspError::CryptoError(e.to_string()))?;

    // Extract the raw hash bytes to use as an AES key
    let hash = password_hash.hash.ok_or(HaspError::CryptoError("Hash failed".into()))?;
    
    // Copy hash into our key array (ensuring it fits 32 bytes)
    let src = hash.as_bytes();
    let len = src.len().min(32);
    key[..len].copy_from_slice(&src[..len]);

    Ok(key)
}

/// 2. Encryption
/// Takes the Master Key and the Secret Data, returns (Encrypted Data, Nonce)
pub fn encrypt(data: &str, key: &[u8; 32]) -> HaspResult<(Vec<u8>, Vec<u8>)> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    
    // Generate a unique random "number used once" (Nonce)
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
    
    // Encrypt the data
    let ciphertext = cipher.encrypt(&nonce, data.as_bytes())
        .map_err(|e| HaspError::CryptoError(e.to_string()))?;

    Ok((ciphertext, nonce.to_vec()))
}

/// 3. Decryption
/// Takes the Master Key, Encrypted Data, and Nonce -> returns Plain Text
pub fn decrypt(ciphertext: &[u8], nonce: &[u8], key: &[u8; 32]) -> HaspResult<String> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Nonce::from_slice(nonce);

    let plaintext_bytes = cipher.decrypt(nonce, ciphertext)
        .map_err(|_| HaspError::DecryptionFailed)?;

    let plaintext = String::from_utf8(plaintext_bytes)
        .map_err(|e| HaspError::CryptoError(e.to_string()))?;

    Ok(plaintext)
}
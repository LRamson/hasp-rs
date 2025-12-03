use crate::errors::HaspError;
use crate::HaspResult;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce
};
use argon2::{
    password_hash::{ PasswordHasher, SaltString},
    Argon2,
};

pub fn derive_key(password: &str) -> HaspResult<[u8; 32]> {
    let mut key = [0u8; 32];
    
    let salt_str = "somesuperrandomsaltstringformysupersecureapp"; 
    let salt = SaltString::from_b64(salt_str).map_err(|e| HaspError::CryptoError(e.to_string()))?;

    let argon2 = Argon2::default();
    
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)
        .map_err(|e| HaspError::CryptoError(e.to_string()))?;

    let hash = password_hash.hash.ok_or(HaspError::CryptoError("Hash failed".into()))?;
    
    let src = hash.as_bytes();
    let len = src.len().min(32);
    key[..len].copy_from_slice(&src[..len]);

    Ok(key)
}


pub fn encrypt(data: &str, key: &[u8; 32]) -> HaspResult<(Vec<u8>, Vec<u8>)> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); 
    

    let ciphertext = cipher.encrypt(&nonce, data.as_bytes())
        .map_err(|e| HaspError::CryptoError(e.to_string()))?;

    Ok((ciphertext, nonce.to_vec()))
}


pub fn decrypt(ciphertext: &[u8], nonce: &[u8], key: &[u8; 32]) -> HaspResult<String> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Nonce::from_slice(nonce);

    let plaintext_bytes = cipher.decrypt(nonce, ciphertext)
        .map_err(|_| HaspError::DecryptionFailed)?;

    let plaintext = String::from_utf8(plaintext_bytes)
        .map_err(|e| HaspError::CryptoError(e.to_string()))?;

    Ok(plaintext)
}
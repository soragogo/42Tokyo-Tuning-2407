use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand::Rng;

use crate::errors::AppError;
use bcrypt::{hash, DEFAULT_COST};
const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                         abcdefghijklmnopqrstuvwxyz\
                         0123456789";

pub fn generate_session_token() -> String {
    let mut rng = rand::thread_rng();
    let token: String = (0..30)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    token
}



pub fn hash_password(password: &str) -> Result<String, AppError> {
    match hash(password, DEFAULT_COST) {
        Ok(hashed_password) => Ok(hashed_password),
        Err(_) => Err(AppError::InternalServerError),
    }
}

pub fn verify_password(hashed_password: &str, input_password: &str) -> Result<bool, AppError> {
    let input_password_bytes = input_password.as_bytes();
    let parsed_hash = match PasswordHash::new(hashed_password) {
        Ok(hash) => hash,
        Err(_) => return Err(AppError::InternalServerError),
    };
    match Argon2::default().verify_password(input_password_bytes, &parsed_hash) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

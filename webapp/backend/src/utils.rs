use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand::Rng;
use md5;

use crate::errors::AppError;

pub fn generate_session_token() -> String {
    let mut rng = rand::thread_rng();
    let token: String = (0..30)
        .map(|_| {
            let idx = rng.gen_range(0..62);
            let chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
            chars[idx] as char
        })
        .collect();
    token
}

pub fn hash_password(password: &str) -> Result<String, AppError> {

    let password_bytes = password.as_bytes();
    let digest = md5::compute(password_bytes);

    //let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    //let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    //match argon2.hash_password(password_bytes, &salt) {
        //Ok(hashed_password_bytes) => Ok(hashed_password_bytes.to_string()),
        //Err(_) => Err(AppError::InternalServerError),
    //}

    //match md5::compute(password_bytes) {
        //Ok(digest) => Ok(digest.to_string()),
        //Err(_) => Err(AppError::InternalServerError),
    //}
    //format!("{:x}", digest)
    Ok(format!("{:x}", digest))
}

pub fn verify_password(hashed_password: &str, input_password: &str) -> Result<bool, AppError> {

    let password_bytes = input_password.as_bytes();
    let digest = md5::compute(password_bytes);


    match hashed_password == format!("{:x}", digest) {
        true => Ok(true),
        false => Ok(false),
    }
}
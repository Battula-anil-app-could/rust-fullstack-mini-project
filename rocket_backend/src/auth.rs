use argon2::password_hash::Error;
use rand::{distributions::Alphanumeric, Rng};
use crate::{modules::User};
use argon2::{PasswordHash, PasswordVerifier};
use rocket_db_pools::*;
use argon2::{password_hash::{rand_core::OsRng, SaltString}, PasswordHasher};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Credentials{
    pub username: String,
    pub password: String,
}


pub fn authorize_user(user: &User, credentials: Credentials) -> Result<User, Error>{
    let argon2 = argon2::Argon2::default();
    let db_password = PasswordHash::new(&user.password)?;
    argon2.verify_password(credentials.password.as_bytes(), &db_password)?;
    Ok(user.clone())
}


pub fn password_hash(password: String) -> Result<String, Error>{

    let salt = SaltString::generate(OsRng);
    let password_has = argon2::Argon2::default().hash_password(password.as_bytes(), &salt)?;
    Ok(password_has.to_string())
}
use std::error::Error;
use std::io::Read;
use crypto_common::{Key, KeyInit};
use hmac::HmacCore;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use sha2::Sha256;
use uuid::Uuid;
use crypto_common::rand_core::{OsRng};
use argon2::{self, Config};

use crate::comm::keys::Salt;

use super::crypto::{HmacSha256};
use super::keys::SessionToken;
use super::queries::{self};

pub fn login_test(username: String, password: String, hash: &String) -> Result<SessionToken, Box<dyn Error>> {
    match argon2::verify_encoded(&hash.to_owned(), password.as_bytes()) {
        Ok(_) => Ok(SessionToken::new(username)),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn login(username: String, password: String,) -> Result<SessionToken, Box<dyn Error>> {
    match argon2::verify_encoded("", password.as_bytes()) {
        Ok(_) => Ok(SessionToken::new(username)),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn signup(username: String, password: String) -> Result<String, Box<dyn Error>> {
    let rng = OsRng::default();
    let session_key: Key<HmacSha256> =  HmacSha256::generate_key(rng);
    println!("Key Size: {:#?}", session_key.len());
    let password = password.as_bytes();
    let salt = Salt::generate_key(rng);
    let config = build_config();
    println!("SALT: {:#?}", &salt.len());
    let hash = argon2::hash_encoded(password, &salt, &config)?;
    Ok(hash)
}

fn build_config<'a>() -> argon2::Config<'a> {
    Config {
        variant: argon2::Variant::Argon2id,
        version: argon2::Version::Version13,
        mem_cost: 65536,
        time_cost: 10,
        lanes: 4,
        thread_mode: argon2::ThreadMode::Parallel,
        secret: &[],
        ad: &[],
        hash_length: 32
    }
}

pub fn init_db() -> Result<(), Box<dyn Error>> {
    let connection = sqlite::open("../data/poshmon.db").unwrap();

    connection.execute(queries::CREATE_USER_TABLE_SQL)?;
    Ok(())
}

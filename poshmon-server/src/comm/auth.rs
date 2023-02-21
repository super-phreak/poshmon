use std::error::Error;
use crypto_common::{Key, KeyInit};
use crypto_common::rand_core::{OsRng};
use argon2::{self, Config};

use poshmon_lib::networking::{SessionToken, Salt, HmacSha256};
use super::queries::{self};

const USER_DB: &'static str = "../data/poshmon.sqlite";

pub fn login_test(username: String, password: String, hash: &String) -> Result<SessionToken, Box<dyn Error>> {
    match argon2::verify_encoded(&hash.to_owned(), password.as_bytes()) {
        Ok(_) => Ok(SessionToken::new(username)),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn login(username: String, password: String,) -> Result<SessionToken, Box<dyn Error>> {
    let connection = sqlite::Connection::open_with_full_mutex(USER_DB)?;
    let mut statement = connection.prepare(queries::LOOKUP_USER_SQL)?;
    println!("UN: {}, PW: {}", username, password);
    statement.bind((":username",username.as_str()))?;
    match statement.next() {
        Ok(_) => 
            match argon2::verify_encoded(statement.read::<String, _>("hash")?.as_str(), password.as_bytes()) {
                Ok(_) => Ok(SessionToken::new(username)),
                Err(e) => Err(Box::new(e)),
            },
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
    println!("UN: {:#?} SALT: {:#?}", username, &salt.len());
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
    let connection = sqlite::Connection::open_with_full_mutex(USER_DB)?;
    connection.execute(queries::CREATE_USER_TABLE_SQL)?;
    Ok(())
}

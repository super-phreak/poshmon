use std::error::Error;
use crypto_common::{Key, KeyInit};
use crypto_common::rand_core::{OsRng};
use argon2::{self, Config};

use actix_web::web::{Json, Path, Data, self};
use actix_web::{HttpResponse, post};
use diesel::{QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::dbc::{DbcPool, get_user, create_user};

use poshmon_lib::networking::{SessionToken, Salt, HmacSha256};

use crate::httpconst::{APPLICATION_JSON, CONNECTION_POOL_ERROR};

#[derive(Debug, Deserialize, Serialize)]
struct SessionCookie {
    pkey: String,
    session_id: String
}

impl SessionCookie {
    pub fn new(username: String) -> Self {
        let token = SessionToken::new(username);
        SessionCookie {
            pkey: base64::encode(token.session_key),
            session_id: token.session_id.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    username: String,
    password: String,
}

pub fn login_test(username: String, password: String, hash: &String) -> Result<SessionToken, Box<dyn Error>> {
    match argon2::verify_encoded(&hash.to_owned(), password.as_bytes()) {
        Ok(_) => Ok(SessionToken::new(username)),
        Err(e) => Err(Box::new(e)),
    }
}

#[post("/login")]
pub async fn login(req: Json<Request>, pool: Data<DbcPool>) -> HttpResponse {
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);

    println!("Sent user: {}", req.username);

    let user = web::block(move || get_user(req.username.clone(), &mut conn)).await;

    HttpResponse::Ok()
    .content_type(APPLICATION_JSON)
    .json(SessionCookie::new(user.unwrap().unwrap().username))
}

#[post("/signup")]
pub async fn signup(req: Json<Request>, pool: Data<DbcPool>) -> HttpResponse {
    let mut connection = pool.get().expect(CONNECTION_POOL_ERROR);
    println!("Signup for user: {}", req.username);

    let uname = req.username.clone();

    let rng = OsRng::default();
    let session_key: Key<HmacSha256> =  HmacSha256::generate_key(rng);
    println!("Key Size: {:#?}", session_key.len());
    let password = req.password.as_bytes();
    let salt = Salt::generate_key(rng);
    let config = build_config();
    println!("UN: {:#?} SALT: {:#?}", req.username, &salt.len());

    if let Ok(hash) = argon2::hash_encoded(password, &salt, &config) {

        match web::block(move || create_user(&req.username, hash, &mut connection)).await {
            Ok(result) => {
                if result == 1 {
                    HttpResponse::Ok()
                        .content_type(APPLICATION_JSON)
                        .json(SessionCookie::new(uname.clone()))
                } else {
                    HttpResponse::Unauthorized()
                        .content_type(APPLICATION_JSON)
                        .await
                        .unwrap()
                }
            },
            Err(_) => todo!(),
        }
    } else {
        HttpResponse::InternalServerError()
                        .content_type(APPLICATION_JSON)
                        .await
                        .unwrap()
    }
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


use std::time::SystemTime;
use crypto_common::{Key, KeyInit};
use crypto_common::rand_core::{OsRng};
use argon2::{self, Config};

use actix_web::web::{Json, Data, self};
use actix_web::{HttpResponse, post, get};
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Pong {
    msg: String,
    time: SystemTime,
}

impl Pong {
    pub fn new(msg: String) -> Self {
        Pong { msg, time: SystemTime::now()}
    }
}

#[get("/ping")]
pub async fn ping() -> HttpResponse {
    let pong = Pong::new("pong".to_owned());

    HttpResponse::Ok()
    .content_type(APPLICATION_JSON)
    .json(pong)
}

#[post("/login")]
pub async fn login(req: Json<Request>, pool: Data<DbcPool>) -> HttpResponse {
    //Pulling the password out so the borrow checker doesn't steal it when we hand the request over to the 
    //database to pull the user.
    let password = req.password.clone();

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let user_res = web::block(move || get_user(&req.username.clone(), &mut conn)).await;

    //Need to find a way to bullet-proof the unwraps in the ones where JSON isn't sent back.
    match user_res {
        Ok(user) => {
            match user {
                Ok(user) => {
                    if let Ok(res) = argon2::verify_encoded(&user.hash, password.as_bytes()) {
                        if res {
                            HttpResponse::Ok()
                                .content_type(APPLICATION_JSON)
                                .json(SessionCookie::new(user.username))
                        } else {
                            HttpResponse::Unauthorized()
                                .content_type(APPLICATION_JSON)
                                .await
                                .unwrap()
                        }
                    } else {
                        HttpResponse::InternalServerError()
                            .content_type(APPLICATION_JSON)
                            .await
                            .unwrap()
                    }
                    
                },
                Err(_) => HttpResponse::NotFound()
                            .content_type(APPLICATION_JSON)
                            .await
                            .unwrap(),
            }
        },
        Err(_) => HttpResponse::InternalServerError()
                    .content_type(APPLICATION_JSON)
                    .await
                    .unwrap(), 
    }
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
            Err(_) => HttpResponse::Unauthorized()
                        .content_type(APPLICATION_JSON)
                        .await
                        .unwrap()
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

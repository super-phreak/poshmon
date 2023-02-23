
use std::time::SystemTime;
use actix_web::http::header::ContentType;
use crypto_common::{Key, KeyInit};
use crypto_common::rand_core::{OsRng};
use argon2::{self, Config};

use actix_web::web::{Json, Data, self};
use actix_web::{HttpResponse, post, get};
use r2d2::Pool;
use serde::{Deserialize, Serialize};

use crate::dbc::{DbcPool, get_user, create_user, insert_session};

use poshmon_lib::networking::{SessionToken, Salt, HmacSha256};

use crate::httpconst::CONNECTION_POOL_ERROR;

#[derive(Debug, Deserialize, Serialize)]
struct SessionCookie {
    pkey: String,
    session_id: String
}

impl SessionCookie {
    pub fn new(token: SessionToken) -> Self {
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
    .content_type(ContentType::json())
    .json(pong)
}

#[post("/login")]
pub async fn login(req: Json<Request>, pool: Data<DbcPool>, redis: Data<Pool<redis::Client>>) -> Result<HttpResponse, actix_web::Error> {
    //Pulling the password out so the borrow checker doesn't steal it when we hand the request over to the 
    //database to pull the user.
    let password: String = req.password.clone();

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let user_res = web::block(move || get_user(&req.username.clone(), &mut conn)).await;

    match user_res {
        Ok(user) => {
            match user {
                Ok(user) => {
                    if let Ok(res) = argon2::verify_encoded(&user.hash, password.as_bytes()) {
                        if res {
                            let mut redis = redis.get().expect(CONNECTION_POOL_ERROR);
                            let session_res = web::block(move || insert_session(user.username, &mut redis)).await;

                            match session_res {
                                Ok(session_info) => {
                                    match session_info {
                                        Ok(session_token) => {
                                            Ok(HttpResponse::Ok()
                                                .content_type(ContentType::json())
                                                .json(SessionCookie::new(session_token))
                                            )
                                        }
                                        Err(_) => HttpResponse::InternalServerError()
                                                    .content_type(ContentType::json())
                                                    .await, 
                                    }
                                }
                                Err(_) => HttpResponse::InternalServerError()
                                            .content_type(ContentType::json())
                                            .await, 
                            }
                            
                        } else {
                            HttpResponse::Unauthorized()
                                .content_type(ContentType::json())
                                .await
                        }
                    } else {
                        HttpResponse::InternalServerError()
                            .content_type(ContentType::json())
                            .await
                    }
                    
                },
                Err(_) => HttpResponse::NotFound()
                            .content_type(ContentType::json())
                            .await,
            }
        },
        Err(_) => HttpResponse::InternalServerError()
                    .content_type(ContentType::json())
                    .await, 
    }
}

#[post("/signup")]
pub async fn signup(req: Json<Request>, pool: Data<DbcPool>) -> Result<HttpResponse, actix_web::Error> {
    let mut connection = pool.get().expect(CONNECTION_POOL_ERROR);
    println!("Signup for user: {}", req.username);

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
                        .content_type(ContentType::json())
                        .await
                } else {
                    HttpResponse::Unauthorized()
                        .content_type(ContentType::json())
                        .await
                }
            },
            Err(_) => HttpResponse::Unauthorized()
                        .content_type(ContentType::json())
                        .await
        }
    } else {
        HttpResponse::InternalServerError()
                        .content_type(ContentType::json())
                        .await
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

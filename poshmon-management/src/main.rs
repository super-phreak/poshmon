use std::{env, io};

use actix_web::{middleware, App, HttpServer, web::Data};

use diesel::r2d2::ConnectionManager;
use diesel::pg::PgConnection;

use dotenv::dotenv;

mod auth;
mod httpconst;
mod dbc;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_manager = ConnectionManager::<PgConnection>::new(database_url);
    let db_pool = r2d2::Pool::builder()
        .build(db_manager)
        .expect("Failed to create pool");

    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let redis_client = redis::Client::open(redis_url).expect("Redis Connection is required");
    let redis_pool = r2d2::Pool::builder()
        .build(redis_client)
        .expect("Failed to create pool");

    let port = env::var("PORT").expect("Port must be set");

    HttpServer::new(move || {
        App::new()
            // Set up DB pool to be used with web::Data<Pool> extractor
            .app_data(Data::new(db_pool.clone()))
            .app_data(Data::new(redis_pool.clone()))
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(auth::login)
            .service(auth::signup)
            .service(auth::ping)
    })
    .bind(format!("0.0.0.0:{}",port))?
    .run()
    .await
}
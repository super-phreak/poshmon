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
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            // Set up DB pool to be used with web::Data<Pool> extractor
            .app_data(Data::new(pool.clone()))
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(auth::login)
            .service(auth::signup)
    })
    .bind("0.0.0.0:8443")?
    .run()
    .await
}
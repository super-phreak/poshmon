use diesel::{PgConnection, Connection};
use diesel::result::Error;
use diesel::r2d2::ConnectionManager;
use diesel::query_dsl::methods::{FilterDsl, LimitDsl, OrderDsl};
use diesel::{ExpressionMethods, Insertable, Queryable, RunQueryDsl};

use r2d2::{Pool, PooledConnection};

use dotenv::dotenv;
use uuid::Uuid;

use std::env;

use self::models::User;

pub mod models;
pub mod schema;

pub type DbcPool = Pool<ConnectionManager<PgConnection>>;
pub type DbcConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_user(_username: String, connection: &mut DbcConnection) -> Result<User, Error> {
    use self::schema::users::dsl::*;

    let res = users.filter(username.eq(_username)).load::<User>(connection);

    match res {
        Ok(user_list) => {
            match user_list.first() {
                Some(user) => Ok(user.clone()),
                None => Err(Error::NotFound),
            }
        },
        Err(_) => todo!(),
    }
}

pub fn create_user(username: &String, hash: String, connection: &mut DbcConnection) -> usize {
    use self::schema::users;

    let new_user = User { id: Uuid::new_v4(), username: username.clone(), hash };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(connection)
        .expect("Error saving new post")
}
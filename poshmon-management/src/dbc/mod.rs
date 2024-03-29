use chrono::Utc;

use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::query_dsl::methods::FilterDsl;
use diesel::{ExpressionMethods, RunQueryDsl};

use r2d2::{Pool, PooledConnection};

use uuid::Uuid;

use poshmon_lib::networking::SessionToken;

use self::models::User;

pub mod models;
pub mod schema;

pub type DbcPool = Pool<ConnectionManager<PgConnection>>;
pub type DbcConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn get_user(_username: &String, connection: &mut DbcConnection) -> Result<User, diesel::result::Error> {
    use self::schema::users::dsl::*;

    let res = users.filter(username.eq(_username)).load::<User>(connection);

    match res {
        Ok(user_list) => {
            match user_list.first() {
                Some(user) => Ok(user.clone()),
                None => Err(diesel::result::Error::NotFound),
            }
        },
        Err(_) => todo!(),
    }
}

pub fn insert_session(user: User, client: &mut PooledConnection<redis::Client>) -> Result<SessionToken, redis::RedisError> {
    let session = SessionToken::new(user.username.clone());


    let res: Result<(), redis::RedisError> = redis::pipe()
        .atomic()
        .sadd(
            "active_users",
            user.id.to_string()
        )
        .sadd(
            user.id.to_string(),
            session.session_id.to_string()
        )
        .hset_multiple(
            session.session_id.to_string(),
            &[
                ("pkey", base64::encode(session.session_key)),
                ("created_time", Utc::now().to_rfc3339())
                ]
        ).query(client);    

    match res {
        Ok(_) => Ok(session),
        Err(e) => Err(e),
    }
}

pub fn create_user(username: &String, hash: String, connection: &mut DbcConnection) -> usize {
    use self::schema::users;

    let new_user = User { id: Uuid::new_v4(), username: username.clone(), hash };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(connection)
        .unwrap_or(0)
}
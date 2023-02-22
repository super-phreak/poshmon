use diesel::{Queryable, Insertable};
use uuid::Uuid;
use crate::dbc::schema::users;

#[derive(Queryable, Insertable, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub hash: String,
}
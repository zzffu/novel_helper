use crate::schema::user;
use diesel::Queryable;
#[derive(Debug, Queryable)]
pub struct User {
    pub user_id: i32,
    pub user_name: String,
    pub user_password: String,
}

#[derive(Insertable)]
#[table_name = "user"]
pub struct NewUser<'a> {
    pub user_name: &'a str,
    pub user_password: &'a str,
}

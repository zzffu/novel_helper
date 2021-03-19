#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpServer};
use diesel::{
    r2d2::{self, ConnectionManager},
    MysqlConnection,
};
use dotenv::dotenv;
use std::env;
use std::io::Result;
mod entity;
mod handle;
mod middleware;
mod models;
mod schema;
mod token;

use handle::{book, user};

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap();

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(web::scope("/user").service(user::login).service(user::join))
            .service(web::scope("/search").service(book::search))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

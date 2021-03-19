use crate::{
    entity::{ApiResponse, RequestUser},
    models::User,
    token::encodetoken,
};
use actix_web::{post, web, HttpResponse, Responder};
use diesel::{r2d2::ConnectionManager, MysqlConnection, QueryResult, RunQueryDsl};

type DbPool = diesel::r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[post("/join")]
pub async fn join(user: web::Json<RequestUser>, pool: web::Data<DbPool>) -> impl Responder {
    let user = user.0;
    let coon = pool.get().unwrap();

    let result = web::block(move || insert_new_user(&user.name, &user.password, &coon))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        });

    if let Ok(1) = result {
        HttpResponse::Ok().json(ApiResponse::data(
            200,
            "success".to_string(),
            encodetoken().unwrap(),
        ))
    } else {
        HttpResponse::Ok().json(ApiResponse::new(500, "error".to_string()))
    }
}

pub fn insert_new_user(name: &str, password: &str, conn: &MysqlConnection) -> QueryResult<usize> {
    use crate::models::NewUser;
    use crate::schema::user;

    let new_user = NewUser {
        user_name: name,
        user_password: password,
    };

    diesel::insert_into(user::table)
        .values(&new_user)
        .execute(conn)
}

#[post("/login")]
pub async fn login(user: web::Json<RequestUser>, pool: web::Data<DbPool>) -> impl Responder {
    let user = user.0;
    let conn = pool.get().unwrap();

    let result = select_user(&user.name, &user.password, &conn);

    match result {
        Ok(v) if v.len() == 1 => HttpResponse::Ok().json(ApiResponse::data(
            200,
            "success".to_string(),
            encodetoken().unwrap(),
        )),
        Ok(_) => HttpResponse::Ok().json(ApiResponse::new(500, "error".to_string())),
        Err(_) => HttpResponse::Ok().json(ApiResponse::new(500, "error".to_string())),
    }
}

fn select_user(
    name: &str,
    password: &str,
    conn: &MysqlConnection,
) -> Result<Vec<User>, diesel::result::Error> {
    use crate::schema::user::dsl::*;
    use diesel::prelude::*;

    user.filter(user_name.eq(name))
        .filter(user_password.eq(password))
        .load::<User>(conn)
}

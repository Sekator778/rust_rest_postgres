#[macro_use]
extern crate diesel;

mod schema;
mod models;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use models::*;
use serde::Deserialize;
use std::env;
use uuid::Uuid;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

async fn create_user(
    pool: web::Data<DbPool>,
    user: web::Json<CreateUser>,
) -> impl Responder {
    use schema::users;

    let new_user = User {
        id: Uuid::new_v4(),
        name: user.name.clone(),
        email: user.email.clone(),
    };

    let conn = pool.get().expect("Couldn't get DB connection from pool");
    let mut conn = conn; // Додаємо змінну для змінного посилання

    let user = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(&mut conn)
        .expect("Error saving new user");

    HttpResponse::Ok().json(user)
}

async fn get_users(pool: web::Data<DbPool>) -> impl Responder {
    use schema::users::dsl::*;

    let conn = pool.get().expect("Couldn't get DB connection from pool");
    let mut conn = conn; // Додаємо змінну для змінного посилання

    let results = users
        .load::<User>(&mut conn)
        .expect("Error loading users");

    HttpResponse::Ok().json(results)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Оновлено до нового синтаксису
            .route("/users", web::post().to(create_user))
            .route("/users", web::get().to(get_users))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

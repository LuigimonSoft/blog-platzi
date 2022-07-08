#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use dotenv::dotenv;
use std::env;
use diesel::{prelude::*, connection};
use diesel::pg::PgConnection;

use diesel::r2d2::{self, ConnectionManager};
use diesel::r2d2::Pool;

use crate::models::PostSimplificado;

use actix_web::{ web, App, HttpServer, Responder, HttpResponse,  get, post};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

    use self::models::{Post, NewPost, NewPostHandler};
    use self::schema::posts;
    use self::schema::posts::dsl::*;

#[get("/")]
async fn index(pool: web::Data<DbPool>) -> impl Responder {
    let conn = pool.get().expect("Problemas al traer la base de datos");

    match web::block(move || {posts.load::<Post>(&conn)}).await {
        Ok(_data) => HttpResponse::Ok().body(format!("{:?}", _data)),
        Err(_err) => HttpResponse::Ok().body("Error al recibir la data")
    }
}

#[post("/new_post")]
async fn new_post(pool: web::Data<DbPool>, item: web::Json<NewPostHandler>) -> impl Responder {
    let conn = pool.get().expect("Problemas al traer la base de datos");

    match web::block(move || {Post::crate_post(&conn, &item)}).await {
        Ok(_data) => HttpResponse::Ok().body(format!("{:?}", _data)),
        Err(_err) => HttpResponse::Ok().body("Error al recibir la data")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
     dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("db url variable no encontrada");

    let connection = ConnectionManager::<PgConnection>::new(db_url);

    let pool = r2d2::Pool::builder().build(connection) .expect("No se pudo crear el pool");

    
    HttpServer::new(move || {
        App::new()
        .service(index)
        .service(new_post)
        .app_data(web::Data::new(pool.clone()))
    }).bind(("localhost",8080))?.run().await
}


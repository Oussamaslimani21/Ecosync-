use actix_web::{
    App, 
    HttpServer, 
    HttpResponse, 
    HttpRequest, 
    Responder, 
    http::header::CONTENT_LENGTH
};
use actix_web::{post, get, web};
use uuid::Uuid;
use serde_json::from_str;
use lazy_static::lazy_static;
use std::env;

mod modules;
pub use crate::modules::{types, queries};
use modules::types::Register;
use modules::queries::Database;

lazy_static! {
    static ref ACTIX_IP: String = env::var("ACTIX_IP_ADDRESS")
        .unwrap_or(String::from("0.0.0.0"));

    static ref ACTIX_PORT: u16 = env::var("ACTIX_PORT")
        .unwrap_or(String::from("3000"))
        .parse()
        .unwrap();

    static ref DB_USERNAME: String = env::var("DB_USERNAME")
        .unwrap_or(String::from("root"));

    static ref DB_PASSWORD: String = env::var("DB_PASSWORD")
        .unwrap_or(String::from("root"));

    static ref DB_IP: String = env::var("DB_HOST")
        .unwrap_or(String::from("0.0.0.0"));

    static ref DB_PORT: i32 = env::var("DB_PORT")
        .unwrap_or(String::from("3306"))
        .parse()
        .unwrap();

    static ref DB_DATABASE: String = env::var("DB_DATABASE")
        .unwrap_or(String::from("mysqldb"));

    static ref DB: Database<'static> = Database {
        username: &DB_USERNAME,
        password: &DB_PASSWORD,
        ip: &DB_IP,
        port: *DB_PORT,
        database: &DB_DATABASE
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = fs::create_dir_all(format!("/assets")).await;
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .service(register)
                    .service(file_upload)
            )
            .service(home)
    })
    .bind((ACTIX_IP.to_string(), *ACTIX_PORT as u16))?
    .run()
    .await
}

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body(format!("Nothing to see here"))
}

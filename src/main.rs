use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[get("/version")]
async fn version() -> impl Responder {
    HttpResponse::Ok().body("version 1.0.0")
}
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("hello, world")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("server is running : http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            .service(version)
            .service(index)
            .service(echo)
            .route("/hello", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

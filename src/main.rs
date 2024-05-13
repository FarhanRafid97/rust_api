mod controllers;

use actix_web::{App, HttpServer};

mod db;
use routes::hello;
mod routes;

use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable not found");
    println!("{}", db_url);
    HttpServer::new(|| App::new().service(hello))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

pub mod app_data;
mod controllers;
use crate::app_data::AppData;
use actix_web::{App, HttpServer};
mod db;
use routes::{hello, home, static_file};
mod routes;

use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable not found");
    println!("{}", db_url);
    HttpServer::new(|| {
        let tera_data = match tera::Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(er) => {
                println!("{}", er);

                ::std::process::exit(1);
            }
        };
        App::new()
            .app_data(actix_web::web::Data::new(AppData {
                template: tera_data.clone(),
            }))
            .service(static_file)
            .service(hello)
            .service(home)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

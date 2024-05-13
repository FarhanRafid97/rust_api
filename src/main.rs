mod controllers;

use actix_web::{App, HttpServer};
use db::connection;
mod db;
use routes::hello;
mod routes;
use controllers::user;
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let nama_saya = user::get_user("name from main".to_string());
    println!("{}", nama_saya);
    // let pool = connection::connection().await.unwrap();
    let data_user = connection::get_user_data().await.unwrap();
    for user in data_user {
        println!(
            "ID: {}, Name: {}, Email: {}",
            user.id, user.name, user.email
        );
    }

    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

mod controllers;

use db::connection;
mod db;
use controllers::{add_test, user};
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let nama_saya = user::get_user("name from main".to_string());
    println!("{}", nama_saya);
    let pool = connection::connection().await.unwrap();
    let data_user = connection::get_user_data(pool).await.unwrap();

    for user in data_user {
        println!(
            "ID: {}, Name: {}, Email: {}",
            user.id, user.name, user.email
        );
    }
    println!("Hello, world!");
    let new_val = add_test("hello world");
    println!("{}", new_val);
    Ok(())
}

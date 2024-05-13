use crate::db::connection::Database;

use actix_web::{get, HttpResponse, Responder};
use serde::Serialize;

use uuid::Uuid;

#[derive(sqlx::FromRow, Debug)]
pub struct SqlxUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct SqlxUserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
}
fn filter_db_record(note: &SqlxUser) -> SqlxUserResponse {
    SqlxUserResponse {
        email: note.email.to_owned(),
        id: note.id.to_string().to_owned(),
        name: note.name.to_owned(),
    }
}

#[get("/")]
pub async fn hello() -> impl Responder {
    let pool = match Database::new().await {
        Ok(pool) => pool,
        Err(_) => return HttpResponse::InternalServerError().body("ERROR connect"),
    };

    let actix_users_result = sqlx::query_as::<_, SqlxUser>("SELECT id, name, email FROM users")
        .fetch_all(&pool.database)
        .await;

    match actix_users_result {
        Ok(actix_users) => {
            // Serialize user data to JSON
            let response = actix_users
                .into_iter()
                .map(|res| filter_db_record(&res))
                .collect::<Vec<SqlxUserResponse>>();

            HttpResponse::Ok()
                .content_type("application/json")
                .json(serde_json::json!({"status":200,"message":"success retrive data", "result":response}))
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Database Error: {}", err)),
    }
}

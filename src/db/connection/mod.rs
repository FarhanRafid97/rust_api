use actix_web::Error;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug)]
pub struct ActixUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

pub struct Database {
    pub database: Pool<Postgres>,
}

impl Database {
    pub async fn new() -> Result<Self, Error> {
        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable not found");

        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Failed to create database connection pool");

        Ok(Self { database: pool })
    }
}

pub async fn get_user_data() -> Result<Vec<ActixUser>, actix_web::Error> {
    let pool = match Database::new().await {
        Ok(pool) => pool,
        Err(err) => return Err(Error::from(err)), // Propagate the error if connection fails
    };

    let actix_users = sqlx::query_as::<_, ActixUser>("SELECT id, name, email FROM users")
        .fetch_all(&pool.database)
        .await;
    println!("acticc user {:?}", actix_users);
    Ok(actix_users.unwrap())
}

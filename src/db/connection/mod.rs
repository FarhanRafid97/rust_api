use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug)]
pub struct ActixUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

pub async fn get_user_data(pool: Pool<Postgres>) -> Result<Vec<ActixUser>, actix_web::Error> {
    let actix_users = sqlx::query_as::<_, ActixUser>("SELECT id, name, email FROM users")
        .fetch_all(&pool)
        .await;
    println!("acticc user {:?}", actix_users);
    Ok(actix_users.unwrap())
}

pub async fn connection() -> Result<Pool<Postgres>, actix_web::Error> {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await;

    return Ok(pool.unwrap());
}

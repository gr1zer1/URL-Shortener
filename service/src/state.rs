use sqlx::postgres::{PgPool,PgPoolOptions};
use redis::Client;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: Client,
}



impl AppState {

    pub async fn new() -> Self {
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
        
        AppState {
            db:PgPoolOptions::new()
                .max_connections(10)
                .connect(db_url.as_str())
                .await
                .expect("Failed to connect to the database"),
            redis:Client::open(
                redis_url
            ).expect("Redis Error"),
        }
    }

}


use sqlx::postgres::{PgPool,PgPoolOptions};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub jwt_config: JWTConfig,
}



impl AppState {

    pub async fn new() -> Self {
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        
        AppState {
            db:PgPoolOptions::new()
                .max_connections(10)
                .connect(db_url.as_str())
                .await
                .expect("Failed to connect to the database"),
            jwt_config: JWTConfig {
                secret: std::env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
                expiration_access: std::env::var("JWT_EXPIRATION_ACCESS")
                    .expect("JWT_EXPIRATION_ACCESS must be set")
                    .parse()
                    .expect("JWT_EXPIRATION_ACCESS must be a number"),
                expiration_refresh: std::env::var("JWT_EXPIRATION_REFRESH")
                    .expect("JWT_EXPIRATION_REFRESH must be set")
                    .parse()
                    .expect("JWT_EXPIRATION_REFRESH must be a number"),
            },
        }
    }

}

#[derive(Clone)]
pub struct JWTConfig {
    pub secret: String,
    pub expiration_access: usize,
    pub expiration_refresh: usize,
}
mod routes;
mod state;
mod models;
mod db;
mod cache;
mod errors;
mod schemas;

use crate::routes::app;
use crate::state::AppState;


#[tokio::main]
async fn main() {

    dotenvy::dotenv().ok();
    let app_state = AppState::new().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind to address");

    let app = app().with_state(app_state);

    axum::serve(listener, app)
        .await
        .expect("Failed to serve application");
}

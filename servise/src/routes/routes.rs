use axum::Json;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::{Router, extract::State, routing::get};
use crate::db::*;
use crate::errors::AppError;
use crate::models::LinkModel;

use crate::state::AppState;

pub async fn root() -> &'static str {
    "Hello, World!"
}

async fn new_link(
    State(state):State<AppState>,
    Query(url):Query<String>
    ) -> Result<(StatusCode,Json<LinkModel>),AppError>{

    let pool = &state.db;

    let link = create_link(pool, url).await?;
    Ok((StatusCode::OK,Json(link)))
            
}

async fn get_all_links(
    State(state):State<AppState>
    ) -> Result<(StatusCode,Json<Vec<LinkModel>>),AppError>{

    let pool = &state.db;
    
    let links = get_links(pool).await?;
    Ok((StatusCode::OK,Json(links)))

}


pub fn app() -> Router<AppState> {
    Router::new()
        .route("/", get(root))
        .route("/links", get(get_all_links).post(new_link))
        
}
use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::{Router, extract::State, routing::get};
use crate::db::*;
use crate::errors::AppError;
use crate::models::LinkModel;
use crate::schemas::*;
use crate::cache::*;
use crate::state::AppState;

pub async fn root() -> &'static str {
    "Hello, World!"
}

async fn new_link(
    State(state):State<AppState>,
    Json(body):Json<ShortenRequest>
    ) -> Result<(StatusCode,Json<LinkModel>),AppError>{

    let pool = &state.db;

    

    let link = create_link(pool, body.url).await?;
    Ok((StatusCode::OK,Json(link)))
            
}

async fn get_all_links(
    State(state):State<AppState>
    ) -> Result<(StatusCode,Json<Vec<LinkModel>>),AppError>{

    let pool = &state.db;
    
    let links = get_links(pool).await?;
    Ok((StatusCode::OK,Json(links)))

}

async fn redirect(
    Path(code): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let url = match get_cached_url(&state.redis, code.clone()).await? {
        Some(url) => url,
        None => {
            let link = get_link_by_code(&state.db, code.clone()).await?;
            set_cached_url(&state.redis, code,link.url.clone()).await?;
            link.url
        }
    };

    Ok(Redirect::permanent(&url))
}


pub fn app() -> Router<AppState> {
    Router::new()
        .route("/", get(root))
        .route("/links", get(get_all_links).post(new_link))
        .route("/{code}", get(redirect))
        
}


mod cache;
mod handlers;

use axum::{
    extract::Query,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Extension, Json, Router,
};
use cache::OpengraphCache;
use handlers::fetch_opengraph_tags;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};

#[tokio::main]
async fn main() {
    let cache = Arc::new(Mutex::new(cache::OpengraphCache::new(100_000)));
    let app = Router::new()
        .route("/", get(index))
        .route("/api", get(get_opengraph_tags))
        .route("/status", get(get_status))
        .layer(Extension(cache));
    let addr = SocketAddr::from(([127, 0, 0, 1], 2340));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize, Clone)]
pub struct OpengraphTag {
    property: String,
    content: String,
}

async fn index() -> Html<&'static str> {
    Html(include_str!("../templates/index.html"))
}

#[derive(Deserialize)]
struct GetOpengraphTagsParameters {
    url: String,
}
async fn get_opengraph_tags(
    Query(query): Query<GetOpengraphTagsParameters>,
    cache: axum::extract::Extension<Arc<Mutex<OpengraphCache>>>,
) -> impl IntoResponse {
    let url = query.url;
    let cache_hit = cache.lock().await.get_from_cache(&url);
    if let Some(tags) = cache_hit {
        println!("Cache hit for {}", &url);
        return (StatusCode::OK, Json(tags));
    }
    let tags = match fetch_opengraph_tags(url.clone()).await {
        Ok(tags) => tags,
        Err(e) => {
            eprintln!("Error fetching opengraph tags for {url}: {e:?}");
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new()));
        }
    };

    // FIXME: This should be fire and forget
    cache.lock().await.add_to_cache(url, tags.clone());
    (StatusCode::OK, Json(tags))
}

async fn get_status(
    cache: axum::extract::Extension<Arc<Mutex<OpengraphCache>>>,
) -> impl IntoResponse {
    let stats = cache.lock().await.get_status();
    (StatusCode::OK, stats)
}

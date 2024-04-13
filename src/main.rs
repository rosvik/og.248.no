mod cache;

use axum::{
    extract::Query, http::StatusCode, response::IntoResponse, routing::get, Error, Extension, Json,
    Router,
};
use cache::OpengraphCache;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tl::{NodeHandle, Parser};
use tokio::{net::TcpListener, sync::Mutex};

#[tokio::main]
async fn main() {
    let cache = Arc::new(Mutex::new(cache::OpengraphCache::new(100_000)));
    let app = Router::new()
        .route("/", get(get_opengraph_tags))
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

async fn fetch_opengraph_tags(url: String) -> Result<Vec<OpengraphTag>, Error> {
    println!("Fetching tags for {}", &url);

    let result = reqwest::get(&url).await.expect("failed to fetch url");
    let data = result.text().await.expect("failed to get response body");

    let dom = tl::parse(&data, tl::ParserOptions::default()).expect("failed to parse html");
    let parser = dom.parser();
    let elements = dom.query_selector("meta").expect("no meta tags found");

    let mut data: Vec<OpengraphTag> = Vec::new();
    elements.into_iter().for_each(|element| {
        if let Some(ogd_data) = extract_opengraph_tag(element, parser) {
            data.push(ogd_data);
        }
    });
    Ok(data)
}

fn extract_opengraph_tag(node: NodeHandle, parser: &Parser) -> Option<OpengraphTag> {
    let node = node.get(parser).expect("element not found");
    let dom_tag = node.as_tag().expect("element is not a tag");
    if let Some(Some(property)) = dom_tag.attributes().get("property") {
        let property = property.as_utf8_str().to_string();
        if !property.starts_with("og:") {
            return None;
        }
        let content = match dom_tag.attributes().get("content") {
            Some(Some(content)) => content.as_utf8_str().into_owned(),
            _ => String::new(),
        };
        return Some(OpengraphTag { property, content });
    }
    None
}

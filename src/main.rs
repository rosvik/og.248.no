use axum::{
    extract::Query,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tl::{NodeHandle, Parser};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(get_ogd_data));
    let addr = SocketAddr::from(([127, 0, 0, 1], 2339));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct GetOgdDataParameters {
    url: String,
}
#[derive(Serialize)]
struct OgdData {
    property: String,
    content: String,
}

async fn get_ogd_data(Query(query): Query<GetOgdDataParameters>) -> impl IntoResponse {
    let url = query.url;
    let result = reqwest::get(url).await.expect("failed to fetch url");
    let data = result.text().await.expect("no data from url");

    let dom = tl::parse(&data, tl::ParserOptions::default()).expect("failed to parse html");
    let parser = dom.parser();
    let elements = dom.query_selector("meta").expect("no meta tags not found");

    let mut data: Vec<OgdData> = Vec::new();
    elements.into_iter().for_each(|element| {
        if let Some(ogd_data) = extract_ogd_data(element, parser) {
            data.push(ogd_data);
        }
    });

    (StatusCode::OK, HeaderMap::new(), Json(data))
}

fn extract_ogd_data(element: NodeHandle, parser: &Parser) -> Option<OgdData> {
    let element = element.get(parser).expect("element not found");
    let tag = element.as_tag().expect("element is not a tag");
    if let Some(Some(property)) = tag.attributes().get("property") {
        let property = property.as_utf8_str().to_string();
        if !property.starts_with("og:") {
            return None;
        }
        let content = tag
            .attributes()
            .get("content")
            .expect("content attribute is unset")
            .expect("content is empty")
            .as_utf8_str()
            .into_owned();
        return Some(OgdData { property, content });
    }
    None
}

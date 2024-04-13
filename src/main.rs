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
    let app = Router::new().route("/", get(get_opengraph_tags));
    let addr = SocketAddr::from(([127, 0, 0, 1], 2339));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct GetOpengraphTagsParameters {
    url: String,
}
#[derive(Serialize)]
struct OpengraphTag {
    property: String,
    content: String,
}

async fn get_opengraph_tags(Query(query): Query<GetOpengraphTagsParameters>) -> impl IntoResponse {
    let url = query.url;
    let result = reqwest::get(url).await.expect("failed to fetch url");
    let data = result.text().await.expect("no data from url");

    let dom = tl::parse(&data, tl::ParserOptions::default()).expect("failed to parse html");
    let parser = dom.parser();
    let elements = dom.query_selector("meta").expect("no meta tags found");

    let mut data: Vec<OpengraphTag> = Vec::new();
    elements.into_iter().for_each(|element| {
        if let Some(ogd_data) = extract_opengraph_tag(element, parser) {
            data.push(ogd_data);
        }
    });

    (StatusCode::OK, HeaderMap::new(), Json(data))
}

fn extract_opengraph_tag(node: NodeHandle, parser: &Parser) -> Option<OpengraphTag> {
    let node = node.get(parser).expect("element not found");
    let dom_tag = node.as_tag().expect("element is not a tag");
    if let Some(Some(property)) = dom_tag.attributes().get("property") {
        let property = property.as_utf8_str().to_string();
        if !property.starts_with("og:") {
            return None;
        }
        let content = dom_tag
            .attributes()
            .get("content")
            .expect("content attribute is unset")
            .expect("content is empty")
            .as_utf8_str()
            .into_owned();
        return Some(OpengraphTag { property, content });
    }
    None
}

use crate::OpengraphTag;
use axum::Error;
use tl::{NodeHandle, Parser};

pub async fn fetch_opengraph_tags(url: String) -> Result<Vec<OpengraphTag>, Error> {
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

pub fn extract_opengraph_tag(node: NodeHandle, parser: &Parser) -> Option<OpengraphTag> {
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

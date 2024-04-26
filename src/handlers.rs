use crate::OpengraphTag;
use anyhow::Result;
use tl::{NodeHandle, Parser};

pub async fn fetch_opengraph_tags(url: String) -> Result<Vec<OpengraphTag>> {
    println!("Fetching tags for {}", &url);

    let result = reqwest::get(&url).await?;
    let data = result.text().await?;

    let dom = tl::parse(&data, tl::ParserOptions::default())?;
    let parser = dom.parser();
    let mut data: Vec<OpengraphTag> = Vec::new();
    if let Some(elements) = dom.query_selector("meta") {
        elements.into_iter().for_each(|element| {
            if let Some(ogd_data) = extract_opengraph_tag(element, parser) {
                data.push(ogd_data);
            }
        });
    }
    Ok(data)
}

pub fn extract_opengraph_tag(node: NodeHandle, parser: &Parser) -> Option<OpengraphTag> {
    let node = node.get(parser)?;
    let dom_tag = node.as_tag()?;
    if let Some(Some(property)) = dom_tag.attributes().get("property") {
        let property = property.as_utf8_str().to_string();
        if !property.starts_with("og:") && !property.starts_with("twitter:") {
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

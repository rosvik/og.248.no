use crate::OpengraphTag;
use anyhow::Result;
use tl::{NodeHandle, Parser};

pub async fn fetch_opengraph_tags(url: String) -> Result<Vec<OpengraphTag>> {
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

        // E.g. `<meta property="og:title" content="Hello world" />`
        // E.g. `<meta property="article:published_time" content="2025-08-30T22:19:15+02:00">`
        if property.starts_with("og:") || property.starts_with("article:") {
            let content = match dom_tag.attributes().get("content") {
                Some(Some(content)) => content.as_utf8_str().into_owned(),
                _ => String::new(),
            };
            return Some(OpengraphTag { property, content });
        }
    }

    if let Some(Some(name)) = dom_tag.attributes().get("name") {
        let name = name.as_utf8_str().to_string();

        // E.g. `<meta name="twitter:title" content="Hello world" />`
        // E.g. `<meta name="article:modified_time" content="2025-08-30T16:41:58.259Z">`
        // E.g. `<meta name="cXenseParse:publishtime" content="2025-08-30T11:04:56.930Z">`
        if name.starts_with("twitter:")
            || name.starts_with("article:")
            || name.starts_with("cXenseParse:")
        {
            let content = match dom_tag.attributes().get("content") {
                Some(Some(content)) => content.as_utf8_str().into_owned(),
                _ => String::new(),
            };
            return Some(OpengraphTag {
                property: name,
                content,
            });
        }
    }
    None
}

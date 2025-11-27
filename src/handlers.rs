use crate::{OpengraphTag, second_level_domain::SecondLevelDomain};
use anyhow::Result;
use tl::{NodeHandle, Parser};
use url::Url;

pub async fn fetch_opengraph_tags(url: String) -> Result<Vec<OpengraphTag>> {
    let client = reqwest::Client::builder()
        .user_agent(get_user_agent(&url))
        .build()
        .unwrap();
    let result = client.get(&url).send().await?;
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

// https://www.useragents.me/
const DEFAULT_USER_AGENT: &str =
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 14.7; rv:136.0) Gecko/20100101 Firefox/136.0";

fn get_user_agent(url: &str) -> &'static str {
    println!("DEFAULT: {}", reqwest::header::USER_AGENT.as_str());
    let url = match Url::parse(url) {
        Ok(url) => url,
        Err(_) => return DEFAULT_USER_AGENT,
    };
    let domain = match url.second_level_domain() {
        Some(domain_name) => domain_name,
        None => return DEFAULT_USER_AGENT,
    };

    match domain.as_str() {
        // https://stackoverflow.com/a/46616889
        "youtube.com" => "facebookexternalhit/1.1",
        "youtu.be" => "facebookexternalhit/1.1",
        _ => DEFAULT_USER_AGENT,
    }
}

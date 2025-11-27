use url::Url;

pub trait SecondLevelDomain {
    /// Returns the domain of the URL without the subdomain
    fn second_level_domain(&self) -> Option<String>;
}

impl SecondLevelDomain for Url {
    /// Returns the domain of the URL without the subdomain
    fn second_level_domain(&self) -> Option<String> {
        let domain_parts = self.domain()?.split('.').collect::<Vec<&str>>();
        let top_level_domain = domain_parts[domain_parts.len() - 2..].join(".");
        Some(top_level_domain)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_second_level_domain() {
        let url = Url::parse("https://www.youtube.com/watch?v=dQw4w9WgXcQ").unwrap();
        assert_eq!(url.second_level_domain(), Some("youtube.com".to_string()));
    }
}

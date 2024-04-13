use quick_cache::unsync::Cache;

pub struct OpengraphCache {
    cache: Cache<String, Vec<crate::OpengraphTag>>,
}

impl OpengraphCache {
    pub fn new(size: usize) -> Self {
        Self {
            cache: Cache::new(size),
        }
    }

    pub fn add_to_cache(&mut self, url: String, tags: Vec<crate::OpengraphTag>) {
        self.cache.insert(url, tags);
    }

    pub fn get_from_cache(&self, url: &str) -> Option<Vec<crate::OpengraphTag>> {
        self.cache.get(url).cloned()
    }

    pub fn get_status(&self) -> String {
        let len = self.cache.len();
        let weight = self.cache.weight();
        let capacity = self.cache.capacity();
        format!(
            "Number of cached items: {}\nTotal cache weight: {}\nCapacity (maximum weight of cached items): {}\n",
            len, weight, capacity
        )
    }
}

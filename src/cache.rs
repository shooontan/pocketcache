use std::collections::HashMap;
use std::time::SystemTime;

use crate::time::Expiration;

#[derive(Debug)]
struct CacheItem<T> {
    cached_at: SystemTime,
    data: T,
}

/// pocketcache's cache struct
///
/// # Examples
///
/// ```
/// use pocketcache::cache::Cache;
/// use pocketcache::time::Expiration;
///
/// let expiration = Expiration::Hour(10);
/// let mut cache = Cache::<&str>::new(expiration);
///
/// cache.set("fruit", "banana");
/// assert_eq!(cache.get("fruit"), Some(&"banana"));
/// assert!(cache.get("meat").is_none());
///
/// cache.set("meat", "crab");
/// cache.delete("meat");
/// assert!(cache.get("meat").is_none());
/// ```
#[derive(Debug)]
pub struct Cache<T> {
    expiration: Expiration,
    items: HashMap<String, CacheItem<T>>,
}

impl<T> Cache<T> {
    fn now(&self) -> SystemTime {
        SystemTime::now()
    }

    pub fn new(expiration: Expiration) -> Cache<T> {
        Cache {
            expiration,
            items: HashMap::new(),
        }
    }

    ///
    /// # set
    ///
    /// ```
    /// use pocketcache::time::Expiration;
    /// use pocketcache::cache::Cache;
    ///
    /// let mut cache = Cache::<String>::new(Expiration::Default);
    ///
    /// assert!(cache.get("fruit").is_none());
    ///
    /// cache.set("fruit", String::from("banana"));
    ///
    /// assert_eq!(cache.get("fruit"), Some(&String::from("banana")));
    /// ```
    pub fn set(&mut self, key: &str, value: T) {
        self.items.insert(
            key.to_string(),
            CacheItem {
                cached_at: self.now(),
                data: value,
            },
        );
    }

    ///
    /// # get
    ///
    /// ```
    /// use pocketcache::time::Expiration;
    /// use pocketcache::cache::Cache;
    ///
    /// let mut cache = Cache::<String>::new(Expiration::Default);
    ///
    /// assert!(cache.get("fruit").is_none());
    ///
    /// cache.set("fruit", String::from("banana"));
    ///
    /// assert_eq!(cache.get("fruit"), Some(&String::from("banana")));
    /// ```
    pub fn get(&mut self, key: &str) -> Option<&T> {
        {
            let item = self.items.get(key);
            if let Some(item) = item {
                let duration = self.now().duration_since(item.cached_at);
                match duration {
                    Ok(duration) => {
                        if duration.as_secs() >= self.expiration.to_sec() {
                            self.delete(key);
                        }
                    }
                    _ => {
                        self.delete(key);
                    }
                }
            }
        }
        match self.items.get(key) {
            Some(item) => Some(&item.data),
            None => None,
        }
    }

    ///
    /// # delete
    ///
    /// ```
    /// use pocketcache::time::Expiration;
    /// use pocketcache::cache::Cache;
    ///
    /// let mut cache = Cache::<String>::new(Expiration::Default);
    ///
    /// cache.set("fruit", String::from("banana"));
    /// cache.delete("fruit");
    ///
    /// assert!(cache.get("fruit").is_none());
    /// ```
    pub fn delete(&mut self, key: &str) {
        self.items.remove(key);
    }

    ///
    /// # clear
    ///
    /// ```
    /// use pocketcache::time::Expiration;
    /// use pocketcache::cache::Cache;
    ///
    /// let mut cache = Cache::<String>::new(Expiration::Default);
    ///
    /// cache.set("fruit", String::from("orange"));
    /// cache.set("meat", String::from("crab"));
    /// cache.clear();
    ///
    /// assert!(cache.get("fruit").is_none());
    /// assert!(cache.get("meat").is_none());
    /// ```
    pub fn clear(&mut self) {
        self.items = HashMap::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getset() {
        let mut cache = Cache::<String>::new(Expiration::Default);
        cache.set("fruit", String::from("apple"));
        assert_eq!(cache.get("fruit"), Some(&String::from("apple")));
        cache.set("fruit", String::from("banana"));
        assert_eq!(cache.get("fruit"), Some(&String::from("banana")));
        assert!(cache.get("meat").is_none());
    }

    #[test]
    fn delete() {
        let mut cache = Cache::<String>::new(Expiration::Default);
        cache.set("fruit", String::from("orange"));
        assert_eq!(cache.get("fruit"), Some(&String::from("orange")));
        cache.delete("fruit");
        assert!(cache.get("fruit").is_none());
    }

    #[test]
    fn clear() {
        let mut cache = Cache::<String>::new(Expiration::Default);
        cache.set("fruit", String::from("banana"));
        cache.set("vegetable", String::from("carrot"));
        cache.set("meat", String::from("crab"));
        cache.clear();
        assert!(cache.get("fruit").is_none());
        assert!(cache.get("vegetable").is_none());
        assert!(cache.get("meat").is_none());
    }

    #[test]
    fn expired() {
        // TODO: time mock
        let mut cache = Cache::<String>::new(Expiration::Second(0));
        cache.set("fruit", String::from("orange"));
        assert!(cache.get("fruit").is_none());
    }
}

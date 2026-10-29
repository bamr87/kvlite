use std::collections::BTreeMap;

/// In-memory ordered store: the future memtable.
#[derive(Default)]
pub struct MemStore {
    map: BTreeMap<String, String>,
}

impl MemStore {
    pub fn put(&mut self, k: String, v: String) {
        self.map.insert(k, v);
    }
    pub fn get(&self, k: &str) -> Option<String> {
        self.map.get(k).cloned()
    }
    pub fn delete(&mut self, k: &str) -> bool {
        self.map.remove(k).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn put_get_delete() {
        let mut s = MemStore::default();
        s.put("a".into(), "1".into());
        assert_eq!(s.get("a").as_deref(), Some("1"));
        assert!(s.delete("a"));
        assert_eq!(s.get("a"), None);
    }
}

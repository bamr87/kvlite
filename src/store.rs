use std::collections::BTreeMap;

/// In-memory ordered store: the future memtable.
#[derive(Default)]
pub struct MemStore {
    map: BTreeMap<String, String>,
}

impl MemStore {
    /// Inserts `v` under `k`, replacing any existing value (last write wins).
    pub fn put(&mut self, k: String, v: String) {
        self.map.insert(k, v);
    }
    /// Returns a copy of the value stored under `k`, or `None` if the key is absent.
    pub fn get(&self, k: &str) -> Option<String> {
        self.map.get(k).cloned()
    }
    /// Removes `k`; returns `true` if the key was present, `false` if it was not.
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

    #[test]
    fn put_overwrites_existing_value() {
        let mut s = MemStore::default();
        s.put("a".into(), "1".into());
        s.put("a".into(), "2".into());
        assert_eq!(s.get("a").as_deref(), Some("2"));
    }

    #[test]
    fn delete_missing_key_returns_false() {
        let mut s = MemStore::default();
        assert!(!s.delete("absent"));
        s.put("a".into(), "1".into());
        assert!(s.delete("a"));
        assert!(!s.delete("a"));
    }
}

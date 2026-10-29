mod store;

fn main() {
    let mut db = store::MemStore::default();
    db.put("hello".into(), "world".into());
    println!("kvlite 0.1.0: hello={}", db.get("hello").unwrap_or_default());
}

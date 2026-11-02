mod store;

fn main() {
    let mut db = store::MemStore::default();
    db.put("hello".into(), "world".into());
    // Version comes from Cargo.toml so release-please bumps never drift from this banner.
    println!(
        "kvlite {}: hello={}",
        env!("CARGO_PKG_VERSION"),
        db.get("hello").unwrap_or_default()
    );
}

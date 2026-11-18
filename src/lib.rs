//! kvlite: a small LSM-tree key-value store, built one stage at a time.
//!
//! Today the library holds [`store::MemStore`], the in-memory store that will
//! become the memtable. The `kvlite` binary (`src/main.rs`) is an interactive
//! put/get/del shell over it. Nothing is persisted yet: the write-ahead log is
//! the next planned feature (KVL-002).

pub mod store;

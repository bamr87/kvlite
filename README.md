# kvlite

> A small LSM-tree key-value store in Rust, growing into a Raft-replicated cluster.

Learn storage engines and consensus by building them: a write-ahead log, memtable, SSTables and compaction, then Raft replication across nodes.

**Status:** foundation (phase 1). Part of the [bamr87 fleet](https://github.com/bamr87/bamr87).

## Goals

- Durable single-node KV store (WAL + memtable + SSTables)
- Leveled compaction and bloom filters
- Raft-replicated multi-node mode with a gRPC API

## Skills learned

- Rust ownership, traits and testing
- LSM-tree internals
- Raft consensus
- Benchmarking and fault injection

## Fleet tie-in

A candidate embedded store for fleet tools (e.g. fleet activity log buffering) and a systems-depth showcase in the portfolio.

## Quick start

```bash
cargo build && cargo test
```

## How it grows

The roadmap lives in [PRD.md](PRD.md) and [features/features.yml](features/features.yml) (ids `KVL-NNN`, most `status: planned`). Work arrives as pull requests from the hub's evolution and fan-out loops, the `@claude` handler, or the local `evolve` lane (off until `EVOLVE_ENABLED=true`). A human merges every PR. Rules: [docs/constitution.md](docs/constitution.md) · agent guide: [AGENTS.md](AGENTS.md).

## License

[MIT](LICENSE)

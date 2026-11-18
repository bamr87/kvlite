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
cargo fmt --check                            # formatting gate; `cargo fmt` fixes it
cargo clippy --all-targets -- -D warnings    # lint gate; any warning fails
printf 'put a 1\nget a\ndel a\nget a\n' | cargo run -q
# OK
# 1
# OK
# (nil)
```

Run `cargo run -q` with no input to type the commands yourself: a `kvlite>` prompt appears, and Ctrl-D exits. `cargo run -q -- --help` prints the usage.

What exists today is phase 1's first step only. The library target ([`src/lib.rs`](src/lib.rs)) exports `MemStore` from [`src/store.rs`](src/store.rs), an in-memory `BTreeMap<String, String>` with `put` (last write wins), `get` and `delete`. The `kvlite` binary ([`src/main.rs`](src/main.rs)) is a line-oriented shell over it, and [`tests/cli.rs`](tests/cli.rs) drives that binary end to end. It reads one command per line from stdin and prints one reply per command:

| Command | Reply |
| --- | --- |
| `put <key> <value>` | `OK`. The value is the rest of the line after the key, so it may contain spaces. |
| `get <key>` | The value, or `(nil)` if the key is absent. |
| `del <key>` | `OK` if the key was removed, `(nil)` if it was absent. |
| `help` | The usage. |

Blank lines are skipped. An unknown command or a wrong number of arguments prints `ERR <reason>` and the shell carries on; at the end of input it exits 0. The prompt goes to stderr, and only when stdin is a terminal, so piped output is exactly the replies. Nothing is persisted yet: every run starts empty, and the write-ahead log is the next planned feature (KVL-002).

## How it grows

The roadmap lives in [PRD.md](PRD.md) and [features/features.yml](features/features.yml) (ids `KVL-NNN`, most `status: planned`). Work arrives as pull requests from the hub's evolution and fan-out loops, the `@claude` handler, or the local `evolve` lane (off until `EVOLVE_ENABLED=true`). A human merges every PR. Rules: [docs/constitution.md](docs/constitution.md) · agent guide: [AGENTS.md](AGENTS.md) · every document, one line each: [docs/README.md](docs/README.md) (regenerate with `python3 tools/docs_index.py --write`).

## License

[MIT](LICENSE)

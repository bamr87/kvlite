<!-- kit: governance v0.1.0 -->
# 0002 — Missions

- **Status:** ratified
- **Date:** 2026-11-16
- **Proposed by:** Grok Bot (agent), at the maintainer's request
- **Ratified by:** bamr87, by merging [#12](https://github.com/bamr87/kvlite/pull/12) on 2026-11-18
- **Affects:** constitution §1; PRD.md §1.1; KVL-008
- **Supersedes:** none
- **Related:** [PRD.md](../../PRD.md) §1.1

## Context

Constitution §1 was a single mission sentence or short paragraph, not one direction statement with exactly three numbered missions. The fleet rule is that every repo's constitution §1 uses that form, adopted through a numbered amendment the maintainer ratifies by merging. Mission 1 is the maintainer's own wording, adapted to this repo: provide data, tools, resources and transparency for all stakeholders, practitioners in the domain, and the general public. The ambition is the direction, not a claim that the work is already finished.

The earlier §1 text was:

> Learn storage engines and consensus by building them: a write-ahead log, memtable, SSTables and compaction, then Raft replication across nodes. It is a learning-first portfolio project in the bamr87 fleet: A candidate embedded store for fleet tools (e.g. fleet activity log buffering) and a systems-depth showcase in the portfolio.

## Amendment

Retitle constitution §1 **Missions**. Replace the body of §1 with the following. Any invariants or other normative rules that already lived outside §1 are unchanged.

kvlite's direction is to become the ultimate open home for all things embedded key-value storage and consensus — data, tools, resources and transparency, gathered in one place that anyone can inspect and reuse.

1. **Provide data, tools, resources and transparency for all stakeholders, systems programmers, storage practitioners and the general public.** Learners, systems builders, fleet-tool authors, and the general public are first-class audiences. The aim is a shared, open commons: the same sources, the same tools, and the same trail of evidence for everyone.
2. **Make storage engines and consensus learnable by building them in the open: a write-ahead log, memtable, SSTables and compaction, then Raft replication across nodes.** Each layer stays inspectable; correctness is proven by tests, not by claims.
3. **Put that store in every learner's and builder's hands, and make it a candidate embedded store for fleet tools.** The path includes uses such as fleet activity-log buffering, and a systems-depth showcase in the portfolio — an open toolkit for working with all things embedded key-value storage and consensus.

These missions set the course; they do not claim the work is finished. They serve the existing constitution and PRD purpose: learn storage and consensus by building them, then offer the result as a candidate embedded store. The earlier mission paragraph is absorbed into these three. Every feature must trace to them.

_Source: [../PRD.md](../PRD.md) §1.1._

## Consequences

Agents and reviewers judge fit against these three missions before inventing scope, treating them as the direction of the repo rather than a description of what already ships. The same text is added as PRD.md §1.1 so the constitution's `_Source:` line stays accurate. The PRD purpose, goals, feature catalog and invariants outside this addition are unchanged except for a new catalog entry (KVL-008) and a regenerated feature-catalog block where the repo already has one. This PR also aligns the README and AGENTS.md one-liners with the missions and links them to constitution §1.

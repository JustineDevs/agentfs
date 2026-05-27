# AgentFS Repository Architecture

> [!IMPORTANT]
> This file defines the target repository structure and subsystem ownership for AgentFS. It should be treated as an architectural contract, not just a directory sketch.

> [!NOTE]
> The current repository is documentation-first. The file tree and subsystem descriptions below describe the intended buildout target, not a literal inventory of what exists in this checkout today.

This document defines the intended production-grade monorepo layout for AgentFS. It is written as the canonical architecture map for contributors so the repository can grow without drifting on naming, ownership, or subsystem boundaries.

The current checkout is documentation-first and does not yet contain the full scaffold described here. Treat this file tree as the target repository architecture that implementation should grow toward, not as a literal snapshot of the current working tree.

The design goals behind the layout are:

- keep the native runtime small and auditable
- isolate platform-critical code from docs and adapters
- make Git automation and security guardrails first-class
- support a polyglot contributor experience without turning the core into a Node-only or Rust-only project

| Design Goal | Why It Exists |
| --- | --- |
| Small trusted core | Keeps the runtime auditable and easier to secure |
| Layer separation | Prevents CLI or SDK wrappers from owning core correctness |
| Git-first automation | Makes repository state reconciliation part of the product |
| Polyglot ergonomics | Lets multiple environments consume one core runtime model |

## Current State vs Target State

Current repository state:

- founding documentation
- license
- Git remote wiring

Target repository state:

- native runtime crates
- CLI wrapper
- SDK packages
- CI, release, and security automation
- contributor toolchain and tests

| State | Contents |
| --- | --- |
| Current | Founding docs, legal files, git wiring, early repo conventions |
| Target | Runtime, Git integration, SDKs, CI, tests, release automation, contributor tooling |

## Root Monorepo Layout

> [!TIP]
> Read the tree below as a responsibility map. The important part is not only where files live, but which layer owns which category of behavior.

```text
agentfs/
├── .changeset/
│   ├── config.json
│   └── README.md
├── .devcontainer/
│   └── devcontainer.json
├── .github/
│   ├── workflows/
│   │   ├── ci.yml
│   │   ├── release.yml
│   │   └── security.yml
│   ├── DEPENDENCIES.md
│   └── FUNDING.yml
├── .vscode/
│   ├── extensions.json
│   └── settings.json
├── apps/
│   ├── cli/
│   │   ├── src/
│   │   │   └── index.ts
│   │   ├── package.json
│   │   └── tsconfig.json
│   └── docs/
│       ├── docs.json
│       └── introduction.mdx
├── crates/
│   ├── afs-core/
│   │   ├── src/
│   │   │   ├── db/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── schema.rs
│   │   │   │   └── wal.rs
│   │   │   ├── security/
│   │   │   │   ├── keys.rs
│   │   │   │   ├── signatures.rs
│   │   │   │   └── policy.rs
│   │   │   ├── vfs/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── projections.rs
│   │   │   │   └── mounts.rs
│   │   │   ├── vector/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── quantize.rs
│   │   │   │   └── cache.rs
│   │   │   ├── runtime/
│   │   │   │   ├── context.rs
│   │   │   │   ├── locks.rs
│   │   │   │   ├── events.rs
│   │   │   │   └── scopes.rs
│   │   │   ├── cli.rs
│   │   │   ├── config.rs
│   │   │   ├── lib.rs
│   │   │   └── main.rs
│   │   └── Cargo.toml
│   └── afs-git/
│       ├── src/
│       │   ├── clean_filter.rs
│       │   ├── diff_driver.rs
│       │   ├── hooks.rs
│       │   ├── merge_driver.rs
│       │   └── smudge_filter.rs
│       └── Cargo.toml
├── packages/
│   ├── sdk-python/
│   │   ├── agentfs/
│   │   │   ├── __init__.py
│   │   │   ├── client.py
│   │   │   └── context.py
│   │   └── pyproject.toml
│   └── sdk-ts/
│       ├── src/
│       │   ├── index.ts
│       │   ├── client.ts
│       │   └── context.ts
│       ├── package.json
│       └── tsconfig.json
├── docs/
│   ├── Architect-File-tree.md
│   ├── Concept.md
│   └── Roadmap.md
├── scripts/
│   ├── bootstrap-dev.sh
│   ├── install-git-hooks.sh
│   ├── release-artifacts.sh
│   └── verify-binary-size.sh
├── tests/
│   ├── fixtures/
│   ├── integration/
│   ├── security/
│   └── snapshots/
├── benchmarks/
│   ├── harness/
│   │   ├── runner.rs
│   │   ├── config.rs
│   │   └── report.rs
│   ├── micro/
│   │   ├── db_read.rs
│   │   ├── db_write.rs
│   │   ├── vector_query.rs
│   │   └── lock_contention.rs
│   ├── macro/
│   │   ├── agent_session.rs
│   │   └── git_lifecycle.rs
│   ├── fixtures/
│   │   ├── small_repo/
│   │   ├── large_repo/
│   │   └── concurrent_agents/
│   └── results/
│       ├── baselines/
│       └── .gitkeep
├── .editorconfig
├── .gitignore
├── biome.json
├── Cargo.toml
├── deny.toml
├── lefthook.yml
├── package.json
├── pnpm-workspace.yaml
├── renovate.json
└── turbo.json
```

## Why This Layout Works

| Layer | Primary Responsibility | Should Not Own |
| --- | --- | --- |
| `crates/afs-core` | Runtime correctness, storage, trust boundaries | UX wrapper logic or website concerns |
| `crates/afs-git` | Git lifecycle integrations | Core semantic retrieval policy |
| `apps/cli` | Operator ergonomics and user-facing flows | Reimplementation of runtime semantics |
| `packages/*` | Language adapters and consumers | Canonical business logic of the core |
| `docs/` | Product and architecture truth | Implementation-only decisions hidden from public docs |
| `tests/` | correctness, regression, and security verification | performance claims |
| `benchmarks/` | latency, throughput, contention, and workload evidence | product runtime logic |

### `crates/` owns the trusted core

The native implementation belongs in `crates/` so performance-critical behavior, filesystem integration, and trust-boundary logic are isolated from higher-level wrappers.

`crates/afs-core/` is the authority for:

- database schema
- memory mapping
- concurrency control
- signature verification
- runtime context resolution
- projection and reflection mechanics

`crates/afs-git/` is separate because Git lifecycle plumbing has different failure modes and upgrade cadence than the main storage engine. Keeping that code isolated reduces blast radius when merge logic or filter behavior changes.

### `apps/cli/` is the ergonomic shell entrypoint

The repository reserves `apps/cli/` for a TypeScript operator wrapper around the native runtime. That lets the project ship:

- a polished `npm i -g agentfs` experience
- shell-friendly prompts and error messages
- future plugin surfaces without bloating the core binary

The wrapper should stay thin. It should orchestrate, not reimplement, runtime behavior already owned by `afs-core`.

### `packages/` provides language adapters

Python and TypeScript SDKs matter because not every agent system will integrate through the shell. A language adapter layer makes AgentFS consumable from:

- IDE extensions
- autonomous coding agents
- backend agent frameworks
- internal developer tools

The SDK packages should remain small and treat the core runtime as the source of truth.

### `docs/` stays product-facing and durable

Long-lived design documents should not be buried inside package folders. Root-level `docs/` keeps the narrative artifacts stable and easy to reference from issues, PRs, and the project website.

## Deep Dive: Core Technical Areas

### `crates/afs-core/src/db/`

This subsystem defines the binary data contract.

Expected ownership:

- schema creation and migrations
- WAL and checkpoint policy
- low-level query APIs
- binary size budget enforcement
- TTL archival or pruning policies

Files:

- `schema.rs`
  Central table definitions, indexes, constraints, and compatibility rules.
- `wal.rs`
  Write-ahead logging tuning, checkpoint strategy, and corruption recovery logic.
- `mod.rs`
  Shared database entrypoint, connection pooling, and transactional helpers.

### `crates/afs-core/src/runtime/`

This layer turns stored records into live agent context.

Expected ownership:

- path-to-scope resolution
- context assembly
- LRU or sliding-window cache behavior
- row locking and mutex semantics for multi-agent execution
- event dispatch for changes that should invalidate caches

Files:

- `context.rs`
  Builds active instruction sets for the current working scope.
- `scopes.rs`
  Normalizes folder paths and inheritance from nested scopes to global scope.
- `locks.rs`
  Multi-agent coordination, lock ownership, and timeout rules.
- `events.rs`
  Internal pub/sub or append-queue lifecycle for runtime updates.

### `crates/afs-core/src/vector/`

This subsystem is optional in the very first shipping slice, but the layout should reserve it from the start to avoid later structural churn.

Expected ownership:

- embedding ingestion
- quantization strategy
- vector or semantic-hash retrieval
- memory budget enforcement

Files:

- `quantize.rs`
  Binary or scalar compression path for keeping the index small.
- `cache.rs`
  In-memory index windows capped by a fixed RAM budget.

### `crates/afs-core/src/security/`

Security is a product boundary, not a future enhancement.

> [!WARNING]
> Changes in this area are high-risk because they define whether the system preserves human control over protected policy and hook surfaces.

Expected ownership:

- signature verification
- role or domain separation
- protected write paths
- enterprise policy import validation

Files:

- `keys.rs`
  Local developer key discovery and trust store mapping.
- `signatures.rs`
  Signature generation and verification routines.
- `policy.rs`
  Protected-table access control and tamper detection.

### `crates/afs-core/src/vfs/`

This layer explains how a binary-first system remains human-debuggable.

Expected ownership:

- projection of binary state into readable text
- optional FUSE, projected filesystem, or generated-view support
- on-demand export of virtual files

Files:

- `projections.rs`
  Markdown or plaintext renderers for rules, hooks, and memory state.
- `mounts.rs`
  Platform-specific mount adapters or shims.

## Workspace Tooling

| File | Purpose |
| --- | --- |
| `turbo.json` | Task graph, caching, and affected-workspace execution |
| `lefthook.yml` | Fast local quality gates before commits land |
| `deny.toml` | Dependency, license, and bloat control for the trusted core |

### `turbo.json`

Turbo coordinates task execution across the monorepo so docs, SDKs, and the CLI wrapper do not force redundant native builds when the Rust core is untouched.

Representative responsibilities:

- cache build output
- limit task execution to affected workspaces
- separate docs and SDK build lanes from native compile lanes

### `lefthook.yml`

Lefthook owns local quality gates. The repository uses it for fast contributor feedback before code becomes a commit.

Expected checks:

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `pnpm biome check`
- `cargo deny check`

### `deny.toml`

This file exists to preserve the product promise that the core remains lightweight, secure, and legally clean.

Responsibilities:

- ban disallowed licenses
- block dependency duplication where it threatens binary size
- reject oversized or unnecessary feature selections

## CI and Release Topology

> [!NOTE]
> These workflows are described as target operational infrastructure. They become real only when the implementation layer reaches the point where build, test, and release automation are meaningful.

### `.github/workflows/ci.yml`

Runs on pull requests and mainline updates.

Recommended checks:

- Rust formatting and lint
- JavaScript and TypeScript linting
- unit and integration tests
- binary size budget enforcement
- documentation link validation

### `.github/workflows/release.yml`

Produces platform artifacts for:

- Linux x86_64
- macOS Apple Silicon
- Windows x86_64

The release pipeline should publish signed binaries and checksums so installation paths remain simple and trustworthy.

### `.github/workflows/security.yml`

Owns:

- dependency advisories
- secret scanning
- supply chain checks
- potentially license or signature audits

## Contributor Experience Layers

### `.devcontainer/`

Provides a predictable workspace for:

- Rust toolchain
- Node and pnpm
- SQLite tooling
- docs preview tooling

This matters because a repo with native code, wrappers, and documentation can otherwise become hard to onboard quickly.

### `.vscode/`

The VS Code defaults should reinforce repository rules instead of fighting them:

- format on save
- recommended extensions
- consistent indentation
- project-local task definitions

## Testing Strategy by Folder

| Folder | Focus |
| --- | --- |
| `tests/integration/` | init flows, rule retrieval, Git lifecycle plumbing |
| `tests/security/` | signature enforcement, forbidden writes, tamper detection |
| `tests/snapshots/` | diff export and virtual reflection rendering |
| `tests/fixtures/` | fake repos, broken merge cases, legacy config ingestion samples |

## Benchmark Strategy by Folder

> [!IMPORTANT]
> Benchmarking is a root-level concern because AgentFS performance claims cross subsystem boundaries. Micro-benchmarks exercise `crates/afs-core/`, while macro-benchmarks exercise the CLI, Git integration, fixtures, and storage together.

| Folder | Focus |
| --- | --- |
| `benchmarks/harness/` | orchestration, warmup, teardown, iteration control, and report aggregation |
| `benchmarks/micro/` | isolated latency and throughput for database, vector, lock, and write paths |
| `benchmarks/macro/` | end-to-end agent-session and Git-lifecycle workload simulations |
| `benchmarks/fixtures/` | synthetic repositories, trace inputs, and concurrent-agent scenarios |
| `benchmarks/results/` | generated benchmark output plus intentional committed baselines |

### Benchmark scope

| Benchmark | Measurement Target | Why It Exists |
| --- | --- | --- |
| `micro/db_read.rs` | scoped record lookup latency | validates the `<1ms` local-read ambition |
| `micro/db_write.rs` | WAL-backed write throughput and checkpoint cost | validates local mutation behavior |
| `micro/vector_query.rs` | vector lookup latency and memory pressure | validates future semantic retrieval design |
| `micro/lock_contention.rs` | multi-agent lock acquisition under load | validates concurrency strategy |
| `macro/agent_session.rs` | full context resolution flow | validates user-visible agent turn cost |
| `macro/git_lifecycle.rs` | clean/smudge/diff/merge simulation | validates Git lifecycle overhead |

### Result policy

Generated benchmark output is ignored by default. Stable baseline summaries may be committed under `benchmarks/results/baselines/` when they are intentionally used as release evidence.

## File Ownership Guidance

To avoid architectural drift, contributors should follow these ownership rules:

- do not implement Git merge logic inside the CLI wrapper
- do not implement signature checks only in the UI layer
- do not let SDKs diverge from core scope semantics
- do not let docs invent commands that the CLI does not intend to support

## Architecture Summary

This layout keeps AgentFS credible as a real systems project:

| Outcome | Structural Reason |
| --- | --- |
| small trusted center | runtime logic is isolated in native crates |
| good contributor ergonomics | wrappers, docs, and tooling are separated cleanly |
| better safety review | trust boundaries and Git lifecycle code have explicit homes |
| better long-term scalability | adapters and docs can grow without destabilizing the core |

- native code owns correctness and performance
- wrappers own usability
- docs own clarity and adoption
- CI and hook tooling own trust

That separation is what lets the repository stay small at the center while still supporting a world-class open-source contributor ecosystem.

# AgentFS Technical Roadmap

> [!IMPORTANT]
> This roadmap is the full technical sequencing and execution reference for AgentFS. It is intentionally deeper than a normal backlog because the project depends on coherent ordering across storage, Git integration, reflection, trust boundaries, compatibility, concurrency, and release infrastructure.

> [!NOTE]
> The repository is still documentation-first. This file describes intended process, phases, syntax, tools, research parallels, operating flows, and milestone logic. It should not be read as evidence that the listed implementation phases are already complete in this checkout.

---

## 1. Roadmap Purpose

This file answers the technical planning questions that follow the concept:

- what should be built first?
- what must exist before later phases make sense?
- what tools and technologies are involved?
- what terms and syntax matter operationally?
- what blockers are known in advance?
- what are the tradeoffs of the architecture?

### Roadmap dimensions

| Dimension | What It Controls |
| --- | --- |
| Product scope | what AgentFS is actually trying to become |
| Phase ordering | what must exist before downstream work is safe |
| Technical honesty | what is planned versus what is shipped |
| Tooling alignment | how build, test, release, and Git behavior fit together |
| Risk management | how blockers and tradeoffs are handled over time |

---

## 2. System Goal

The goal of AgentFS is to replace scattered AI repository configuration with one project-native semantic store and one coherent runtime model.

### High-level system goals

| Goal | Meaning |
| --- | --- |
| Local-first runtime | the repository remains the primary execution boundary |
| Single source of truth | one semantic store replaces many conflicting files |
| Scope-aware retrieval | only relevant context should be loaded |
| Protected policy model | hooks and rules remain under human authority |
| Git-native state management | merge, diff, stage, commit, and push behavior are designed in |
| Reflection for humans | a binary-backed system remains auditable and explorable |

---

## 3. Terminology and Syntax

The roadmap uses several terms that must stay stable.

### Core terms

| Term | Meaning |
| --- | --- |
| `.agent.db` | canonical hidden semantic store |
| `scope` | path or namespace where a record applies |
| `category` | record type such as rule, hook, skill, memory |
| `user domain` | protected human-owned state |
| `agent domain` | mutable agent-writable state |
| `reflection layer` | readable export or virtual view over binary-backed state |
| `merge driver` | custom Git reconcile logic for `.agent.db` |
| `clean filter` | Git stage-time transform |
| `smudge filter` | Git materialization-time transform |
| `TTL` | automatic expiry of temporary records |
| `BQ` / `SQ` | vector compression strategies |
| `LRU` | bounded cache/window strategy |

### Human command syntax

```bash
afs init
afs add rule "Always use strict TypeScript" --scope /apps/web
afs add hook pre-commit "pnpm test" --scope global
afs status
afs doctor
afs export --format markdown
afs ingest --clean
```

### Conceptual internal syntax

```text
ai://rules/syntax
ai://state/memory
ai://skills/git/commit
ai://hooks/pre-commit
```

### Operational variables and fields

| Field | Purpose |
| --- | --- |
| `id` | stable record identity |
| `category` | rule / skill / hook / memory / log / snapshot |
| `scope` | path-local semantic routing key |
| `content` | main payload |
| `signature` | authority proof for protected records |
| `embedding` | semantic retrieval payload |
| `ttl_seconds` | expiration policy |
| `authority_zone` | `user` or `agent` write domain |
| `updated_at` | reconciliation and ordering timestamp |

---

## 4. Research and Technical Basis

AgentFS adapts several established technical ideas rather than inventing every building block from scratch.

| Reference | What It Contributes |
| --- | --- |
| SQLite | embedded single-file persistence |
| DuckDB | analytical embedded patterns and future semantic extensions |
| LMDB | memory-mapped local database inspiration |
| Apache Arrow | zero-copy data access ideas |
| Git | repository-local truth and lifecycle semantics |
| FUSE / projected filesystems | virtual readable reflection layers |
| actor-model appenders | serialized concurrency-safe writing |
| vector compression techniques | bounded local semantic retrieval |

### Why this matters to the roadmap

The roadmap is not only a feature sequence. It is a decision to combine:

- embedded storage
- semantic retrieval
- Git operations
- local-first computing
- explicit trust boundaries

into one coherent repository system.

### Implementation references to evaluate

| Reference | What It Changes in the Roadmap |
| --- | --- |
| [`sqlite-vec`](https://github.com/asg017/sqlite-vec) / [`sqlite-vss`](https://github.com/asg017/sqlite-vss) | vector search should be modeled as an optional provider boundary in `afs-core`, not hardcoded as one in-house algorithm |
| [Git LFS](https://github.com/git-lfs/git-lfs/blob/main/docs/spec.md) | clean/smudge filters need pointer stability, idempotency, and clear degraded behavior |
| [git-annex](https://git-annex.branchable.com/git-annex/) | Git can manage indirect file state, but only if humans understand the indirection |
| [Git attributes](https://git-scm.com/docs/gitattributes) | merge, diff, clean, and smudge rules are part of the product lifecycle |
| [Sigstore / Cosign](https://docs.sigstore.dev/cosign/signing/overview/) | Phase 6 should consider OIDC-backed policy signing and transparency, not only per-developer GPG keys |
| [CoALA](https://arxiv.org/abs/2309.02427) | memory categories should map to established agent-memory vocabulary where possible |

> [!NOTE]
> These references do not reduce the need for AgentFS. They clarify which parts are prior art and which part is the actual product synthesis: a repository-native substrate combining storage, scope, authority, Git behavior, migration, and human-readable reflection.

---

## 5. Technical Problem Landscape

AgentFS exists because AI repository state is fragmented and operationally weak.

### Failure matrix

| Problem | Immediate Effect | Long-Term Cost |
| --- | --- | --- |
| root clutter | too many AI files in the tree | reduced clarity and weaker adoption |
| token waste | entire text blobs get re-read | higher cost and lower relevance |
| merge conflict friction | branch reconciliation is noisy | collaboration becomes painful |
| authority ambiguity | protected and mutable state drift together | unsafe behavior becomes plausible |
| state fragmentation | every tool invents its own storage | no shared trustworthy substrate |

> [!WARNING]
> If the roadmap solves only retrieval speed but does not solve authority, reflection, Git semantics, and migration, AgentFS will degrade into another partial config tool instead of becoming an actual repository substrate.

---

## 6. Full Technical Architecture at a Glance

```text
┌──────────────────────────────────────────────────────────────┐
│                      AgentFS System Model                    │
├──────────────────────────────────────────────────────────────┤
│  CLI / IDEs / SDKs / Agents / Docs Portal                   │
└───────────────┬───────────────────────┬──────────────────────┘
                │                       │
                ▼                       ▼
      Scoped Retrieval Engine      Git Lifecycle Engine
      - rules                      - merge driver
      - hooks                      - clean filter
      - memory                     - smudge filter
      - logs                       - diff export
      - authority checks           - hook install
                \                       /
                 \                     /
                  ▼                   ▼
                   ┌───────────────────┐
                   │     .agent.db     │
                   │ policy + memory + │
                   │ hooks + logs +    │
                   │ signatures + meta │
                   └───────────────────┘
```

### System layers

| Layer | Responsibility | Failure If Missing |
| --- | --- | --- |
| Storage engine | canonical semantic state | no durable truth surface |
| Context runtime | scope-aware retrieval | tools load noisy or wrong context |
| Git integration | merge/diff/filter/hook behavior | shared state becomes brittle |
| Reflection layer | human-readable surfaces | binary state becomes opaque |
| Security boundary | protected vs mutable domains | unsafe authority drift occurs |

---

## 7. Production-Grade Monorepo Architecture

> [!IMPORTANT]
> The roadmap assumes a serious monorepo from the beginning because the runtime, adapters, docs, CI, and release surfaces all matter to long-term viability.

### Target file tree

```text
agentfs/ (Repository Root)
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
│   │   ├── src/index.ts
│   │   ├── package.json
│   │   └── tsconfig.json
│   └── docs/
│       ├── docs.json
│       └── introduction.mdx
├── crates/
│   ├── afs-core/
│   │   ├── src/
│   │   │   ├── db/mod.rs
│   │   │   ├── vfs/mod.rs
│   │   │   ├── crypto.rs
│   │   │   └── main.rs
│   │   └── Cargo.toml
│   └── afs-git/
│       ├── src/
│       │   ├── merge_driver.rs
│       │   └── hooks.rs
│       └── Cargo.toml
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

### Why this monorepo shape exists

| Area | Responsibility |
| --- | --- |
| `crates/afs-core` | trusted native runtime |
| `crates/afs-git` | Git integration behavior |
| `apps/cli` | user-facing CLI wrapper |
| `apps/docs` | documentation portal |
| root tooling | build, validation, release, and dependency discipline |

---

## 8. Tools and Solutions

### Runtime and language tools

| Tool | Role |
| --- | --- |
| Rust | primary systems implementation language |
| SQLite | embedded semantic record storage |
| DuckDB | optional analytical or semantic extension support |
| memmap2 | memory-mapped access primitives |
| clap | CLI parsing |
| ratatui | terminal UI |
| crossterm | terminal control |
| ed25519-dalek | signature verification support |

### Workflow tools

| Tool | Role |
| --- | --- |
| Turbo | workspace task orchestration and caching |
| Lefthook | fast local pre-commit gates |
| Biome | JS/TS formatting and linting |
| cargo-deny | dependency, license, and bloat enforcement |
| GitHub Actions | CI, release, and security automation |

### Platform-level tools and APIs

| Mechanism | Usage |
| --- | --- |
| FUSE | readable projection on Linux/macOS |
| Projected FS / Dokany-style approach | readable projection on Windows |
| inotify | Linux event-driven file triggers |
| FSEvents | macOS event-driven file triggers |
| ReadDirectoryChangesW | Windows filesystem wakeups |

---

## 9. Runtime Configuration and Binary Footprint Strategy

The roadmap assumes a small trusted core, ideally under roughly `< 5MB` for the main native binary.

### Release configuration example

```toml
[package]
name = "afs"
version = "0.1.0"
edition = "2021"
authors = ["JustineDevs <Justinedevs@jstn.site>"]
description = "The open-source single-file runtime semantic store for AI Agents."

[dependencies]
rusqlite = { version = "0.31.0", features = ["bundled", "limits"] }
memmap2 = "0.9.4"
clap = { version = "4.5.4", features = ["derive", "cargo"] }
ratatui = { version = "0.26.1", default-features = false, features = ["crossterm"] }
crossterm = "0.27.0"
ed25519-dalek = { version = "2.1.1", features = ["rand_core"] }

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

### Why these settings matter

| Setting | Benefit |
| --- | --- |
| `opt-level = "z"` | size-focused optimization |
| `lto = true` | tighter whole-program optimization |
| `codegen-units = 1` | better packing for release builds |
| `panic = "abort"` | less unwinding overhead |
| `strip = true` | smaller shipping binary |

---

## 10. Core DevTools and Guardrails

### `turbo.json`

The repo should avoid rebuilding heavy Rust targets when only docs or TS-only areas change.

```json
{
  "$schema": "https://turbo.build",
  "extends": ["//"],
  "tasks": {
    "build": {
      "dependsOn": ["^build"],
      "outputs": ["dist/**", "target/release/afs"],
      "inputs": ["src/**/*.rs", "src/**/*.ts", "Cargo.toml"]
    },
    "test": {
      "dependsOn": ["build"],
      "outputs": [],
      "inputs": ["tests/**/*.rs", "src/**/*.ts"]
    },
    "lint": {
      "outputs": []
    }
  }
}
```

### `lefthook.yml`

The repo should block low-quality changes locally before commits land.

```yaml
pre-commit:
  parallel: true
  commands:
    rust-lint:
      glob: "*.rs"
      run: cargo clippy --workspace --all-targets -- -D warnings
    rust-format:
      glob: "*.rs"
      run: cargo fmt -- --check
    ts-format-lint:
      glob: "*.{ts,js,json}"
      run: pnpm biome check --apply {staged_files}
    security-audit:
      run: cargo deny check bans licenses sources
```

### `deny.toml`

The core binary should stay small, legally clean, and dependency-disciplined.

```toml
[licenses]
unlicensed = "deny"
allow = [ "MIT", "Apache-2.0", "BSD-3-Clause" ]
deny = [ "GPL-3.0", "AGPL-3.0" ]

[bans]
multiple-versions = "deny"
deny = [
    { name = "tokio", version = "*", features = ["full"] }
]
skip-tree = []
```

### Guardrail summary

| File | Purpose |
| --- | --- |
| `turbo.json` | smart monorepo caching |
| `lefthook.yml` | local quality gate enforcement |
| `deny.toml` | bloat, license, and dependency guardrails |

---

## 11. Git Lifecycle and `.gitattributes` Strategy

`.gitattributes` helps, but it is not enough alone.

### What `.gitattributes` can capture

| Mechanism | Trigger Surface | AgentFS Use |
| --- | --- | --- |
| merge driver | `merge`, `pull`, `rebase` | reconcile `.agent.db` semantically |
| clean filter | `add` | normalize/compress before staging |
| smudge filter | `checkout`, `clone`, `switch` | refresh local projected state |
| diff driver | `diff`, `show` | readable binary-backed review |

### What `.gitattributes` cannot capture

| Missing Surface | Why It Still Needs Hooks |
| --- | --- |
| `git commit` | needs validation before history is sealed |
| `git push` | needs leak or trust checks before publishing |
| `git status` | not a transform path |
| general user intent | file attributes do not understand workflow semantics |

### Therefore the complete Git solution is:

- `.gitattributes`
- merge driver
- clean filter
- smudge filter
- diff export
- native Git hooks

### Lifecycle diagram

```text
                 ┌───────────────────────────────┐
                 │      DEVELOPER COMMAND        │
                 └──────────────┬────────────────┘
                                │
        ┌───────────────────────┴───────────────────────┐
        ▼                                               ▼
[ Triggers Git Hooks ]                      [ Triggers .gitattributes ]
• git commit  ──► pre-commit               • git add      ──► clean filter
• git push    ──► pre-push                 • git checkout ──► smudge filter
• git merge   ──► post-merge               • git merge    ──► merge driver
```

---

## 12. Merge Driver and Team Synchronization

Because `.agent.db` is a critical binary-backed state surface, Git branch collaboration needs a custom reconcile path.

### Team sync diagram

```text
 Developer A's Machine                      Developer B's Machine
┌───────────────────────────┐              ┌───────────────────────────┐
│  Modified .agent.db       │              │  Modified .agent.db       │
└─────────────┬─────────────┘              └─────────────┬─────────────┘
              │                                          │
              ▼                                          ▼
      git push / merge path                     git pull / merge path
              │                                          │
              └──────────────────► 💾 ◄──────────────────┘
                                   │
                                   ▼
                        [ afs-merge-driver ]
                        • compares rows
                        • resolves timestamps
                        • preserves authority
                        • emits fresh binary
```

### Required merge-driver properties

| Requirement | Why |
| --- | --- |
| row-aware reconciliation | text-only diff semantics are insufficient |
| timestamp/conflict policy | concurrent changes need deterministic resolution |
| authority preservation | protected policy must not be casually overwritten |
| deterministic output | teams need repeatable results |

---

## 13. Reflection and Compatibility Strategy

The roadmap must solve both human readability and tool compatibility.

### Reflection modes

| Mode | Purpose |
| --- | --- |
| markdown export | quick human inspection |
| JSON export | machine-readable audit |
| virtual projection | readable local surfaces without repo clutter |
| generated local compatibility files | support legacy tools that still require text files |

### Compatibility problem

Many tools still expect:

- `.cursorrules`
- `.github/copilot-instructions.md`
- `AGENTS.md`

AgentFS therefore needs:

- a virtual reflection layer
- or temporary local generated files
- or future SDK / extension adapters

### Reflection diagram

```text
.agent.db
   │
   ├── export --format markdown
   ├── export --format json
   └── .afs/virtual/
        ├── rules/
        ├── hooks/
        ├── memory/
        └── logs/
```

---

## 14. Runtime Command Surface

### Core commands

```bash
afs init
afs add rule "Always use strict TypeScript. No 'any' types." --scope /Folder-1
afs add hook pre-commit "pnpm test" --scope global
afs status
afs doctor
afs export --format=markdown > audit_rules.md
afs ingest --clean
```

### Command matrix

| Command | Role |
| --- | --- |
| `afs init` | create `.agent.db`, register Git integrations, provision reflection |
| `afs add rule` | add path-scoped policy |
| `afs add hook` | register trusted lifecycle commands |
| `afs status` | inspect runtime state |
| `afs doctor` | verify integrity and local setup |
| `afs export` | flatten semantic state into readable output |
| `afs ingest` | migrate existing AI config clutter |

---

## 15. Example Python Prototype

The original technical direction included a lightweight prototype shape to prove the single-file model works even before the full native implementation.

```python
import sqlite3
import os

DB_FILE = ".agent.db"

def init_agent_storage():
    conn = sqlite3.connect(DB_FILE)
    cursor = conn.cursor()
    cursor.execute("PRAGMA journal_mode=WAL;")
    cursor.execute("PRAGMA synchronous=NORMAL;")
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS agent_data (
            id TEXT PRIMARY KEY,
            category TEXT,
            scope TEXT,
            content TEXT,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
    """)
    conn.commit()
    conn.close()

def query_agent_context(scope_name, category_filter):
    conn = sqlite3.connect(DB_FILE)
    cursor = conn.cursor()
    cursor.execute("""
        SELECT content FROM agent_data
        WHERE (scope = ? OR scope = 'global') AND category = ?
    """, (scope_name, category_filter))
    results = [row[0] for row in cursor.fetchall()]
    conn.close()
    return results
```

### Prototype value

| Why It Matters | Meaning |
| --- | --- |
| proves single-file feasibility | the concept is buildable with existing primitives |
| proves scoped retrieval | path-aware lookup is easy to model |
| proves local-first operation | no server is required to start |

---

## 16. Performance Model

AgentFS makes four major runtime promises.

| Promise | Meaning |
| --- | --- |
| Zero idle CPU | no constant polling loops |
| Fast scoped reads | local relevant context is cheap to fetch |
| Bounded memory | retrieval layers must fit within predictable budgets |
| Small trusted binary | installation and auditing remain practical |

### Zero-idle rule

The runtime should:

- wake only when needed
- use filesystem events when required
- avoid permanent busy loops
- terminate lightweight CLI operations quickly

### Why idle CPU can stay near zero

| Mechanism | Benefit |
| --- | --- |
| one-shot CLI execution | command exits when work is done |
| passive event loops | waiting does not consume active CPU |
| mapped reads | less file-descriptor churn |
| OS-native wakeups | activity only happens when triggered |

---

## 17. Concurrency and Multi-Agent Coordination

Multiple agents create real coordination risks.

### Concurrency hazards

| Hazard | Description |
| --- | --- |
| direct write contention | many agents write to the same DB simultaneously |
| lock conflicts | one scope is touched by multiple agents |
| stale reads | one agent reads before another write is durable |
| unsafe memory drift | mutable state bleeds into protected behavior |

### Planned controls

| Control | Purpose |
| --- | --- |
| scope locks | isolate subsystem work |
| row mutexes | protect fine-grained state |
| append-only writer | serialize writes safely |
| actor-style appender | keep agents out of direct concurrent DB writes |

### Appender diagram

```text
Agent A ----\
             \
Agent B ------> [ Named Pipe / Ring Buffer ] --> [ Single Appender Thread ] --> .agent.db
             /
Agent C ----/
```

### Tradeoff

Writes become eventually consistent across readers for a brief window, but safety and correctness improve significantly.

---

## 18. Semantic Retrieval, Compression, and TTL

Semantic retrieval is useful only if it stays lightweight.

### Techniques in scope

| Technique | Intended Benefit |
| --- | --- |
| Binary Quantization (BQ) | major embedding footprint reduction |
| Scalar Quantization (SQ) | lighter storage than full precision |
| truncated embeddings | smaller semantic payloads |
| semantic hashing | low-cost approximate retrieval |
| LRU / sliding windows | bounded active memory |
| TTL pruning | temporary memory does not become permanent clutter |

### Risk matrix

| Risk | Mitigation |
| --- | --- |
| vector explosion | quantization, truncation, compaction |
| loading too much memory | LRU and sliding windows |
| stale temporary lessons | TTL-based expiry or archive |
| loss of semantic accuracy | accepted tradeoff for local bounded operation |

---

## 19. Migration and Adoption Pipeline

AgentFS cannot succeed without a low-friction migration path.

### Core migration command

```bash
afs ingest --clean
```

### Ingestion lifecycle

```text
┌─────────────────────────────────────────────────────────────────────────┐
│                           INGESTION LIFECYCLE                           │
├───────────────────┬─────────────────────────────────────────────────────┤
│ 1. Scan           │ find AI files such as .cursorrules and AGENTS.md   │
├───────────────────┼─────────────────────────────────────────────────────┤
│ 2. Parse & Scope  │ map their contents into exact repository paths      │
├───────────────────┼─────────────────────────────────────────────────────┤
│ 3. Store / Index  │ write canonical rows into .agent.db                 │
├───────────────────┼─────────────────────────────────────────────────────┤
│ 4. Erase & Stream │ remove duplicates and activate reflection support    │
└───────────────────┴─────────────────────────────────────────────────────┘
```

### Adoption benefits

| Benefit | Meaning |
| --- | --- |
| zero lost work | existing repository guidance is preserved |
| clean workspace | duplicate file clutter can be removed |
| one authority surface | tools converge on one semantic store |

---

## 20. Deployment Modes

The roadmap should support three operating modes without changing the core conceptual model.

| Mode | Best Fit | Tradeoff |
| --- | --- | --- |
| 100% local offline | privacy-sensitive or solo development | no built-in remote sync |
| static cloud distribution | lightweight team sharing | remote distribution complexity |
| peer-distributed | decentralized specialized environments | highest coordination complexity |

### Comparison

| Feature | Static Cloud | 100% Local Offline | P2P Team Network |
| --- | --- | --- | --- |
| Requires running server? | No | No | No |
| Network needed? | Yes | No | Yes |
| Setup complexity | Low | Very low | Medium |
| Best fit | distributed remote teams | solo/private work | decentralized teams |

---

## 21. Enterprise Governance Roadmap

Enterprises care about more than local convenience.

### Enterprise requirements

| Requirement | Architectural Response |
| --- | --- |
| upstream policy | remote signed policy deltas |
| tamper resistance | SSH/GPG-backed protected writes |
| auditability | readable exports and controlled logs |
| non-escalating automation | dual-domain trust boundaries |

### Enterprise flow

```text
Enterprise Policy Source
          │
          ▼
Signed Delta
          │
          ▼
Local AgentFS Runtime
          │
          ▼
Validated Merge into Protected Policy Domain
```

---

## 22. Platform and Distribution Strategy

AgentFS is intended to be universally usable across OS platforms and across agent ecosystems.

### Platform diagram

```text
                  ┌───────────────────────────┐
                  │    AgentFS Native Core    │
                  └─────────────┬─────────────┘
                                │
       ┌────────────────────────┼────────────────────────┐
       ▼                        ▼                        ▼
    macOS                    Windows                   Linux
   FSEvents + mmap           Win32 APIs                inotify + mmap
   Apple Silicon + Intel     x86_64 / ARM64            desktop / server / CI
```

### Installation surface

```bash
curl -fsSL https://agentfs.systems | sh
```

Potential package manager paths:

- `brew install agentfs`
- `winget install agentfs`
- `apt install agentfs`
- `apk add agentfs`

---

## 23. Interactive Runtime UX

The roadmap also assumes a lightweight terminal interface for operational inspection.

```text
┌─ AgentFS v0.1.0 ───────────────────────── Systems Status: ACTIVE ──┐
│ Active File: /projects/my-monorepo/.agent.db [Size: 4.2 MB]        │
│ Memory Footprint: 12.4 MB / 50.0 MB Max (LRU Enabled)              │
└─────────────────────────────────────────────────────────────────────┘
┌─ Workspace Topology ───────────────────┐┌─ Live Agent Context ─────┐
│ Global Shared Context                  ││ Scope: /Folder-1         │
│ Folder-1: 4 Rules / 2 Hooks            ││ Rule-01 Strict Types     │
│ Folder-2: 2 Rules / 0 Hooks            ││ Hook-01 pre-commit       │
└────────────────────────────────────────┘└───────────────────────────┘
┌─ Security & Verification ──────────────┐┌─ Recent Sync Ledger ─────┐
│ Signed Entries: 6/6                    ││ merge: OK                │
│ Guardrails: STRICT                     ││ memory log               │
│ Authority: SSH / GPG verified          ││ init checkpoint          │
└────────────────────────────────────────┘└───────────────────────────┘
```

### UI purpose

| UI Surface | Why It Exists |
| --- | --- |
| active file view | show the canonical state surface |
| memory budget view | prove the bounded-runtime promise |
| topology view | expose scope distribution |
| security panel | expose trust state clearly |
| sync ledger | show recent lifecycle behavior |

---

## 24. Full Challenge Register

| Challenge | Why It Matters | Required Answer |
| --- | --- | --- |
| IDE compatibility wall | current tools expect text files | reflection, adapters, or generated compatibility |
| Git binary diff problem | plain text reconciliation is insufficient | merge driver + diff export |
| corruption risk | `.agent.db` becomes critical state | WAL, checkpoints, validation |
| human readability | binary-only systems lose trust | export and virtual projection |
| file-size growth | semantic data can balloon | quantization, truncation, TTL, compaction |
| race conditions | multiple agents write concurrently | locks, appenders, actor-like serialization |
| hook escalation risk | agents might mutate execution paths | protected human-only domains and signatures |

---

## 25. Tradeoff Matrix

| Area | Innovative Blueprint | Hard Tradeoff |
| --- | --- | --- |
| Data storage | single binary hybrid semantic store | plain Git text diffs are inadequate |
| IDE integration | reflection, adapters, generated compatibility | higher implementation complexity |
| Concurrency | single appender / scoped locking | eventual consistency windows |
| Memory retrieval | compressed local semantic search | some semantic fidelity may be sacrificed |
| Human UX | clean repository root | reflection layer becomes mandatory |
| Security | cryptographically protected policy | agents cannot self-modify execution paths freely |

---

## 26. Phase-by-Phase Delivery Plan

### Phase 0: Foundation

Goal:

- shared architecture language
- docs truth
- repo conventions
- governance surfaces

Exit criteria:

- README, Concept, TA, Architecture, and Roadmap agree
- governance and legal files exist
- repository claims are honest

### Phase 1: Local runtime baseline

Goal:

- `.agent.db`
- `afs init`
- `afs status`
- `afs doctor`
- basic scoped CRUD

Exit criteria:

- local repository can initialize AgentFS
- rules can be stored and queried by scope
- basic integrity checks exist

### Phase 2: Git lifecycle integration

Goal:

- merge driver
- clean filter
- smudge filter
- diff export
- hook bootstrap

Exit criteria:

- `.agent.db` participates safely in Git workflows
- binary-backed state is inspectable in review

### Phase 3: Reflection and compatibility

Goal:

- export surfaces
- virtual projection or generated compatibility files
- support for legacy AI-file ecosystems

Exit criteria:

- humans can inspect state
- legacy tools can still work during migration

### Phase 4: Multi-agent runtime safety

Goal:

- locks
- append-only writer
- bounded concurrent mutation model

Exit criteria:

- multiple agents can share state without corrupting it

### Phase 5: Semantic retrieval optimization

Goal:

- compressed embeddings or semantic hashing
- bounded memory windows
- TTL pruning

Exit criteria:

- retrieval remains local, small, and cost-effective

### Phase 6: Enterprise governance

Goal:

- signed policy deltas
- upstream remote governance
- stronger audit and trust workflows

Exit criteria:

- organizations can adopt AgentFS under real governance constraints

### Delivery summary table

| Phase | Main Outcome | Why It Must Happen Here |
| --- | --- | --- |
| 0 | shared language and repo truth | prevents incoherent implementation |
| 1 | local usable runtime | the core system must exist before integrations |
| 2 | Git-safe project state | shared adoption depends on Git viability |
| 3 | readable and compatible system | migration and human trust depend on it |
| 4 | safe concurrent use | multi-agent behavior only matters after single-agent correctness |
| 5 | optimized semantic retrieval | performance tuning follows correctness |
| 6 | enterprise governance | advanced controls come after the core trust model exists |

### Phase decision register

> [!IMPORTANT]
> These decisions are implementation blockers, not optional polish. If they are deferred too long, AgentFS risks becoming a fast context lookup tool instead of a durable repository substrate.

#### Phase 1: Local runtime baseline

| Decision | Required Answer | Risk If Ignored |
| --- | --- | --- |
| D1: SQLite WAL setup | define `PRAGMA journal_mode=WAL`, checkpoint timing, crash behavior, and how `.agent.db-wal` / `.agent.db-shm` are explained | developers see unexplained sidecar files or encounter unclear recovery behavior |
| D2: schema migration strategy | embed schema versioning before user files exist | later phases break early `.agent.db` files |
| D3: scope inheritance resolution | implement path-walking inheritance, override precedence, and parent fallback | scoped rules return incomplete or surprising context |
| D4: cross-platform hook installation | support Windows behavior and hook chaining with existing systems like Husky | `afs init` breaks existing repo automation |
| D5: `.gitignore` vs `.gitattributes` | decide whether `.agent.db` is committed or ignored for the preview collaboration model | team sharing and Git integration become incoherent |

#### Phase 2: Git lifecycle integration

| Decision | Required Answer | Risk If Ignored |
| --- | --- | --- |
| D6: merge driver correctness | real three-way SQLite merge tests with deterministic conflict handling | one corrupt merge destroys trust in the project |
| D7: clean/smudge filter stability | filters must be idempotent and handle missing `.agent.db` | Git staging or checkout becomes unsafe |
| D8: readable diff output | export format must work for local `git diff` and GitHub PR review | reviewers see only binary noise |
| D9: `.gitattributes` auto-registration | `afs init` must wire merge, diff, and filters with graceful degradation if `afs` is absent | clones without `afs` fail silently or fall back to binary conflicts |

#### Phase 3: Migration and compatibility

| Decision | Required Answer | Risk If Ignored |
| --- | --- | --- |
| D10: vendor parsing heuristics | `afs ingest` needs `--dry-run`, preview, and review because source files are freeform | destructive ingest misclassifies real policy |
| D11: scope assignment during ingest | define how root-level and nested vendor files map to scopes | users inherit rules in the wrong places |
| D12: IDE compatibility generation | generate or reflect vendor-specific files from `.agent.db` where tools still require them | migration breaks Cursor, Copilot, Claude, or other existing workflows |

#### Phase 4: Multi-agent runtime

| Decision | Required Answer | Risk If Ignored |
| --- | --- | --- |
| D13: SQLite concurrent write limits | define busy timeout, retry policy, and write serialization | simultaneous agents see lock errors or silent write loss |
| D14: scope-level lock granularity | use lock rows with TTL or heartbeat expiry | crashed agents permanently block scopes |
| D15: append-only event ordering | sequence counter or timestamp strategy with clock-skew awareness | event history becomes non-deterministic |

#### Phase 5: Semantic retrieval

| Decision | Required Answer | Risk If Ignored |
| --- | --- | --- |
| D16: embedding storage bloat | use quantization, truncation, or compact embeddings early | `.agent.db` balloons from a system file into an opaque artifact |
| D17: local embedding dependency | decide tiny local model, optional embeddings, or pluggable providers | the small trusted binary promise collapses under model weight |
| D18: TTL pruning correctness | prune safely with reference awareness | active memory or referenced logs are deleted accidentally |

#### Phase 6: Enterprise controls

| Decision | Required Answer | Risk If Ignored |
| --- | --- | --- |
| D19: signature scheme design | define key distribution, recovery, per-user vs per-repo authority, and possible Sigstore path | "signature" becomes a column without real PKI semantics |
| D20: remote registry sync conflicts | define authority hierarchy for local overrides vs remote signed policy | policy merges become the same problem as Git merges with higher stakes |

#### Cross-cutting decisions

| Decision | Required Answer | Risk If Ignored |
| --- | --- | --- |
| D21: binary mistrust | export and reflection must ship early | users reject `.agent.db` before transparency tooling exists |
| D22: code-enforced dual-domain boundary | `afs-core` must reject agent writes to user-domain records | documentation-only trust boundary fails under automation |
| D23: layer boundary discipline | keep `afs-core`, `afs-git`, and CLI UX responsibilities separate | convenience logic leaks across layers and testing becomes brittle |
| D24: roadmap trap | do not skip Git and migration in favor of speed plus embeddings | AgentFS becomes another partial tool, not a substrate |

---

## 27. Release Targets

> [!TIP]
> Version tags must remain sequencing anchors until the described behavior exists and is verifiable. Documentation alone is not completion.

| Release Target | Main Goal |
| --- | --- |
| `pre-idea-0.1.0` | documentation-first system definition |
| `v0.1.0` | initial local runtime + Git story + reflection baseline |
| `v0.1.5` | mutex and concurrency safety improvements |
| `v0.2.0` | snapshots, rollback, TTL, richer semantic lifecycle |
| `v0.3.0` | enterprise governance and stronger distribution models |

---

## 28. Operational Verification Flow

```text
[ Contributor Code Update ] ──► [ Lefthook Blocks/Passes Local Formatting ]
                                          │
                                          ▼
[ Pull Request Opened ]     ──► [ CI Validates Cross-OS Compilation ]
                                          │
                                          ▼
[ Changeset Merged ]        ──► [ Release Pipeline Ships Native Binaries ]
```

### Pipeline meaning

| Step | Why It Exists |
| --- | --- |
| local hooks | catch low-quality or unsafe changes early |
| CI validation | confirm cross-platform build viability |
| release pipeline | ship trusted native artifacts |

---

## 29. Final Roadmap Definition

The AgentFS roadmap is a systems-sequencing plan for turning a single-file semantic repository concept into a production-grade local-first runtime with Git integration, human reflection, compressed retrieval, safe concurrency, and protected authority boundaries.

## Project Identity

| Field | Value |
| --- | --- |
| Project | `AgentFS` |
| CLI | `afs` |
| Roadmap Scope | process, phases, tools, syntax, risks, sequencing, operational model |
| Author | `@Justinedevs` |
| Email | `Justinedevs@jstn.site` |
| Domain | `agentfs.systems` |

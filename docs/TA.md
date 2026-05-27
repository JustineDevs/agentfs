# AgentFS TA

> [!IMPORTANT]
> `TA.md` is the single-file Technical Architect reference for AgentFS. It is intended to hold the deepest architectural context for the project in one place: problem framing, system model, monorepo design, syntax, variables, tools, diagrams, tradeoffs, migration, security, concurrency, platform behavior, and delivery sequencing.

> [!NOTE]
> This repository is still documentation-first. This file describes the intended production system, the terminology it uses, the design research it draws from, and the implementation constraints it must obey. It is not a claim that every described subsystem already exists in this checkout.

---

## 1. Executive Definition

AgentFS is a local-first, repository-native semantic operating layer for AI-assisted software development.

Its central claim is:

> AI repository state should stop living as scattered text clutter and start living as governed infrastructure.

In practical terms, AgentFS moves rules, skills, hooks, memory, logs, and lifecycle state into a single hidden project file, typically `.agent.db`, and layers on:

- path-aware retrieval
- Git-native lifecycle behavior
- human-versus-agent authority separation
- offline-first performance
- reflection and export for human review

---

## 2. Project Mission

The mission of AgentFS is to create the open-source "USB port" for AI-native repositories:

- one repository-local semantic store
- one runtime context model
- one trust boundary model
- one Git integration model
- one migration story from today's fragmented tool-specific files

### Mission Outcomes

| Outcome | Meaning |
| --- | --- |
| Clean repositories | Fewer AI-specific clutter files in the root and subfolders |
| Better context precision | Only relevant records are loaded for the current path |
| Better collaboration | Shared state can be merged and inspected predictably |
| Better governance | Humans retain authority over protected policy and executable hooks |
| Better performance | Local-first access keeps reads fast and idle overhead near zero |

---

## 3. Problem Statement

The current AI tooling ecosystem treats repository intelligence as a loose collection of text files. That model is convenient at small scale and weak at systems scale.

### The file sprawl pattern

Modern repositories accumulate:

- `.cursorrules`
- `AGENTS.md`
- `copilot-instructions.md`
- hidden prompt folders
- tool-specific YAML or JSON
- undocumented local scratch files
- uncontrolled runtime memory blobs

### System-level failures this causes

| Failure | What Happens | Why It Hurts |
| --- | --- | --- |
| Root clutter | Machine-facing files leak into the human-facing repo surface | Developers lose a clean mental model |
| Prompt bloat | Entire files are re-read or reinjected into LLM prompts | Higher token cost, lower relevance |
| Merge conflict pressure | Human and tool edits collide in normal Git flows | Team collaboration becomes brittle |
| State fragmentation | Each tool invents its own truth surface | Different agents see different reality |
| Trust ambiguity | Hooks, rules, and memory all share the same edit plane | Unsafe privilege drift becomes possible |

### The deeper problem

The deeper issue is not merely "too many files." The deeper issue is that AI repository state has no canonical systems substrate.

Without that substrate:

- policy has weak ownership
- memory has weak boundaries
- Git has no reliable state semantics
- runtime context stays noisy
- multiple agents cannot coordinate safely

---

## 4. Architecture Thesis

AgentFS is shaped by seven architecture theses.

| Thesis | Architectural Meaning |
| --- | --- |
| Context is data | Repository intelligence should be queryable and structured |
| Scope is first-class | Directory path must influence retrieval |
| Category matters | Rules, hooks, memory, skills, and logs are not interchangeable |
| Authority matters | Human-owned policy and agent-owned state require different write rules |
| Git is part of the runtime | Merge, diff, commit, push, and checkout behavior are product surfaces |
| Local-first is the default | The canonical store should live with the repository |
| Reflection is mandatory | Binary-backed state must remain human-reviewable |

---

## 5. Core System Model

At the center of AgentFS is one hidden project file:

```text
.agent.db
```

This file is intended to behave as:

- a semantic store
- a local runtime substrate
- a path-scoped policy registry
- a hook and lifecycle control plane
- a synchronization surface for agents
- a Git-aware state object

### High-level system diagram

```text
┌──────────────────────────────────────────────────────────────┐
│                        AgentFS Runtime                       │
├──────────────────────────────────────────────────────────────┤
│  Human Tools / IDEs / Agents / SDKs / CLI                   │
└───────────────┬───────────────────────────────┬──────────────┘
                │                               │
                ▼                               ▼
      ┌───────────────────┐           ┌────────────────────┐
      │ Scoped Retrieval  │           │ Git Lifecycle      │
      │ Engine            │           │ Integration        │
      │ - rules           │           │ - merge driver     │
      │ - hooks           │           │ - clean filter     │
      │ - memory          │           │ - smudge filter    │
      │ - logs            │           │ - diff export      │
      └─────────┬─────────┘           └─────────┬──────────┘
                │                               │
                └──────────────┬────────────────┘
                               ▼
                    ┌──────────────────────┐
                    │ .agent.db            │
                    │ - rules              │
                    │ - skills             │
                    │ - hooks              │
                    │ - memory             │
                    │ - logs               │
                    │ - signatures         │
                    │ - snapshots          │
                    └──────────────────────┘
```

---

## 6. Research and Technical Parallels

AgentFS is not invented from nowhere. It deliberately adapts proven systems patterns.

| Reference System / Concept | What AgentFS Borrows |
| --- | --- |
| SQLite | single-file structured persistence |
| DuckDB | analytical and embedded local query model |
| LMDB | memory-mapped local data access ideas |
| Apache Arrow | zero-copy and columnar access inspiration |
| Git | repository-local truth, lifecycle semantics, merge workflows |
| Docker image layers | single artifact carrying structured internal state |
| FUSE / projected filesystems | readable virtual views over non-text-native storage |
| Actor model / append queue | serialized writes for concurrency safety |
| Vector DB techniques | semantic retrieval, but only if kept lightweight and bounded |

### Research position

AgentFS is best understood as a synthesis of:

- embedded databases
- filesystem semantics
- Git operations
- local-first software
- AI retrieval infrastructure

### Additional prior art that constrains implementation

> [!IMPORTANT]
> AgentFS should reuse proven infrastructure patterns where possible. The project is novel in how it combines storage, authority, Git lifecycle, and agent memory, not in pretending every primitive must be invented from scratch.

| Prior Art | Relevance to AgentFS | Architectural Consequence |
| --- | --- | --- |
| [`sqlite-vec`](https://github.com/asg017/sqlite-vec) / [`sqlite-vss`](https://github.com/asg017/sqlite-vss) | SQLite-native vector search extensions | `crates/afs-core/src/vector/` should evaluate extension-backed vector search before implementing all vector logic internally |
| [Git LFS](https://github.com/git-lfs/git-lfs/blob/main/docs/spec.md) | clean/smudge filters and pointer-file handling for non-text payloads | `crates/afs-git/` should treat filter idempotency, pointer stability, and graceful fallback as first-order requirements |
| [git-annex](https://git-annex.branchable.com/git-annex/) | Git-managed indirection for large or externalized file contents | AgentFS can learn from Git-native metadata indirection without copying git-annex's content-addressed file model |
| [Git attributes](https://git-scm.com/docs/gitattributes) | merge, diff, clean, and smudge behavior | `.gitattributes` is part of the product contract, not incidental setup |
| [Sigstore / Cosign](https://docs.sigstore.dev/cosign/signing/overview/) | OIDC-backed keyless signing and transparency logs | Phase 6 enterprise controls should consider Sigstore-style identity-backed signatures, not only raw GPG key workflows |
| [CoALA](https://arxiv.org/abs/2309.02427) | formal memory categories for language agents | AgentFS should map `rule`, `skill`, `hook`, and `memory` to working, episodic, semantic, and procedural memory vocabulary where useful |

#### sqlite-vec / sqlite-vss

AgentFS plans compact semantic vectors stored close to the `.agent.db` record model. `sqlite-vec` and its predecessor `sqlite-vss` prove that vector search can live directly inside SQLite through an extension model. That makes them natural references for `crates/afs-core/src/vector/`.

The design implication is not "depend immediately." The design implication is that AgentFS should define a vector provider boundary:

| Vector Concern | Preview Direction | Later Direction |
| --- | --- | --- |
| Storage | SQLite BLOBs or extension tables | quantized vector payloads with provider-specific adapters |
| Query | exact or approximate local matching | extension-backed ANN where available |
| Portability | no hard dependency in the minimal core | optional feature flags for vector engines |
| Size | avoid heavy local models by default | support pluggable embeddings and compression |

#### FUSE and virtual filesystem precedents

The VFS reflection layer planned for `crates/afs-core/src/vfs/` is not speculative in the abstract. FUSE and platform-specific projected filesystem systems have already proven the idea that non-text or indirect storage can be reflected into a human-readable tree.

Git LFS and git-annex are also relevant even though they solve different problems. They show that Git can be extended to handle objects that are not plain text, provided the indirection model is deterministic and reviewable.

#### Sigstore / Cosign

AgentFS currently describes SSH/GPG-style signatures for protected records. That remains a useful local baseline, but enterprise policy delivery should consider Sigstore and Cosign because keyless signing with OIDC identity reduces long-lived key handling and introduces transparency-log auditability.

For AgentFS, that changes the Phase 6 question from:

```text
who owns the private key?
```

to:

```text
which identity provider, transparency log, and verification policy authorize this policy delta?
```

#### Cognitive Architecture Research: CoALA

The CoALA paper formalizes language-agent memory around working memory, episodic memory, semantic memory, and procedural memory. AgentFS does not need to copy that taxonomy exactly, but it should avoid inventing private names where established vocabulary helps adoption.

| CoALA Vocabulary | AgentFS Mapping |
| --- | --- |
| Working memory | active scoped context for the current path or task |
| Episodic memory | append-only agent logs, snapshots, and event history |
| Semantic memory | durable facts, rules, and repository knowledge |
| Procedural memory | skills and hooks that encode operational behavior |

---

## 7. Fundamental Concepts and Terms

This section defines the core vocabulary used across the project.

### Core terms

| Term | Meaning |
| --- | --- |
| `AgentFS` | the project and architectural model |
| `afs` | the CLI and operator-facing command name |
| `.agent.db` | canonical hidden semantic store at repository root |
| `scope` | the path or namespace where a record applies |
| `category` | high-level record type such as rule, hook, or memory |
| `user domain` | human-controlled protected policy surface |
| `agent domain` | mutable agent-writable runtime surface |
| `reflection layer` | human-readable export or virtual projection of binary-backed state |
| `merge driver` | custom Git integration used to reconcile `.agent.db` |
| `clean filter` | Git filter invoked during staging |
| `smudge filter` | Git filter invoked on checkout/switch/clone materialization |
| `TTL` | time-to-live policy for temporary memory rows |
| `VFS` | virtual filesystem behavior or projection over semantic state |
| `BQ` | binary quantization for compact embeddings |
| `SQ` | scalar quantization for reduced vector footprint |
| `LRU` | least recently used memory/window caching strategy |

### System-level concepts

| Concept | Why It Matters |
| --- | --- |
| Path inheritance | local scopes can inherit global or parent rules |
| Deterministic reconciliation | binary-backed state still needs predictable merge behavior |
| Structured mutability | not every record can be equally writable |
| Offline-first operation | privacy and speed are preserved by default |
| Event-driven lifecycle | idle CPU should stay near zero |

---

## 8. Categories, Variables, and Internal Semantics

AgentFS is not only a storage idea. It is a semantic model with well-defined variables.

### Canonical record variables

| Variable | Type | Purpose |
| --- | --- | --- |
| `id` | text | stable identity for a record |
| `category` | enum/text | record type such as rule, skill, hook, memory |
| `scope` | text | path or namespace where the record applies |
| `content` | text/blob | primary payload |
| `signature` | text/blob | proof of human authority for protected records |
| `embedding` | blob | compact semantic retrieval payload |
| `ttl_seconds` | integer | pruning policy for temporary state |
| `updated_at` | timestamp | change ordering and reconciliation |
| `source` | text | origin of migrated or imported records |
| `authority_zone` | enum | `user` or `agent` ownership boundary |
| `snapshot_id` | text | rollback grouping or state checkpoint identifier |

### Core category set

| Category | Intended Meaning | Write Model |
| --- | --- | --- |
| `rule` | human-authored policy and instructions | protected |
| `skill` | reusable structured workflows or commands | mostly protected |
| `hook` | lifecycle triggers and shell targets | protected |
| `memory` | agent-visible task memory and observations | mutable |
| `log` | runtime event output | mutable |
| `snapshot` | rollback or time-travel state | controlled |

---

## 9. Conceptual Schema

### Minimal conceptual schema

```sql
CREATE TABLE agent_data (
    id TEXT PRIMARY KEY,
    category TEXT CHECK(category IN ('rule', 'skill', 'hook', 'memory')),
    scope TEXT NOT NULL,
    content TEXT NOT NULL,
    signature TEXT,
    embedding BLOB,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

### Extended schema direction

```sql
CREATE TABLE agent_data (
    id TEXT PRIMARY KEY,
    category TEXT NOT NULL,
    scope TEXT NOT NULL,
    content TEXT NOT NULL,
    authority_zone TEXT NOT NULL CHECK(authority_zone IN ('user', 'agent')),
    signature TEXT,
    source TEXT,
    embedding BLOB,
    ttl_seconds INTEGER,
    snapshot_id TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE locks (
    id TEXT PRIMARY KEY,
    scope TEXT NOT NULL,
    owner TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP
);

CREATE TABLE snapshots (
    id TEXT PRIMARY KEY,
    label TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

### Internal path syntax examples

The original concept implies that internal semantic addressing may eventually use shapes like:

```text
ai://rules/syntax
ai://state/memory
ai://skills/git/commit
ai://hooks/pre-commit
```

That syntax is useful because it expresses:

- category
- location
- semantic intent

---

## 10. Scope Model

Scope is one of the most important primitives in AgentFS.

### Scope examples

```text
global
/apps/web
/apps/docs
/crates/afs-core
/packages/sdk-ts
/docs
```

### Scope semantics

| Question | Why Scope Must Answer It |
| --- | --- |
| What applies here? | local rules should be precise |
| What is inherited? | global defaults still matter |
| What overrides what? | nested specialization needs deterministic behavior |
| What memory is relevant? | retrieval must not be globally noisy |

### Scope resolution flow

```text
current path
   │
   ▼
/apps/web/components
   │
   ├── inherits from /apps/web
   ├── inherits from /apps
   └── inherits from global
```

---

## 11. Single-File Virtual Filesystem Model

The core conceptual leap is that the project file behaves more like:

- an embedded repository state image
- an SQLite or DuckDB file
- a local semantic filesystem

than like a flat markdown note.

### Concept diagram

```text
Monorepo Root/
├── Folder-1/
├── Folder-2/
└── .agent.db

Internal conceptual view of .agent.db
├── rules/
│   ├── global
│   └── Folder-1
├── skills/
│   ├── git/
│   └── deploy/
├── hooks/
│   ├── pre-commit
│   └── post-test
├── memory/
│   ├── Folder-1
│   └── Folder-2
└── logs/
```

### Why this is better than file sprawl

| Text-File Model | AgentFS Model |
| --- | --- |
| many files compete for authority | one canonical store owns semantic truth |
| large text blobs are repeatedly re-read | relevant records can be selected by scope and category |
| hooks, memory, and rules blur together | structure preserves meaning and ownership |
| Git sees only text clutter | Git integration can be explicitly designed around the project file |

---

## 12. Memory-Mapped Access Model

### Core idea

Instead of sequentially reading many config files, the runtime uses OS-level memory mapping (`mmap`) to expose the project file directly into process memory space.

### Technical meaning

| Property | Result |
| --- | --- |
| zero-parse retrieval | fewer repeated string parsing costs |
| on-demand page loading | OS only loads touched portions of the file |
| lower idle overhead | system sleeps when not actively serving reads or writes |
| strong locality | path-scoped reads stay fast |

### Architectural rationale

This matters because the design target is:

- fast local access
- minimal CPU drain
- bounded memory
- no noisy always-on service unless explicitly necessary

---

## 13. Dual-Engine Design: Symbolic + Semantic

Traditional configuration formats are purely symbolic. AgentFS must support both explicit policy and semantic retrieval.

### Dual-engine table

| Engine | Purpose | Example Contents |
| --- | --- | --- |
| Relational / symbolic | deterministic policy and commands | rules, hooks, scope maps, protected settings |
| Semantic / retrieval | relevant memory search | embeddings, compressed vectors, semantic hashes |

### Why both are needed

Without the symbolic layer:

- policy becomes ambiguous
- hooks become hard to govern

Without the semantic layer:

- memory retrieval becomes weak
- local history becomes hard to search intelligently

### Intended update flow

```text
human updates rule
      │
      ▼
relational row changes
      │
      ▼
semantic index refreshes if needed
      │
      ▼
future retrieval sees both explicit policy and relevant memory
```

---

## 14. Trust and Authority Model

> [!WARNING]
> This is the most critical architectural boundary in the entire system. If AgentFS fails here, it becomes a dangerous automation substrate instead of a safe repository operating layer.

### Dual-zone authorization model

| Zone | Authority | Typical Contents | Hard Rule |
| --- | --- | --- | --- |
| User Domain | human-controlled | rules, hooks, policy, signatures, enterprise defaults | agents must not silently rewrite it |
| Agent Domain | agent read/write | memory, logs, transient observations, embeddings | writes must never escalate into policy ownership |

### Why it exists

If policy and memory share the same mutability, the system can drift into:

- unauthorized hook modification
- silent execution escalation
- memory rewriting repository standards
- unreviewable behavior changes

### Signature and cryptographic protection

Protected writes should be enforceable by:

- SSH keys
- GPG keys
- future enterprise signing pipelines

---

## 15. Security Guardrails

| Guardrail | Architectural Purpose |
| --- | --- |
| Signature-gated protected updates | preserve human control |
| Audit-friendly Git integration | keep state reviewable |
| Reflection surfaces | avoid black-box failure modes |
| Controlled hook ownership | prevent agentic execution-path drift |
| Bounded mutable runtime memory | reduce governance pollution |

### Critical security concern

If an agent can freely rewrite:

- `pre-commit`
- `pre-push`
- build hooks
- shell execution records

then AgentFS stops being governable. Therefore hook ownership is not an implementation detail. It is a first-order architectural boundary.

---

## 16. Git as a First-Class Product Layer

Git is part of the runtime model, not just source control.

### What `.gitattributes` can capture

| Mechanism | Git Commands Affected | Intended AgentFS Use |
| --- | --- | --- |
| merge driver | `merge`, `pull`, `rebase` | reconcile `.agent.db` semantically |
| clean filter | `add` | normalize or compress before staging |
| smudge filter | `checkout`, `switch`, `clone` | refresh local projections or compatibility views |
| diff driver | `diff`, `show` | render human-readable changes for binary-backed state |

### What `.gitattributes` cannot capture

| Missing Surface | Why It Matters |
| --- | --- |
| `git commit` | needs validation and rule enforcement |
| `git push` | may need security and leak checks |
| `git status` | not a file transform path |
| general workflow intent | Git attributes do not know operator purpose |

### Therefore the complete Git strategy is:

- `.gitattributes`
- merge driver
- diff export/textconv
- clean filter
- smudge filter
- local hooks

### Git lifecycle architecture diagram

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

## 17. Merge Driver Strategy

Because `.agent.db` is binary-backed, ordinary Git text conflict behavior is insufficient.

### Required merge-driver behavior

| Requirement | Reason |
| --- | --- |
| understand ancestor/current/other inputs | standard 3-way merge mechanics still apply |
| compare by row identity and timestamps | semantic state must reconcile structurally |
| preserve authority boundaries | protected records cannot be casually overwritten |
| emit deterministic result | team workflows depend on predictability |

### Team sync diagram

```text
 Developer A's Machine                      Developer B's Machine
┌───────────────────────────┐              ┌───────────────────────────┐
│  Modified .agent.db       │              │  Modified .agent.db       │
└─────────────┬─────────────┘              └─────────────┬─────────────┘
              │                                          │
              ▼                                          ▼
    git push / merge path                      git pull / merge path
              │                                          │
              └──────────────────► 💾 ◄──────────────────┘
                                   │
                                   ▼
                        [ afs-merge-driver ]
                        • compares rows
                        • resolves timestamps
                        • preserves policy ownership
                        • writes fresh binary
```

---

## 18. Reflection Layer

A binary semantic store must still be reviewable by humans.

### Reflection forms

| Form | Purpose |
| --- | --- |
| markdown export | quick audit |
| JSON export | machine-readable inspection |
| diff export | readable change review in Git |
| virtual projection | human-readable mounted view without polluting the repo |

### Virtual reflection example

```text
.afs/virtual/
├── rules/
├── hooks/
├── memory/
└── logs/
```

### Why this is mandatory

Without reflection:

- contributors cannot inspect state
- security teams cannot audit behavior
- broken memory becomes invisible
- Git reviews become too opaque

---

## 19. Human Transparency vs Machine Optimization

One of the main tradeoffs in the original design brief is:

| Human Need | Machine Need |
| --- | --- |
| readable and editable policy | compact and efficient storage |
| obvious ownership | structured mutable memory |
| reviewable changes | low-latency access |
| simple diagnostics | binary-level optimization |

AgentFS resolves this with a split strategy:

- canonical truth lives in `.agent.db`
- human inspection lives in exports, projections, and CLI tooling

---

## 20. Runtime Access Syntax and Command Surface

### Human CLI syntax

```bash
afs init
afs add rule "Always use strict TypeScript. No 'any' types." --scope /Folder-1
afs add hook pre-commit "pnpm test" --scope global
afs status
afs doctor
afs export --format=markdown > audit_rules.md
afs ingest --clean
```

### Internal conceptual syntax

```text
ai://rules/syntax
ai://state/memory
ai://skills/git/commit
ai://hooks/pre-commit
```

### Python prototype syntax from the original brief

```python
DB_FILE = ".agent.db"

cursor.execute("""
    CREATE TABLE IF NOT EXISTS agent_data (
        id TEXT PRIMARY KEY,
        category TEXT,
        scope TEXT,
        content TEXT,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )
""")

cursor.execute("""
    SELECT content FROM agent_data
    WHERE (scope = ? OR scope = 'global') AND category = ?
""", (scope_name, category_filter))
```

### Why syntax matters here

The syntax defines:

- how humans create and inspect state
- how tools and adapters think about semantic paths
- how records are categorized and queried

---

## 21. Tools and Technologies

### Core runtime candidates

| Tool / Technology | Role |
| --- | --- |
| Rust | primary systems language for native core |
| SQLite | embedded relational store |
| DuckDB | optional analytical or future semantic extension layer |
| memmap2 | memory-mapped file primitives |
| clap | CLI argument parsing |
| ed25519-dalek | signature and trust enforcement support |
| ratatui | lightweight terminal status UI |
| crossterm | terminal interaction layer |

### Repo and workflow tooling

| Tool | Role |
| --- | --- |
| Turbo | monorepo task graph and caching |
| Lefthook | local fast pre-commit enforcement |
| cargo-deny | dependency, license, and bloat discipline |
| Biome | JS/TS formatting and linting |
| GitHub Actions | CI, release, and security automation |

### Platform-level mechanisms

| Mechanism | Usage |
| --- | --- |
| FUSE | Linux/macOS reflection projection |
| Projected File System / Dokany | Windows projection path |
| inotify | Linux file events |
| FSEvents | macOS file events |
| ReadDirectoryChangesW | Windows file event wakeups |

---

## 22. Production-Grade Monorepo Architecture

> [!IMPORTANT]
> This is the complete target repository architecture as given in the original technical context. It is intentionally broad because the repo is meant to support a world-class contributor and release ecosystem while still protecting a small trusted core.

### Full architecture file tree

```text
agentfs/ (Repository Root)
├── .changeset/                             # Changeset versioning records
│   ├── config.json                         # Multi-package semantic versioning settings
│   └── README.md                           # Contributor changelog documentation
├── .devcontainer/                          # Onboarding container ecosystem
│   └── devcontainer.json                   # Docker environment spec (Rust, SQLite, Node)
├── .github/                                # Remote CI/CD and automation house
│   ├── workflows/
│   │   ├── ci.yml                          # Test verification pipeline on every PR
│   │   ├── release.yml                     # Native cross-compilation pipeline (Multi-OS binaries)
│   │   └── security.yml                    # Automated vulnerability scanning
│   ├── DEPENDENCIES.md                     # Compliance data for upstream crates
│   └── FUNDING.yml                         # Open-source sponsorship routes
├── .vscode/                                # Human workspace interface rules
│   ├── extensions.json                     # Recommended plugins (Rust-analyzer, Biome)
│   └── settings.json                       # Editor-level formatting-on-save defaults
├── apps/                                   # High-level interfaces & tools
│   ├── cli/                                # Node/TypeScript CLI wrapper framework
│   │   ├── src/index.ts                    # Main entry point for `npm i -g agentfs`
│   │   ├── package.json
│   │   └── tsconfig.json
│   └── docs/                               # Developer portal (Mintlify/Starlight documentation)
│       ├── docs.json                       # Navigation structures and portal themes
│       └── introduction.mdx                # Interactive reference guide setup
├── crates/                                 # Performance-critical low-level layers (Rust)
│   ├── afs-core/                           # Core runtime compiler engine
│   │   ├── src/
│   │   │   ├── db/mod.rs                   # SQLite/DuckDB embedded layout handlers
│   │   │   ├── vfs/mod.rs                  # FUSE / Windows projected file mappings
│   │   │   ├── crypto.rs                   # GPG / SSH enterprise signature verification
│   │   │   └── main.rs                     # System binary execution loop
│   │   └── Cargo.toml                      # High-optimization compiler instructions
│   └── afs-git/                            # Specialized Git integration tools
│       ├── src/
│       │   ├── merge_driver.rs             # Conflict resolver for the .agent.db binary
│       │   └── hooks.rs                    # Auto-injectable pre-commit & smudge scripts
│       └── Cargo.toml
├── .editorconfig                           # Universal workspace indentation matrix
├── .gitignore                              # Protects local database builds and test environments
├── biome.json                              # 25x faster JS/TS workspace formatting matrix
├── Cargo.toml                              # Multi-crate workspace optimization orchestrator
├── deny.toml                               # Cargo-Deny (Anti-bloat, license & vulnerability gates)
├── lefthook.yml                            # High-speed parallel pre-commit validation manager
├── package.json                            # Root scripts and workspace package manager rules
├── pnpm-workspace.yaml                     # Polyglot monorepo connection manager
├── renovate.json                           # Automated dependency upkeep manager
└── turbo.json                              # Turbo smart build-caching matrix
```

### Monorepo layer ownership table

| Layer | Main Responsibility | Why It Exists |
| --- | --- | --- |
| `crates/afs-core` | trusted runtime core | performance, correctness, authority boundaries |
| `crates/afs-git` | Git lifecycle semantics | merge/filter/hook specialization |
| `apps/cli` | operator experience | global install and user-facing flows |
| `apps/docs` | public docs portal | contributor and user-facing reference |
| repo root tooling | automation and guardrails | reproducibility, quality, release discipline |

---

## 23. DevTools Configuration and Execution Guardrails

### `turbo.json`

The goal is to avoid recompiling native crates when documentation or TS-only changes do not require it.

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

The intent is to make local quality checks very fast and hard to bypass accidentally.

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

The system must protect the small trusted binary from dependency bloat and unsafe licensing.

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

### `release.yml`

The release pipeline must build native artifacts for multiple operating systems.

```yaml
name: Release Binaries
on:
  push:
    tags:
      - 'v*'
jobs:
  compile:
    name: Build Target
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            artifact_name: afs-linux-x64
          - os: macos-latest
            target: aarch64-apple-darwin
            artifact_name: afs-macos-arm64
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            artifact_name: afs-windows-x64.exe
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
      - name: Run Optimized Production Compile
        run: cargo build --release --target ${{ matrix.target }}
      - name: Upload Native Deliverable
        uses: actions/upload-artifact@v4
        with:
          name: ${{ matrix.artifact_name }}
          path: target/${{ matrix.target }}/release/afs*
```

---

## 24. Core Runtime Configuration Example

To preserve a small trusted binary, the brief strongly points toward Rust with aggressive size-focused optimization.

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

### Why these variables matter

| Variable | Reason |
| --- | --- |
| `opt-level = "z"` | optimize for binary size |
| `lto = true` | allow whole-program optimization |
| `codegen-units = 1` | tighter packed release builds |
| `panic = "abort"` | remove unwinding overhead |
| `strip = true` | remove debug symbols from release artifacts |

---

## 25. Operational Commands

### Human-facing commands

```bash
afs init
afs status
afs doctor
afs export --format=markdown
afs ingest --clean
```

### Purpose table

| Command | Purpose |
| --- | --- |
| `afs init` | bootstrap `.agent.db`, Git wiring, and reflection |
| `afs status` | inspect runtime state and active scope |
| `afs doctor` | verify integrity and Git integration |
| `afs export` | flatten semantic state into readable output |
| `afs ingest` | migrate existing tool-specific files into AgentFS |

---

## 26. Interactive Terminal UI Concept

The original design brief also assumes a lightweight terminal dashboard rather than a heavyweight GUI.

```text
┌─ AgentFS v0.1.0 ──────────────────────────── Systems Status: ACTIVE ──┐
│ Active File: /projects/my-monorepo/.agent.db [Size: 4.2 MB]           │
│ Memory Footprint: 12.4 MB / 50.0 MB Max (LRU Caching Enabled)         │
└────────────────────────────────────────────────────────────────────────┘
┌─ Workspace Topology ───────────────────────┐┌─ Live Agent Context ─────┐
│ ├── [Global Shared Context]               ││ Scope: /Folder-1          │
│ ├── Folder-1 (TypeScript Core)            ││ Rule-01 Strict Types Only │
│ │   └── 4 Rules | 2 Hooks Active          ││ Rule-02 Linting Path      │
│ └── Folder-2 (Python Backend)             ││ Hook-01 pre-commit        │
│     └── 2 Rules | 0 Hooks Active          ││ Memory Slice Loaded       │
└───────────────────────────────────────────┘└───────────────────────────┘
┌─ Security & Verification ─────────────────┐┌─ Recent Sync Ledger ──────┐
│ Total Signed Entries: 6/6                 ││ [05:42:11] merge: OK      │
│ Cryptographic Guardrails: STRICT          ││ [05:40:02] memory log     │
│ Authority: GPG / SSH verified             ││ [05:12:44] init checkpoint│
└───────────────────────────────────────────┘└───────────────────────────┘
  [Q] Quit  [A] Add Rule  [H] View Hooks  [M] Flatten to Virtual VFS
```

---

## 27. Platform Strategy

The system is intended to be cross-platform and universally usable by AI tooling.

### Supported platform model

```text
                  ┌───────────────────────────┐
                  │    AgentFS Rust Source    │
                  └─────────────┬─────────────┘
                                │
       ┌────────────────────────┼────────────────────────┐
       ▼                        ▼                        ▼
  macOS                    Windows                   Linux
  FSEvents + mmap          Win32 mapping            inotify + mmap
  Apple Silicon + Intel    x86_64 / ARM64           desktop / server / CI
```

### Platform table

| Platform | Native Mechanisms |
| --- | --- |
| macOS | FSEvents, POSIX mmap |
| Windows | CreateFileMapping, Projected File System or related equivalents |
| Linux | inotify, mmap |

### Installation surface

```bash
curl -fsSL https://agentfs.systems | sh
```

Potential package-manager paths:

- `brew install agentfs`
- `winget install agentfs`
- `apt install agentfs`
- `apk add agentfs`

---

## 28. Deployment Modes

The design supports three deployment or distribution modes.

| Mode | Description | Best For |
| --- | --- | --- |
| 100 percent local offline | single local `.agent.db` with no remote requirement | privacy, solo dev, air-gapped work |
| static cloud distribution | object storage hosts snapshots or deltas | lightweight team sharing |
| peer-distributed | file blocks sync across machines | specialized decentralized environments |

### Architecture comparison

| Feature | Static Cloud | 100% Local Offline | P2P Team Network |
| --- | --- | --- | --- |
| Requires running server? | No | No | No |
| Network needed? | Yes | No | Yes |
| Setup complexity | Low | Very low | Medium |
| Best fit | distributed remote teams | solo/private work | constrained decentralized teams |

---

## 29. Zero Idle CPU Design

The system is explicitly intended not to burn CPU while idle.

### Why idle CPU can stay near zero

| Mechanism | Why It Helps |
| --- | --- |
| CLI exits quickly | no resident process after one-shot commands |
| passive event loops | background waiting does not consume active CPU |
| OS interrupts | filesystem wakes happen only when needed |
| mapped reads | reading bypasses repetitive file-descriptor churn |

### Engineering rules

1. do not use polling loops
2. prefer native filesystem watchers
3. keep the read path passive and local
4. keep semantic caches bounded

---

## 30. Concurrency and Multi-Agent Coordination

Multiple agents create real consistency hazards.

### Concurrency risks

| Risk | Description |
| --- | --- |
| DB write contention | several agents write at once |
| lock conflicts | one agent holds a row or scope while another needs it |
| stale reads | a write is not yet durably visible |
| policy contamination | memory writes drift toward protected policy |

### Intended controls

| Control | Purpose |
| --- | --- |
| scope locks | isolate parallel work per folder or subsystem |
| row mutexes | protect fine-grained updates |
| append-only writer | serialize writes safely |
| actor-model appender | move direct writes out of agent loops |

### Event-driven write diagram

```text
Agent A ----\
             \
Agent B ------> [ Named Pipe / Ring Buffer ] --> [ Single Appender Thread ] --> .agent.db
             /
Agent C ----/
```

### Tradeoff

Writes become eventually consistent rather than instantly globally visible, but the system becomes far safer under concurrency.

---

## 31. Semantic Retrieval and Memory Compression

Semantic retrieval is useful only if it remains lightweight.

### Techniques implied by the brief

| Technique | Purpose |
| --- | --- |
| Binary Quantization (BQ) | compress embeddings aggressively |
| Scalar Quantization (SQ) | lower memory overhead with moderate fidelity |
| truncated embeddings | shrink dimensional footprint |
| semantic hashing | tiny approximate retrieval keys |
| Matryoshka embeddings | compact multi-resolution vector representation |

### Core requirement

The retrieval layer must not explode `.agent.db` from a small local system file into an enormous opaque artifact.

### File-size risk table

| Problem | Mitigation |
| --- | --- |
| embeddings balloon file size | truncation, quantization, compaction |
| whole-index loading breaks RAM budget | LRU or sliding-window caching |
| retrieval gets slower over time | bounded semantic index maintenance |

---

## 32. Time Travel, TTL, and Lifecycle Extensions

The original brief points toward richer state management than plain rules.

### Future extension ideas

| Capability | Value |
| --- | --- |
| time-travel snapshots | recover from bad agent outcomes |
| state deltas | record stepwise evolution |
| TTL pruning | automatically retire temporary context |
| local state merging | preserve recent useful memory without permanent clutter |

### Why TTL matters

Without TTL:

- temporary debugging lessons become permanent noise
- context volume grows without discipline
- token-efficiency advantages decay

---

## 33. IDE Compatibility Wall

This is one of the largest technical blockers.

### Problem

Many existing tools are hardcoded to look for text files like:

- `.cursorrules`
- `.github/copilot-instructions.md`
- `AGENTS.md`

### Therefore AgentFS cannot rely only on the existence of `.agent.db`

It needs one or more of:

- virtual file provider behavior
- generated compatibility files
- post-checkout local projection
- SDK or extension adapters

### Two compatibility mechanisms

| Mechanism | Description | Tradeoff |
| --- | --- | --- |
| virtual reflection layer | files appear to exist without polluting the repo | more platform complexity |
| generated local compatibility files | local temporary files are compiled from `.agent.db` | extra local hook and cleanup logic |

---

## 34. Migration Pipeline

The original brief describes a "consume, convert, and clean" approach.

### `afs ingest --clean`

```bash
afs ingest --clean
```

### Ingestion lifecycle diagram

```text
┌─────────────────────────────────────────────────────────────────────────┐
│                           INGESTION LIFECYCLE                           │
├───────────────────┬─────────────────────────────────────────────────────┤
│ 1. Scan           │ Locate AI files such as .cursorrules and AGENTS.md │
├───────────────────┼─────────────────────────────────────────────────────┤
│ 2. Parse & Scope  │ Map rules into exact repository paths              │
├───────────────────┼─────────────────────────────────────────────────────┤
│ 3. Store / Index  │ Write canonical rows into .agent.db                │
├───────────────────┼─────────────────────────────────────────────────────┤
│ 4. Erase & Stream │ Remove duplicates and activate reflection support   │
└───────────────────┴─────────────────────────────────────────────────────┘
```

### Benefits

| Benefit | Meaning |
| --- | --- |
| zero lost work | legacy prompts remain preserved |
| no duplicate authority | repository converges on one source of truth |
| clean workspaces | human-facing tree stays uncluttered |

---

## 35. Enterprise and Governance Model

Larger organizations require stronger governance than solo local use.

### Enterprise model

| Requirement | Architectural Response |
| --- | --- |
| upstream security policy | read-only remote policy ingestion |
| cryptographic policy integrity | signed deltas and local signature verification |
| non-escalating agent behavior | protected tables locked to human authority |
| auditability | logs and export surfaces remain available |

### Enterprise flow

```text
Corporate Policy Source
          │
          ▼
signed delta
          │
          ▼
local AgentFS runtime
          │
          ▼
validated merge into protected policy domain
```

---

## 36. Operational Verification Pipeline

### End-to-end project flow

```text
[ Contributor Code Update ] ──► [ Lefthook Blocks/Passes Local Formatting ]
                                          │
                                          ▼
[ Pull Request Opened ]     ──► [ GitHub Actions Validates Cross-OS Compilation ]
                                          │
                                          ▼
[ Changeset Merged ]        ──► [ Automated Release Generator Ships Multi-OS Binaries ]
```

### Lifecycle loop

```text
┌────────────────────────────────────────────────────────────────────────┐
│                              LIFECYCLE LOOP                            │
├───────────────────┬────────────────────────────────────────────────────┤
│ 1. Code Commit    │ Engineer pushes code; runtime may log local state  │
├───────────────────┼────────────────────────────────────────────────────┤
│ 2. Team Sync      │ Merge drivers reconcile semantic project state      │
├───────────────────┼────────────────────────────────────────────────────┤
│ 3. Zero-Footprint │ Repo remains clean for humans while state persists  │
└───────────────────┴────────────────────────────────────────────────────┘
```

---

## 37. Tradeoff Matrix

| Area | Innovative Blueprint | Hard Tradeoff |
| --- | --- | --- |
| Data storage | single binary hybrid SQL/semantic file | plain Git text diffs are insufficient |
| IDE integration | adapters, reflection, or providers | compatibility work is non-trivial |
| Memory management | sliding-window cache with compression | some context switches incur small latency |
| Concurrency | single appender or scoped locks | eventual consistency may appear briefly |
| Security | human-signed protected policy | agents cannot freely rewrite execution pathways |
| Human UX | clean repo root | binary truth requires export/projection tooling |

---

## 38. Full Challenge Register

| Challenge | Why It Matters | Required Answer |
| --- | --- | --- |
| IDE extensibility wall | tools expect text | reflection, adapters, or generated compatibility files |
| Git diff conflicts | binary state diverges on branches | merge driver plus diff export |
| corruption on hard crash | store becomes critical state surface | WAL, checkpoints, validation |
| human readability | binary-only systems lose trust | CLI, export, and VFS reflection |
| vector file bloat | semantic memory can get huge | quantization, truncation, compaction |
| multi-agent races | agents write simultaneously | locks, append queues, actor-model serializer |
| unsafe hook drift | agents may try to mutate execution paths | human-only protected domains and signatures |

---

## 39. What AgentFS Is Not

| Not This | Why |
| --- | --- |
| not just another prompt file | does not solve lifecycle, Git, or authority |
| not just SQLite in the root | storage alone does not define repository semantics |
| not a cloud-only service | local-first operation is part of the system identity |
| not a generic agent scratchpad | governance is central, not optional |
| not a Git replacement | Git remains foundational and is extended, not displaced |

---

## 40. TA Conclusion

AgentFS should be built as a repository-native semantic operating layer with:

- one canonical hidden project file
- path-aware structured retrieval
- dual-domain authority boundaries
- Git-native lifecycle integration
- human-readable reflection and export
- bounded local-first runtime behavior
- compatibility paths for existing AI tooling

If those properties hold, AgentFS can become a serious open-source systems project rather than just another configuration convention.

## Project Identity

| Field | Value |
| --- | --- |
| Project | `AgentFS` |
| CLI | `afs` |
| Architect Context File | `docs/TA.md` |
| Author | `@Justinedevs` |
| Email | `Justinedevs@jstn.site` |
| Domain | `agentfs.systems` |

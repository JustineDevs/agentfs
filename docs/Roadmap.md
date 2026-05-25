# AgentFS Technical Roadmap

> [!IMPORTANT]
> This roadmap is both a sequencing artifact and a systems contract. It exists to prevent the project from shipping isolated features that do not fit the repository model, trust model, or Git lifecycle model described elsewhere.

> [!NOTE]
> The repository is still documentation-first. Phases and milestones below describe intended delivery order and system maturity, not completed implementation status in the current checkout.

This roadmap describes AgentFS as a product and implementation program, not merely a backlog. It captures the concepts, syntax, workflows, risks, milestones, and engineering decisions required to turn the single-file repository model into a shippable system.

## Mission

Build a local-first runtime that replaces scattered AI configuration files with one project-native system file and a consistent toolchain for rules, memory, lifecycle hooks, and multi-agent coordination.

| Roadmap Dimension | What It Must Control |
| --- | --- |
| Product scope | What AgentFS is actually trying to become |
| Phase ordering | What must exist before later capabilities make sense |
| Honesty | What is planned versus what is shipped |
| Risk | Where the architecture can fail if sequencing is wrong |

## Product Statement

AgentFS is the systems layer for AI-native repositories:

- one hidden project file
- one initialization command
- one context retrieval model
- one security boundary between human policy and agent state

The project is intentionally designed to work offline first, degrade gracefully, and integrate with existing developer workflows instead of requiring developers to abandon Git, standard editors, or local tooling.

> [!TIP]
> The roadmap should always be read together with `docs/Concept.md`. Concept defines the system identity; roadmap defines the order in which that identity becomes operational.

## Terminology

| Term | Meaning | Why It Matters |
| --- | --- | --- |
| Project file | Canonical hidden semantic store | Defines the main persistence surface |
| Scope | Path or namespace where a record applies | Enables precise context retrieval |
| Category | Record type such as rule or memory | Prevents policy and runtime state from blurring |
| User domain | Protected human-owned record area | Preserves authority boundaries |
| Agent domain | Mutable agent runtime area | Enables safe memory and logging |

### Project file

The canonical hidden store in the repository root, expected to default to:

```text
.agent.db
```

Alternative future extensions may include `.aifs` or `.agentfs`, but `.agent.db` remains the clearest initial contract because it communicates a database-backed implementation.

### Scope

A directory path or reserved namespace that determines where a record applies.

Examples:

```text
global
/apps/web
/crates/afs-core
/docs
```

### Category

The high-level record type stored in the project file:

- `rule`
- `skill`
- `hook`
- `memory`
- `log`
- `snapshot`

### User domain

Protected records controlled by humans and optionally signature-gated.

### Agent domain

Mutable runtime data written by agents, such as memory, observations, or logs.

## Core System Model

AgentFS relies on five cooperating layers.

| Layer | Responsibility | Failure If Missing |
| --- | --- | --- |
| Storage engine | Durable semantic store | Context has no stable source of truth |
| Context runtime | Scoped retrieval and assembly | Tools read too much or the wrong data |
| Git integration | Merge, diff, filter, and hook behavior | Shared state becomes operationally brittle |
| Reflection layer | Human-readable surfaces | Binary-backed state becomes unreviewable |
| Security boundary | Protected versus mutable ownership | Agents can drift into unsafe authority |

### 1. Storage engine

The storage engine keeps all project-local AI state inside a single file using an embedded database model with WAL enabled for reliability.

Candidate technologies:

- SQLite
- DuckDB for analytical overlays
- a hybrid design where SQLite remains the durable control plane and specialized structures support vector or hash search

### 2. Context runtime

The runtime resolves:

- current path
- inherited scopes
- active rules
- matching hooks
- relevant memory

It returns only the records needed for the current task instead of loading full text blobs.

### 3. Git integration

Git lifecycle support covers:

- clean filters
- smudge filters
- merge driver reconciliation
- readable diff exports
- hook installation

### 4. Reflection layer

Because a binary store cannot be casually edited by hand, AgentFS needs a human-readable view. That can be delivered by:

- export commands
- a generated temporary mirror
- an optional virtual filesystem mount

### 5. Security boundary

The runtime must prevent agent self-escalation by restricting writes to protected policy and hook records.

## Problem Landscape

AgentFS exists because AI repository state is fragmented across incompatible formats and vendor-specific conventions.

> [!WARNING]
> If the project tries to solve only "context retrieval speed" without solving authority, migration, and Git behavior, it will become another partial tool rather than a repository substrate.

### Current failure modes

1. Root directory pollution
   AI config files accumulate at the top of the repo until the developer loses confidence in which file actually controls behavior.
2. Token waste
   Long instruction files are repeatedly dumped into prompts even when the agent only needs one rule.
3. Merge conflicts
   Human-edited markdown instructions and tool-generated memory files collide frequently in shared branches.
4. Trust ambiguity
   When hooks, memory, and policy all live as editable text, agents can too easily rewrite operational behavior.
5. Multi-agent drift
   Independent agents have no authoritative shared state boundary, so observations diverge and coordination becomes ad hoc.

## Command Surface

The command line is the stable human entrypoint. These commands define the core product grammar.

| Command Family | Role |
| --- | --- |
| `init` | bootstrap repository-local AgentFS state |
| `add` / `list` / `remove` | manage structured records |
| `export` / `compact` / `vacuum` | inspect and maintain local store health |
| `ingest` | migrate legacy AI context into the canonical store |
| `status` / `doctor` / `verify` | operational diagnostics and safety checks |

### Initialization

```bash
afs init
```

Expected behavior:

- create `.agent.db`
- register local Git wiring
- install hook/bootstrap scripts
- create optional reflection metadata directories

### Rule management

```bash
afs add rule "Always use strict TypeScript" --scope /apps/web
afs list rules --scope /apps/web
afs remove rule <id>
```

### Hook management

```bash
afs add hook pre-commit "pnpm test" --scope global
afs list hooks
afs verify hooks
```

### Memory and export

```bash
afs export --format markdown
afs export --format json
afs vacuum
afs compact
```

### Migration

```bash
afs ingest --clean
afs ingest --from .cursorrules
afs ingest --from AGENTS.md
```

### Diagnostics

```bash
afs status
afs doctor
afs verify
```

## Data Schema Direction

> [!NOTE]
> Schema direction here is conceptual and directional. Exact implementation may evolve, but category, scope, and authority separation are non-negotiable parts of the model.

The file format should support both deterministic retrieval and future semantic retrieval.

Representative baseline schema:

```sql
CREATE TABLE agent_data (
    id TEXT PRIMARY KEY,
    category TEXT NOT NULL,
    scope TEXT NOT NULL,
    content TEXT NOT NULL,
    signature TEXT,
    embedding BLOB,
    ttl_seconds INTEGER,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_agent_data_scope_category
ON agent_data(scope, category);
```

Follow-on tables likely include:

- `locks`
- `snapshots`
- `logs`
- `sources`
- `policies`

| Table | Purpose |
| --- | --- |
| `agent_data` | baseline shared record surface |
| `locks` | multi-agent coordination and write safety |
| `snapshots` | rollback and time-travel state |
| `logs` | bounded runtime event history |
| `policies` | protected or higher-order governance records |

## Architecture Options

AgentFS should support three operating modes without changing the conceptual data model.

| Mode | Best Fit | Tradeoff |
| --- | --- | --- |
| 100 percent local offline | privacy-sensitive or solo work | no built-in shared remote distribution |
| object storage backed | lightweight team distribution | remote sync complexity rises |
| peer-distributed | specialized decentralized team workflows | highest operational complexity |

### A. 100 percent local offline mode

Primary mode for privacy, speed, and predictable behavior.

Properties:

- no network required
- memory-mapped local reads
- easiest trust model
- strongest fit for solo developers and secure teams

### B. Object-storage backed distribution

Useful for organizations that want shared distribution without a long-running control plane.

Properties:

- store file on S3, R2, or release artifacts
- stream byte ranges or full snapshots
- suitable for policy distribution and backup

### C. Peer-distributed team mode

Longer-term option for local-network or decentralized sync workflows.

Properties:

- block-based synchronization
- stronger complexity cost
- only worthwhile if centralized coordination is explicitly undesirable

## Engineering Constraints

| Constraint | Why It Exists | Practical Consequence |
| --- | --- | --- |
| IDE compatibility | Existing tools still expect text files | reflection or compatibility generation is required |
| Git binary handling | raw binary merge behavior is insufficient | custom merge/diff/filter support is necessary |
| corruption risk | single-file state becomes critical | WAL and recovery rules must exist |
| human operability | contributors must trust the system | export and inspection tools are mandatory |

### Constraint 1: IDE compatibility

Many tools expect plain text files today. AgentFS must adapt to that reality through one or both of:

- virtual reflection
- generated local compatibility files that are scrubbed before commit

### Constraint 2: Git cannot merge raw binary state safely by default

This forces a custom merge driver and export-driven diff support.

### Constraint 3: Binary corruption risk must be handled

The design must use:

- WAL
- deterministic checkpoint policy
- crash recovery behavior
- validation commands

### Constraint 4: Human operability matters

If the only interface is a black box database, contributors will not trust the system. Visibility and export tools are mandatory.

## Security Model Roadmap

> [!IMPORTANT]
> Security phases are not optional polish. They determine whether AgentFS is safe to adopt beyond toy usage.

### Phase A: logical domain separation

Start by separating records into protected versus mutable categories in the schema and write APIs.

### Phase B: local signature enforcement

Require valid local signatures for protected record changes such as hooks or organization policies.

### Phase C: enterprise signed delta ingestion

Allow organizations to publish signed upstream policy deltas that can be merged into local AgentFS files without exposing write access to agents.

## Performance Model

AgentFS makes four explicit performance promises.

| Performance Promise | System Meaning |
| --- | --- |
| Zero idle CPU | no busy polling or wasteful resident loops |
| Fast scoped reads | directory-local context can be fetched cheaply |
| Bounded memory | retrieval layers must stay inside predictable budgets |
| Small trusted binary | installation and audit remain practical |

### 1. Zero idle CPU

No busy polling loops. Filesystem watchers and event-driven wakeups only.

### 2. Fast scoped reads

Path-scoped record lookup should be cheap enough to serve as a normal part of agent navigation, not a heavyweight indexing operation.

### 3. Bounded memory

Vector or hash search must stay within a predictable working set using a sliding-window or LRU strategy.

### 4. Small trusted binary

The core runtime should preserve a small distribution footprint so global installation remains practical.

## Git Capture Model

`.gitattributes` alone is insufficient. AgentFS needs both attribute-based file handling and hook-based lifecycle interception.

| Developer action | Mechanism | AgentFS purpose |
| --- | --- | --- |
| `git add` | clean filter | normalize binary state before staging |
| `git checkout` / `git switch` | smudge filter | refresh local projections |
| `git merge` / `git pull` / `git rebase` | merge driver | semantic reconciliation |
| `git diff` / `git show` | diff export | readable reviews |
| `git commit` | pre-commit hook | policy and quality checks |
| `git push` | pre-push hook | leak prevention and security review |

## Product Phases

| Phase | Main Outcome | Why It Comes Here |
| --- | --- | --- |
| Phase 0 | repository foundation | shared language must exist before code scales |
| Phase 1 | local runtime baseline | core persistence and diagnostics come first |
| Phase 2 | Git lifecycle integration | shared state is unsafe without reconciliation |
| Phase 3 | migration and compatibility | adoption depends on meeting teams where they are |
| Phase 4 | multi-agent runtime | concurrency only matters after the single-agent core is stable |
| Phase 5 | semantic retrieval | optimization follows correctness |
| Phase 6 | enterprise controls | advanced governance builds on earlier trust boundaries |

## Phase 0: repository foundation

Goal:

- documentation
- architecture agreement
- naming and subsystem boundaries
- repository automation skeleton

Exit criteria:

- README, concept, architecture, and roadmap are coherent
- remote repository is connected
- contributor-facing vocabulary is stable

## Phase 1: local runtime and CLI baseline

Goal:

- `afs init`
- `afs status`
- `afs doctor`
- scoped rule CRUD
- `.agent.db` creation and verification

Exit criteria:

- local repository can initialize AgentFS
- human can inspect active rules through CLI output
- corruption checks and sanity checks exist

## Phase 2: Git lifecycle integration

Goal:

- clean filter
- smudge filter
- merge driver
- text export for diffs
- local hook installation

Exit criteria:

- Git operations no longer treat the project file as opaque and unmanageable
- merge conflicts can be reconciled semantically

## Phase 3: migration and compatibility

Goal:

- ingest legacy files
- optionally remove legacy files
- support virtual or generated compatibility views

Exit criteria:

- an existing AI-enabled repo can move into AgentFS without losing policy

## Phase 4: multi-agent runtime

Goal:

- row or scope locks
- append-only event writer
- agent-state synchronization

Exit criteria:

- multiple agents can cooperate without corrupting shared state or rewriting each other unintentionally

## Phase 5: semantic retrieval and optimization

Goal:

- embedding or semantic hash storage
- quantization
- memory budgets
- TTL pruning

Exit criteria:

- semantic memory remains local, small, and operationally cheap

## Phase 6: enterprise controls

Goal:

- signed policies
- optional remote registry
- auditable policy sync

Exit criteria:

- organizations can trust the system in regulated or security-sensitive environments

## Release Targets

> [!TIP]
> Version labels in this roadmap are sequencing anchors. They should not be marked complete until the relevant behavior exists and can be verified, not merely described.

### v0.1.0

Focus:

- foundational docs and terminology
- repository structure
- core runtime blueprint
- CLI contract

### v0.1.5

Focus:

- multi-agent mutex locks
- append-queue write coordination

### v0.2.0

Focus:

- snapshot and rollback engine
- TTL lifecycle
- richer export and audit surfaces

### v0.3.0

Focus:

- enterprise policy registry
- signature-gated upstream deltas
- broader SDK support

## Risks and Mitigations

| Risk | Why it matters | Mitigation |
| --- | --- | --- |
| Binary mistrust | Contributors cannot read raw state | strong export and reflection tooling |
| Tool incompatibility | Editors still expect text files | compatibility generation or VFS reflection |
| Corruption during writes | Single-file state becomes a critical asset | WAL, checkpoints, validation, recovery |
| File bloat | Embeddings can balloon storage size | quantization, truncation, TTL, compaction |
| Agent self-escalation | Hooks or policy may be rewritten | domain separation and signature enforcement |

## Success Metrics

| Metric | What Success Looks Like |
| --- | --- |
| Prompt efficiency | less irrelevant context reaches downstream models |
| Repository cleanliness | fewer root-level AI clutter files remain |
| Git reliability | shared state can be reconciled predictably |
| Runtime discipline | memory and idle CPU remain bounded |
| Governance clarity | humans can tell what is protected versus mutable |

The roadmap is successful when AgentFS can demonstrate:

- materially lower prompt token overhead in scoped workflows
- fewer root-level AI configuration files
- reliable Git reconciliation of shared state
- bounded memory and idle CPU behavior
- a comprehensible trust model for humans reviewing agent infrastructure

## Contact

| Field | Value |
| --- | --- |
| Author | `@Justinedevs` |
| Email | `Justinedevs@jstn.site` |
| Domain | `agentfs.systems` |

This document should be updated only when the architecture contract or phase boundaries materially change. It is intended to remain the authoritative roadmap for both implementation and contributor alignment.

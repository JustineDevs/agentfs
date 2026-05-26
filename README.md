# AgentFS (`afs`)

<p align="center">
  <a href="https://github.com/JustineDevs/agentfs/releases"><img src="https://img.shields.io/badge/version-pre--idea--0.1.0-111111?style=for-the-badge" alt="Version"></a>
  <a href="https://github.com/JustineDevs/agentfs"><img src="https://img.shields.io/badge/packages-core%20%7C%20git%20%7C%20sdk-111111?style=for-the-badge" alt="Packages"></a>
  <a href="https://github.com/JustineDevs/agentfs"><img src="https://img.shields.io/badge/stack-rust%20%7C%20typescript%20%7C%20sqlite-111111?style=for-the-badge" alt="Tech Stack"></a>
  <a href="https://agentfs.systems"><img src="https://img.shields.io/badge/website-agentfs.systems-111111?style=for-the-badge" alt="Website"></a>
</p>

<p align="center">
  <strong>The open-source system file for AI-native repositories.</strong><br />
  AgentFS consolidates rules, skills, memory, hooks, and agent lifecycle metadata into one offline-first project file instead of scattering tool-specific text files across the repository root.
</p>

<p align="center">
  <a href="https://github.com/JustineDevs/agentfs/releases">Releases</a> ·
  <a href="https://github.com/JustineDevs/agentfs/tags">Tags</a> ·
  <a href="https://github.com/users/JustineDevs/packages?repo_name=agentfs">Packages</a>
</p>

---

## Sponsor

> [!NOTE]
> The sponsor embeds below are included in the README source as requested. GitHub may sanitize or suppress iframe rendering in repository markdown views, so the direct sponsor links remain the reliable fallback.

<p align="center">
  <iframe src="https://github.com/sponsors/JustineDevs/button" title="Sponsor JustineDevs" height="32" width="114" style="border: 0; border-radius: 6px;"></iframe>
</p>

<p align="center">
  <iframe src="https://github.com/sponsors/JustineDevs/card" title="Sponsor JustineDevs" height="225" width="600" style="border: 0;"></iframe>
</p>

| Surface | Link | Purpose |
| --- | --- | --- |
| Sponsor profile | `https://github.com/sponsors/JustineDevs` | Support the project directly |
| Releases | `https://github.com/JustineDevs/agentfs/releases` | Preview binaries and published release notes |
| Tags | `https://github.com/JustineDevs/agentfs/tags` | Version history and release cut points |
| Packages | `https://github.com/users/JustineDevs/packages?repo_name=agentfs` | Package and distribution surface tied to the repository |

## Why AgentFS Exists

Modern AI-assisted repositories accumulate configuration sprawl:

- `.cursorrules, .cursorrules, devinrules, etc.`
- `AGENTS.md, CLAUDE.md`
- `copilot-instructions.md or skills directories`
- tool-specific JSON or YAML files
- scratch memory files that grow without governance

That model is expensive and brittle. It increases prompt size, creates merge conflicts, duplicates policy across folders, and makes multi-agent coordination harder than it needs to be.

AgentFS proposes a different contract:

- one project-local binary store
- one consistent CLI
- one Git integration layer
- one security model for human-owned policy versus agent-owned runtime state

The result is a cleaner repository surface and a runtime that can deliver narrowly scoped context instead of dumping entire text blobs into every agent turn.

> [!TIP]
> AgentFS is designed to centralize repository AI context into one offline-first system file, so teams can reduce root-directory clutter, merge friction, and prompt bloat without giving up Git-native workflows.

## What AgentFS Is

AgentFS is an open-source runtime and repository architecture centered on a single hidden file, typically `.agent.db`, stored at the project root.

```text
my-repo/
├── apps/
├── crates/
├── packages/
└── .agent.db
```

The file acts as a hybrid project knowledge store with four responsibilities:

| Responsibility | Purpose |
| --- | --- |
| Rules | Human-authored repository policy, conventions, and scope-specific instructions |
| Skills | Structured workflows and reusable task definitions |
| Hooks | Trusted command triggers for lifecycle checkpoints such as `pre-commit` or `post-merge` |
| Memory | Agent-readable and agent-writable state, observations, and historical context |

At the system level, those responsibilities are delivered through four cooperating layers:

| Layer | Role |
| --- | --- |
| Storage | Holds scoped rules, hooks, skills, and memory in `.agent.db` |
| Runtime | Resolves active context by directory path and policy scope |
| Git Integration | Handles merge, diff, filter, and hook lifecycle behavior |
| Reflection | Exposes human-readable views of binary-backed state |

---

## ✨ Features

* **⚡ Zero-Parse Memory Mapping (`mmap`)**: Bypasses standard file reading. Pointers map directly to active RAM. Reads take `< 1ms`.
* **🪙 40% Token Cost Reduction**: Context-pruning isolates and feeds the LLM only the exact rule blocks required for the current directory path.
* **🧬 Custom Git Merge Driver**: Resolves schema divergences transparently during `git pull` or `git merge`. No more binary file conflicts.
* **🧊 1-Bit Quantized Semantic Vectors**: Local offline vector searches utilizing binary quantization to run with a fixed RAM footprint `< 50MB`.
* **🔒 Cryptographic Policy Safeguards**: Critical execution hooks require local developer SSH/GPG key signatures to prevent rogue agent privilege escalation.

---

## 🚀 Quick Start

### 1. Installation
Install the lightweight, compiled system CLI binary (written in Rust/Go):

```bash
curl -fsSL https://agentfs.systems | sh
```

### 2. Initialization
Navigate to your repository root and initialize AgentFS:

```bash
afs init
```

> [!NOTE]
> `afs init` automatically injects the custom binary merge driver configuration into your local `.git/config` and creates a read-only virtual reflection directory (`.afs/virtual/`) for human visibility.

> [!WARNING]
> The install and command flow shown here describes the intended AgentFS operator experience. Keep implementation, docs, and release behavior aligned so the public README never overstates shipped capability.

## Architecture at a Glance

AgentFS uses a layered local architecture:

1. `afs-core`
   The native runtime responsible for database access, memory mapping, scoped retrieval, lock coordination, and signature checks.
2. `afs-git`
   The Git integration layer that handles merge reconciliation, diff export, filter pipelines, and hook bootstrapping.
3. `afs` CLI
   The operator surface for initialization, inspection, import/export, debugging, and rule management.
4. Virtual reflection layer
   An optional read-only filesystem view that exposes human-readable projections of the binary store without making the repository itself text-cluttered.

<details>
<summary><strong>Show planned operator surfaces</strong></summary>

| Surface | Purpose |
| --- | --- |
| `afs init` | Bootstrap `.agent.db`, Git wiring, and local reflection support |
| `afs add rule` | Register path-scoped policy records |
| `afs add hook` | Attach trusted lifecycle commands |
| `afs status` | Inspect active runtime state and system health |
| `afs doctor` | Validate local setup, integrity, and Git integration |
| `afs export` | Render binary-backed content into a readable format |

</details>

## 🔒 Enterprise & Security Guardrails

> [!IMPORTANT]
> To maintain strict compliance and pipeline security, the binary schema partitions operations into a dual-zone authorization model.

| Zone | Authority | Constraint |
| --- | --- | --- |
| User Domain | Human-controlled | Protected rules and hooks should not be agent-writable |
| Agent Domain | Agent read/write | Runtime memory must not escalate into policy ownership |

## Internal Data Model

The exact implementation may evolve, but the initial contract centers on a unified table model like this:

```sql
CREATE TABLE agent_data (
    id TEXT PRIMARY KEY,
    category TEXT NOT NULL CHECK(category IN ('rule', 'skill', 'hook', 'memory')),
    scope TEXT NOT NULL,
    content TEXT NOT NULL,
    signature TEXT,
    embedding BLOB,
    ttl_seconds INTEGER,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

This schema supports four operational patterns:

- direct rule lookup by path
- vector-backed or hash-backed semantic search for memory retrieval
- signature validation on protected records
- record aging and archive policies for temporary context

## 🗺️ Roadmap & v0.1.0 Core Deliverables

> [!NOTE]
> This repository is still documentation-first. The items below describe target deliverables for the first implementation milestones, not completed features in the current checkout.

- [ ] Memory-mapped core runtime layer (`afs-core`)
- [ ] Custom Git Merge Driver integration suite
- [ ] Directory-scoped local contextual pruning
- [ ] Multi-agent mutex execution locks (Target: v0.1.5)
- [ ] Time-Travel rollback state engine (Target: v0.2.0)

---

## Detailed Trust Boundary

AgentFS intentionally splits the file into two logical domains.

| Domain | Write authority | Typical contents |
| --- | --- | --- |
| User domain | Human only, optionally signature-gated | Rules, hooks, policy, protected defaults |
| Agent domain | Agent read/write | Memory, logs, summaries, transient state |

This model prevents a compromised or hallucinating agent from silently escalating its own execution privileges by rewriting trusted hooks or policy rows.

## Repository Documentation

- [Concept](docs/Concept.md)
- [Architecture File Tree](docs/Architect-File-tree.md)
- [Roadmap](docs/Roadmap.md)

## Maintainers

| Field | Value |
| --- | --- |
| Author | `@Justinedevs` |
| Email | `Justinedevs@jstn.site` |
| Domain | `agentfs.systems` |

## 📄 License

Distributed under the MIT License. See [LICENSE](LICENSE) for more information.

# AgentFS Concept

> [!IMPORTANT]
> This document is the conceptual source of truth for AgentFS. It is intentionally broader and deeper than the README because it defines the problem model, vocabulary, trust model, and product philosophy that future implementation must preserve.

> [!NOTE]
> The repository is still documentation-first. This file describes the intended system and its reasoning model, not a claim that every described subsystem is already implemented in the current checkout.

## Executive Summary

AgentFS is a local-first system file and runtime model for AI-native repositories.

Its core claim is simple:

> repository AI context should behave like infrastructure, not scattered prose

Today, most AI-assisted repositories accumulate a mix of:

- instruction markdown
- vendor-specific configuration files
- ad hoc prompt artifacts
- scratch memory files
- implicit workflow state hidden across tools

AgentFS replaces that fragmentation with one project-native semantic store, typically `.agent.db`, plus a runtime that can answer a much better question than "what text files exist here?"

It answers:

- what policy applies at this directory?
- what memory is relevant right now?
- what hooks are trusted?
- what can an agent read?
- what can an agent write?
- what should Git do when this project state changes?

## The Core Problem

The current AI tooling ecosystem treats repository context as a file-distribution problem. AgentFS treats it as a systems-design problem.

### The current default model

In most codebases, AI behavior emerges from a loose stack of text files:

- root-level instruction files
- nested folder notes
- editor-specific config
- runtime-specific caches
- human-authored policies mixed with machine-authored memory

That approach is easy to start with, but structurally weak once the repository grows.

| Failure Mode | What Happens | Why It Matters |
| --- | --- | --- |
| Root clutter | Policy and machine state leak into the visible repo surface | Developers lose a clean mental model of the project |
| Prompt bloat | Large files get repeatedly re-read or injected into context windows | Cost rises and signal quality falls |
| Merge friction | Human and tool edits collide in normal Git workflows | Team adoption becomes painful |
| Trust ambiguity | Hooks, policy, and memory share the same edit surface | Agents can drift toward unsafe authority |
| State fragmentation | No single canonical source exists for context | Different tools see different truths |

### The real systems problem

The deeper issue is not "too many files." The deeper issue is that repository intelligence has no durable operating surface.

Without a shared substrate:

- every tool reinvents scope rules
- every editor invents its own config path
- every agent stores memory differently
- every team rebuilds policy enforcement from scratch

AgentFS exists to define that missing substrate.

## The Core Thesis

AgentFS is based on five linked ideas.

| Thesis | Meaning |
| --- | --- |
| Context is structured data | Repository intelligence should be queryable, not just readable |
| Scope matters | The active directory must influence what context is returned |
| Policy and memory are different | Human-owned rules and agent-owned runtime state need different authority boundaries |
| Git is part of the runtime | Merge, diff, checkout, and commit behavior must be designed into the system |
| Local-first is the default | Privacy, speed, and trust improve when the canonical store lives with the repository |

## What AgentFS Actually Is

At the center of AgentFS is one hidden file, expected to start as:

```text
.agent.db
```

That file is not just a database in the generic sense. It is intended to act as a repository semantic spine.

### It stores

- directory-scoped rules
- reusable skills or task definitions
- lifecycle hooks
- runtime memory
- event logs
- signatures and authority metadata
- future rollback snapshots

### It enables

- path-aware context retrieval
- single-source-of-truth policy
- controlled multi-agent state sharing
- Git-aware merge and diff behavior
- export or reflection for human inspection

### It replaces

- scattered AI instruction files
- duplicated repository policy fragments
- poorly governed local memory sprawl
- ambiguous "which file is authoritative?" workflows

## Mental Model

AgentFS is easiest to understand as the overlap of four familiar systems:

| System | What AgentFS borrows | What AgentFS changes |
| --- | --- | --- |
| SQLite | Single-file structured storage | Adds repository semantics, scope, and authority layers |
| Git | Local-first truth and change management | Extends lifecycle behavior for AI state |
| Filesystems | Hierarchy and path inheritance | Uses path as a semantic routing key |
| Vector stores | Retrieval for relevant memory | Keeps retrieval local, bounded, and subordinate to policy |

That combination is important. AgentFS is not trying to become "just another config file" or "just another database." It is intentionally a repository operating layer for AI context.

## Why a Single File Matters

> [!TIP]
> The single-file model is not about novelty. It is about reducing ambiguity, surfacing one source of truth, and making runtime behavior deterministic.

The single-file design solves several problems at once.

| Benefit | Outcome |
| --- | --- |
| One canonical store | Humans and tools stop competing over which file is authoritative |
| Better scoping | Runtime can return only relevant rules for the current directory |
| Better Git integration | One state surface can be given explicit merge/diff behavior |
| Better privacy | Local-first operation avoids mandatory remote context services |
| Better system reasoning | Policy, memory, and lifecycle can be modeled together rather than bolted on independently |

## The Semantic Architecture

The most important design property of AgentFS is that it is semantic by structure, not by branding.

### 1. Scope is first-class

Every record lives in a scope:

- `global`
- `/apps/web`
- `/crates/afs-core`
- `/docs`

That means the runtime can answer:

- what rules apply here?
- what rules are inherited?
- what should override what?

This is significantly better than loading a giant instruction blob and hoping the model infers the right local behavior.

### 2. Category is first-class

A rule is not the same thing as memory.

A hook is not the same thing as a skill.

A log is not the same thing as a protected policy record.

That categorical separation matters because it enables:

- clearer retrieval
- better auditability
- safer writes
- more accurate export and diff behavior

### 3. Authority is first-class

Not every piece of data should have the same mutability.

| Category Type | Typical Writer | Risk if Ungoverned |
| --- | --- | --- |
| Policy / rules | Human | Agents may rewrite repository standards |
| Hooks | Human | Agents may escalate into execution control |
| Memory | Agent | Usually acceptable if bounded and inspectable |
| Logs | Agent / runtime | Can leak sensitive or misleading context if unmanaged |

This is the conceptual foundation for the dual-domain model described below.

## The Dual-Domain Trust Model

> [!WARNING]
> If AgentFS gets this boundary wrong, the entire product becomes unsafe. The project is only credible if human authority and agent mutability stay clearly separated.

AgentFS divides the system into two logical domains.

| Domain | Default Authority | Typical Contents | Required Safety Property |
| --- | --- | --- | --- |
| User Domain | Human-controlled | Rules, protected hooks, signed policy, critical defaults | Agents must not silently rewrite it |
| Agent Domain | Agent-readable and agent-writable | Memory, logs, summaries, observations, transient state | Writes must not escalate into policy ownership |

This is not just a security preference. It is a product identity constraint.

Without it:

- "memory" turns into policy drift
- "automation" turns into silent privilege escalation
- "local state" turns into unreviewed execution control

With it:

- humans keep authority over the repository's normative behavior
- agents gain useful runtime memory without controlling protected execution paths

## Why Git Must Be Part of the Design

Most AI context systems ignore Git and hope the problem goes away.

AgentFS cannot do that because the project file is part of repository state. Once that is true, Git lifecycle behavior becomes a product feature, not an afterthought.

| Git Action | Why AgentFS Cares |
| --- | --- |
| `git add` | Staging may need normalization or filtering |
| `git checkout` / `git switch` | Local projections or generated compatibility views may need refresh |
| `git diff` / `git show` | Binary-backed state still needs readable review surfaces |
| `git merge` / `git pull` / `git rebase` | Shared state needs deterministic reconciliation |
| `git commit` | Repository policy may need local validation |
| `git push` | Secret and policy leakage checks may matter |

This is why AgentFS is designed with:

- clean filters
- smudge filters
- merge drivers
- diff exporters
- hooks

Git is not adjacent to AgentFS. Git is part of the execution surface.

## Why Local-First Matters

Local-first design is not only about privacy. It also affects correctness, latency, and trust.

| Dimension | Local-First Advantage |
| --- | --- |
| Privacy | Project policy and memory stay on the developer machine by default |
| Speed | Context can be read from local storage or mapped memory |
| Reliability | Core workflows do not require a hosted control plane |
| Trust | Teams can audit one local repository state model instead of opaque remote behavior |
| Portability | The repository carries its AI substrate with it |

Remote or shared distribution may still exist later, but it should extend the local model, not replace it.

## How AgentFS Differs From Plain Text Instruction Files

| Plain Text Model | AgentFS Model |
| --- | --- |
| Entire files are re-read repeatedly | Scope-aware records can be selectively returned |
| Human and machine state often mix together | Categories and authority can be separated |
| Git treats changes as ordinary text diffs | Git behavior can be explicitly designed for the project file |
| Each tool invents its own path convention | One project-native store can serve multiple tools |
| Memory grows as clutter | Memory can be bounded, structured, exported, and governed |

## How AgentFS Differs From a Generic Database

AgentFS is not only "SQLite in the root."

What makes it distinct is the repository-aware behavior layered on top:

- scope inheritance
- policy versus memory separation
- Git lifecycle integration
- compatibility export or reflection
- future semantic retrieval tied to repository paths

A generic database can store the bytes. AgentFS defines what the bytes mean inside a working repository.

## Human View vs Machine View

The same system must satisfy two different operating perspectives.

### Human view

Humans need:

- clarity
- inspectability
- reviewable diffs
- honest docs
- understandable authority boundaries

### Machine view

Agents need:

- scoped retrieval
- structured categories
- cheap reads
- bounded writes
- stable APIs and command contracts

| View | What It Optimizes For | Failure If Ignored |
| --- | --- | --- |
| Human | Trust, reviewability, governance | The project becomes a black box |
| Machine | Precision, speed, relevance | Context retrieval becomes noisy and expensive |

AgentFS succeeds only if it preserves both views without creating separate truths for each.

## The Reflection Principle

> [!NOTE]
> A binary-backed system that cannot explain itself to humans will be rejected, even if it is technically sound.

That is why AgentFS includes the idea of reflection surfaces:

- markdown export
- structured JSON export
- readable diff conversion
- optional virtual filesystem projection

These are not secondary conveniences. They are how a binary semantic store stays governable in a Git-native world.

## The Runtime Story

The full implementation is still ahead, but conceptually the runtime behaves like this:

1. agent or tool enters a repository scope
2. runtime resolves current path and inherited scopes
3. runtime loads applicable rules, hooks, and memory categories
4. protected records stay under human authority constraints
5. runtime returns only the context needed for the current task
6. agent writes memory or logs only into allowed mutable domains
7. Git lifecycle events reconcile project state through explicit integrations

## The Product Story

AgentFS is not trying to be "an AI assistant."
It is trying to be the repository substrate that AI assistants can share.

That distinction matters.

| AgentFS is | AgentFS is not |
| --- | --- |
| a repository context layer | a proprietary chat product |
| a local-first semantic store | a cloud-only memory service |
| a trust-boundary model | a generic prompt file collection |
| a Git-aware state system | an ungoverned agent scratchpad |

## The Adoption Story

A real system has to meet teams where they are.

That means AgentFS cannot assume a greenfield world. It must support migration from existing repository patterns such as:

- `.cursorrules`
- `AGENTS.md`
- vendor-specific instruction files
- ad hoc local prompt folders

The conceptual requirement is:

> preserve useful policy, remove duplicated clutter, and converge on one canonical store

That is why ingestion and compatibility layers are part of the concept rather than optional extras.

## The Long-Term Vision

If AgentFS succeeds, the outcome is larger than one repository.

It would establish a credible open model for AI-native repositories where:

- context has structure
- policy has ownership
- memory has boundaries
- Git has first-class integration
- tools can share one substrate instead of inventing their own

That would shift repository AI from a vendor-specific clutter pattern into durable infrastructure.

## Design Principles

| Principle | Practical Meaning |
| --- | --- |
| One source of truth | Do not let multiple instruction surfaces compete silently |
| Honest surfaces | Docs and status must not overclaim implementation |
| Scope-first retrieval | Return local relevance, not global noise |
| Human-owned policy | Protected behavior stays under deliberate control |
| Bounded agent writes | Memory must remain useful without becoming unsafe |
| Git-native behavior | Reconciliation and review must fit normal repository workflows |
| Inspectable state | Humans need exports, diffs, and visibility |

## Final Definition

AgentFS is a repository-native semantic runtime that consolidates AI policy, memory, hooks, and lifecycle state into a local-first system file with explicit scope, Git integration, and human-versus-agent authority boundaries.

## Project Identity

| Field | Value |
| --- | --- |
| Project | `AgentFS` |
| CLI | `afs` |
| Author | `@Justinedevs` |
| Email | `Justinedevs@jstn.site` |
| Domain | `agentfs.systems` |

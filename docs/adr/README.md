# AgentFS ADR Index

> [!IMPORTANT]
> This directory contains implementation-facing Architecture Decision Records for the AgentFS v0.1.0 preview program. These ADRs are subordinate to the standing doctrine in `docs/Concept.md`, `docs/Roadmap.md`, `docs/TA.md`, and `docs/Architect-File-tree.md`.

> [!NOTE]
> These ADRs do not replace the founding docs. They narrow them into preview-specific implementation decisions, sprint sequencing, and rollout constraints.

## Source of Truth Contract

| Document | Primary Role |
| --- | --- |
| `docs/Concept.md` | product thesis, trust model, and conceptual philosophy |
| `docs/Roadmap.md` | phase ordering, command surface context, and release sequencing |
| `docs/TA.md` | deep technical architecture, research parallels, and system constraints |
| `docs/Architect-File-tree.md` | repository boundaries and subsystem ownership |
| `docs/adr/*` | derived implementation decisions and preview execution commitments |

## ADR Status Model

| Status | Meaning |
| --- | --- |
| `Accepted` | active decision for the preview program |
| `Superseded` | replaced by a newer ADR |
| `Deferred` | intentionally postponed beyond the preview scope |

## Phase Mapping

| Phase | ADR | Focus |
| --- | --- | --- |
| Phase 0 | `0000` | ADR contract and source-of-truth boundaries |
| Phase 1 | `0001` | v0.1.0 preview scope and command contract |
| Phase 2 | `0002`, `0003` | trust/Git/reflection baseline and subsystem ownership |
| Phase 3 | `0004` | backlog mapping and release-readiness gates |

## ADR Set

| ADR | Title | Status |
| --- | --- | --- |
| `0000` | Program Index and Source-of-Truth Contract | Accepted |
| `0001` | v0.1.0 Preview Scope and Command Contract | Accepted |
| `0002` | Trust Boundary, Git Lifecycle, and Reflection Baseline | Accepted |
| `0003` | Repository Boundary and Subsystem Ownership | Accepted |
| `0004` | Backlog and Release-Readiness Mapping | Accepted |

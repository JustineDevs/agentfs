# ADR 0003: Repository Boundary and Subsystem Ownership

- Status: Accepted
- Phase linkage: Phase 2
- Derived from:
  - `docs/Architect-File-tree.md`
  - `docs/TA.md`
  - `docs/Roadmap.md`

> [!NOTE]
> This ADR records ownership boundaries for preview planning. It does not invent new subsystem structure beyond the approved target monorepo design.

## Context

AgentFS spans:

- native runtime concerns
- Git lifecycle behavior
- CLI ergonomics
- docs and architecture truth

The preview sprint needs these ownership boundaries preserved so backlog items map to the right layer.

## Decision

Preview ownership stays aligned with the target monorepo structure.

| Area | Ownership |
| --- | --- |
| `crates/afs-core` | runtime correctness, storage, authority boundaries |
| `crates/afs-git` | merge/filter/hook lifecycle behavior |
| `apps/cli` | operator-facing ergonomics |
| `docs/` | public architecture and product truth |

## Boundary Rules

| Rule | Reason |
| --- | --- |
| CLI should not redefine core semantics | keeps runtime truth centralized |
| Git integration should not own policy doctrine | avoids mixing transport with architecture truth |
| Docs should explain, not silently override runtime assumptions | preserves honest surfaces |

## Consequences

### Positive

- helps map work cleanly into architecture-aware issues
- keeps future implementation lanes reviewable

### Negative

- forces preview planning to stay conscious of repo shape even before code exists

## Execution Impact

- issue bodies for ownership-related work must cite the layer they affect
- rollout verification should ensure subsystem references are consistent with `docs/Architect-File-tree.md`

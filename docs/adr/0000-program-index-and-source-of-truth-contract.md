# ADR 0000: Program Index and Source-of-Truth Contract

- Status: Accepted
- Phase linkage: Phase 0
- Derived from:
  - `docs/Concept.md`
  - `docs/Roadmap.md`
  - `docs/TA.md`
  - `docs/Architect-File-tree.md`

> [!IMPORTANT]
> This ADR exists to prevent the preview ADR set from becoming a competing architecture canon.

## Context

AgentFS already has four foundational doctrine documents. The preview sprint needs implementation-facing ADRs, but the repo is still documentation-first and explicitly values honest surfaces.

## Decision

The ADR set is subordinate to the four founding docs and is limited to preview-shaping implementation decisions.

### Contract

| Rule | Effect |
| --- | --- |
| Founding docs stay primary | ADRs may derive, narrow, and operationalize, but not override silently |
| ADR count stays fixed for preview | No command-fragment ADR explosion |
| ADRs must be phase-linked | Every ADR must declare which roadmap phase it serves |
| ADRs must state execution impact | Decisions must be actionable for rollout |

## Consequences

### Positive

- avoids duplicate doctrine
- keeps the preview plan reviewable
- maps decisions cleanly into the sprint

### Negative

- ADR authors must stay disciplined and avoid restating whole source docs
- later implementation drift will require explicit ADR updates rather than silent edits

## Execution Impact

- `docs/adr/README.md` must link all preview ADRs
- every ADR in this set must include status, phase linkage, and derived-from references
- rollout issues must reference ADRs as derived implementation artifacts, not top-level truth

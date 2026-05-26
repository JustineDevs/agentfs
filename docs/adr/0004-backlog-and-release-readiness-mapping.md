# ADR 0004: Backlog and Release-Readiness Mapping

- Status: Accepted
- Phase linkage: Phase 3
- Derived from:
  - `docs/Roadmap.md`
  - `docs/TA.md`
  - `docs/Concept.md`

> [!IMPORTANT]
> This ADR maps the preview decision set into backlog slices and release-readiness gates. It is the bridge between architecture truth and sprint execution.

## Context

The preview sprint must convert doctrine into:

- a bounded set of ADRs
- a bounded GitHub issue rollout
- a credible release-readiness path for Phases 0 through 3 only

## Decision

The preview backlog stays limited to roadmap Phases 0 through 3.

### Backlog map

| Phase | Preview Outcome |
| --- | --- |
| Phase 0 | ADR contract and source-of-truth boundaries established |
| Phase 1 | preview command contract locked |
| Phase 2 | trust/Git/reflection baseline and subsystem ownership locked |
| Phase 3 | backlog slicing and release-readiness gates mapped |

### Explicit exclusions

| Later-phase area | Why excluded |
| --- | --- |
| multi-agent runtime safety | Phase 4+ concern |
| semantic retrieval optimization | Phase 5+ concern |
| enterprise governance expansion | later phase concern |

## Release-Readiness Gates

| Gate | Meaning |
| --- | --- |
| ADR set complete | `README.md` plus `0000` through `0004` exist and agree with founding docs |
| Preview contract locked | `0001` keeps the approved in-scope command set only |
| Phase 2 invariants captured | `0002` and `0003` reflect trust/Git/reflection and ownership boundaries |
| Remote rollout aligned | milestone, labels, six issues, assignee, and Project 7 dates all exist and are verified |

## Consequences

### Positive

- gives the sprint a bounded definition of done
- keeps rollout aligned to the roadmap instead of drifting into future phases

### Negative

- narrows the preview narrative intentionally
- forces explicit deferral of tempting later-phase features

## Execution Impact

- issue bodies/checklists must reference the relevant roadmap phase goals and exit criteria
- verification must inspect issue content, not just issue metadata

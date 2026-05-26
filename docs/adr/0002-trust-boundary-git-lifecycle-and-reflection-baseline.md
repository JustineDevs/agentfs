# ADR 0002: Trust Boundary, Git Lifecycle, and Reflection Baseline

- Status: Accepted
- Phase linkage: Phase 2
- Derived from:
  - `docs/Concept.md`
  - `docs/Roadmap.md`
  - `docs/TA.md`

> [!WARNING]
> Git behavior, reflection, and authority are coupled surfaces in AgentFS. They must not be split into unrelated implementation assumptions during preview execution.

## Context

AgentFS is only credible if:

- human-owned policy remains protected
- `.agent.db` participates safely in Git workflows
- binary-backed state remains reviewable by humans

These are coupled constraints, not separate nice-to-haves.

## Decision

The preview baseline must preserve three invariants:

| Invariant | Meaning |
| --- | --- |
| Protected policy stays human-owned | hooks and protected rule surfaces are not casually agent-writable |
| Git lifecycle is explicit | merge, filter, and hook assumptions are part of the product surface |
| Reflection remains mandatory | binary-backed state must have a readable inspection path |

## Preview Baseline Requirements

### Trust boundary

| Domain | Baseline rule |
| --- | --- |
| User domain | protected policy and trusted hook targets remain human-controlled |
| Agent domain | mutable memory/log state must not escalate into policy ownership |

### Git lifecycle

| Mechanism | Preview expectation |
| --- | --- |
| merge driver | part of the architectural baseline even if implementation remains preview-grade |
| clean/smudge filters | treated as product-surface concepts for `.agent.db` handling |
| local hooks | required for full lifecycle coverage beyond `.gitattributes` |

### Reflection baseline

| Reflection surface | Preview expectation |
| --- | --- |
| export or readable projection | required for human inspection |
| Git-readable diff model | required for collaboration credibility |

## Consequences

### Positive

- keeps the preview honest about coupled system boundaries
- prevents a misleading “storage only” interpretation of the product

### Negative

- increases the minimum documentation burden for preview execution
- means reflection and Git lifecycle cannot be silently deferred while still claiming the full architecture is intact

## Execution Impact

- Phase 2 issues must reference these three coupled invariants directly
- verification must inspect whether resulting issue bodies reflect trust, Git, and reflection concerns rather than only ADR completion

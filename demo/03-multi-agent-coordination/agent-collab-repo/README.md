# Demo 3: Multi-Agent Coordination

> [!IMPORTANT]
> This demo proves the dual-domain model: agents may write runtime memory, but they cannot overwrite human-owned policy.

## What This Proves

| Capability | Proof |
| --- | --- |
| User-domain protection | signed human rules reject agent writes |
| Agent-domain memory | agents can append observations and task notes |
| Scope locks | concurrent work declares which path it owns |
| Append-only log | multi-agent writes remain ordered and auditable |
| `afs verify` | domain separation can be checked after the run |

## Setup

```bash
cd demo/03-multi-agent-coordination/agent-collab-repo
git init
afs init
afs add rule "Only humans may modify execution hooks" --scope global --domain user
afs add hook pre-commit "pnpm test" --scope global --domain user
```

## Scripted Demo Sequence

```bash
afs status
afs lock acquire --scope /src --owner devin
afs lock acquire --scope /tasks --owner codex
afs memory add "Devin observed src/service.ts has no retry handling" --scope /src --owner devin
afs memory add "Codex observed task-b.md requests CLI verification" --scope /tasks --owner codex
afs add rule "Agent-created policy should be blocked" --scope global --domain agent
afs verify
afs export --format json
```

Expected rejection:

```text
denied: agent-domain writer cannot mutate user-domain policy records
```

## Demo Files

| File | Purpose |
| --- | --- |
| [`tasks/agent-a.md`](./tasks/agent-a.md) | deterministic task for Agent A |
| [`tasks/agent-b.md`](./tasks/agent-b.md) | deterministic task for Agent B |
| [`fixtures/expected-status.md`](./fixtures/expected-status.md) | expected lock/status view |
| [`fixtures/expected-verify.md`](./fixtures/expected-verify.md) | expected verification output |

## Fallback Plan

If live agents are unpredictable during a presentation, use the checked-in task files and expected fixtures to show the same state transition without running autonomous tools.

# Demo 4: Team Git Workflow

> [!IMPORTANT]
> This demo proves that a binary-backed AgentFS store can still be reviewed, merged, and audited in a team Git workflow.

## What This Proves

| Capability | Proof |
| --- | --- |
| Custom merge driver | two branches can modify `.agent.db` and reconcile deterministically |
| Readable diff | `.agent.db` changes can be exported into reviewable text |
| Clean/smudge filters | Git lifecycle hooks can transform or validate the store |
| PR review surface | exported markdown can show reviewers what policy changed |

## Setup

```bash
cd demo/04-team-git-workflow/team-repo
git init
afs init
git add .gitattributes README.md src fixtures
git commit -m "Prepare AgentFS team workflow demo"
```

## Required Git Attributes

The demo includes a planned `.gitattributes` contract:

```gitattributes
.agent.db merge=afs-merge diff=afs-diff filter=afs
```

## Branch Scenario

```bash
git checkout -b branch-alice
afs add rule "Always write JSDoc for public APIs" --scope /src
git add .agent.db
git commit -m "Alice adds public API documentation policy"

git checkout main
afs add rule "Use Vitest for all tests" --scope global
git add .agent.db
git commit -m "Main adds test framework policy"

git merge branch-alice
afs export --format markdown
git diff HEAD~1
```

## Expected Merge Result

| Rule | Scope | Source Branch |
| --- | --- | --- |
| Use Vitest for all tests | `global` | `main` |
| Always write JSDoc for public APIs | `/src` | `branch-alice` |

## Demo Safety Notes

- Run this scenario before presenting it live.
- Keep [`fixtures/expected-merge-export.md`](./fixtures/expected-merge-export.md) ready as fallback output.
- If a machine lacks `afs`, Git should show a clear missing-driver error instead of silently corrupting `.agent.db`.

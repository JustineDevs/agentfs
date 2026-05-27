# AgentFS Demo Workspaces

> [!IMPORTANT]
> These demo workspaces are preparation fixtures, not generated build artifacts. Each demo is intentionally small, reproducible, and focused on one capability cluster so the AgentFS preview can be explained from scratch to end.

> [!NOTE]
> The current repository is documentation-first. The demo commands describe the intended `afs` behavior and include expected outputs/fixtures so contributors can implement against concrete examples.

## Demo Matrix

| Demo | Project | Capability Cluster | Main Proof |
| --- | --- | --- | --- |
| 01 | [`my-monorepo`](./01-greenfield-monorepo/my-monorepo/README.md) | greenfield init, scoped rules, inheritance, export | clean root and readable scoped state |
| 02 | [`legacy-repo`](./02-legacy-repo-migration/legacy-repo/README.md) | vendor-file ingestion and cleanup | migration without losing existing policy |
| 03 | [`agent-collab-repo`](./03-multi-agent-coordination/agent-collab-repo/README.md) | locks, event log, trust boundary | agents can write memory but cannot mutate human policy |
| 04 | [`team-repo`](./04-team-git-workflow/team-repo/README.md) | merge driver, diff export, PR review | binary state can participate in team Git workflows |

## Cross-Demo Setup

All demos assume the preview CLI is available as:

```bash
afs --version
```

Expected preview command families:

```bash
afs init
afs add rule "..." --scope <scope>
afs add hook <hook-name> "..." --scope <scope>
afs list rules --scope <scope>
afs export --format markdown
afs ingest --from <file> --dry-run
afs ingest --clean
afs status
afs doctor
afs verify
```

## Cross-Demo Preparation Checklist

| Item | Why It Matters |
| --- | --- |
| All demo repos initialize cleanly | the audience can reproduce the flow |
| Expected export files are checked in | binary mistrust is answered immediately |
| `afs doctor` output is predictable | diagnostics prove the environment contract |
| Scope diagrams are described in README files | scope resolution is otherwise abstract |
| Legacy vendor files are realistic | migration must feel recognizable |
| Multi-agent actions are scripted | live agent behavior is hard to demo cold |
| Git merge scenario is pre-staged in fixtures | merge drivers are the highest-risk path |

## Capability Coverage

| Capability | Demo 1 | Demo 2 | Demo 3 | Demo 4 |
| --- | --- | --- | --- | --- |
| `afs init` | yes | yes | yes | yes |
| Scoped rule CRUD | yes |  |  |  |
| Scope inheritance | yes |  |  |  |
| `afs export` | yes | yes | yes | yes |
| `afs ingest` |  | yes |  |  |
| Vendor migration |  | yes |  |  |
| Trust boundary enforcement |  |  | yes |  |
| Multi-agent locks |  |  | yes |  |
| Git merge driver |  |  |  | yes |
| Readable PR diff |  |  |  | yes |
| `afs verify` / `afs doctor` | yes | yes | yes | yes |

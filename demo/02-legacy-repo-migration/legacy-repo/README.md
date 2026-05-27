# Demo 2: Legacy Repo Migration

> [!IMPORTANT]
> This demo proves that AgentFS can ingest existing vendor-specific AI files, preview the migration, and converge them into one canonical store.

## What This Proves

| Capability | Proof |
| --- | --- |
| Vendor parsing | `.cursorrules`, `CLAUDE.md`, `AGENTS.md`, and Copilot instructions are discoverable inputs |
| `afs ingest --dry-run` | migration can be reviewed before any destructive cleanup |
| `afs ingest --clean` | vendor clutter can be removed after canonicalization |
| Scope assignment | root-level vendor files map to `global` unless a nested path says otherwise |
| Human export | migrated policy remains readable through markdown export |

## Setup

```bash
cd demo/02-legacy-repo-migration/legacy-repo
git init
afs init
```

## Demo Sequence

```bash
afs ingest --from .cursorrules --dry-run
afs ingest --from CLAUDE.md --dry-run
afs ingest --from AGENTS.md --dry-run
afs ingest --from .github/copilot-instructions.md --dry-run
afs ingest --clean
afs list rules
afs export --format markdown
```

## Scope Assignment Rule

| Source File | Demo Scope | Reason |
| --- | --- | --- |
| `.cursorrules` | `global` | root-level Cursor policy applies across the repo |
| `CLAUDE.md` | `global` | root-level Claude project policy |
| `AGENTS.md` | `global` | root-level agent operating contract |
| `.github/copilot-instructions.md` | `global` | GitHub Copilot repository-wide guidance |

## Migration Safety Rules

- `--dry-run` must show extracted records, proposed categories, and scopes.
- `--clean` must not run without either a dry-run preview or explicit confirmation.
- Duplicate records should be grouped for review instead of silently discarded.
- Contradictory rules should be imported with conflict notes, not merged automatically.

## Expected Output

See [`fixtures/expected-ingest-preview.md`](./fixtures/expected-ingest-preview.md) and [`fixtures/expected-export.md`](./fixtures/expected-export.md).

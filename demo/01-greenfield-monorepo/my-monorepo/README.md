# Demo 1: Greenfield Monorepo

> [!IMPORTANT]
> This demo proves that a new repository can adopt AgentFS without adding vendor-specific AI clutter files.

## What This Proves

| Capability | Proof |
| --- | --- |
| `afs init` | creates `.agent.db`, Git integration, and a clean runtime surface |
| Scoped rules | records can target `global`, `/apps/web`, `/apps/api`, and `/packages/ui` |
| Scope inheritance | nested paths inherit global rules while preserving local specificity |
| `afs export` | binary-backed state can be audited as markdown |
| Clean root | no `.cursorrules`, `CLAUDE.md`, Copilot instructions, or vendor prompt folders are required |

## Project Shape

```text
my-monorepo/
├── apps/
│   ├── web/
│   │   └── src/page.tsx
│   └── api/
│       └── src/route.ts
├── packages/
│   └── ui/
│       └── src/button.ts
├── fixtures/
│   └── expected-export.md
└── package.json
```

## Setup

```bash
cd demo/01-greenfield-monorepo/my-monorepo
git init
afs init
```

Expected filesystem result:

```text
.agent.db
.afs/virtual/
.git/config contains merge driver registration
```

## Demo Sequence

```bash
afs add rule "Always use strict TypeScript" --scope global
afs add rule "Use React Server Components by default" --scope /apps/web
afs add rule "Never expose internal DB types in API responses" --scope /apps/api
afs add rule "Export stable component APIs from packages/ui" --scope /packages/ui

afs list rules --scope /apps/web
afs list rules --scope /apps/api
afs export --format markdown
git status --short
```

## Expected Scope Resolution

| Query Scope | Inherited Records | Local Records |
| --- | --- | --- |
| `/apps/web` | `Always use strict TypeScript` | `Use React Server Components by default` |
| `/apps/api` | `Always use strict TypeScript` | `Never expose internal DB types in API responses` |
| `/packages/ui` | `Always use strict TypeScript` | `Export stable component APIs from packages/ui` |

## Demo Success Criteria

- `afs list rules --scope /apps/web` returns global + web rules.
- `afs list rules --scope /apps/api` returns global + API rules.
- `afs export --format markdown` matches [`fixtures/expected-export.md`](./fixtures/expected-export.md).
- The root contains project files and `.agent.db`, not vendor-specific AI files.

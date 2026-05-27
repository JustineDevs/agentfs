# AgentFS Export: Greenfield Monorepo

## Rules

| ID | Scope | Content |
| --- | --- | --- |
| `rule-global-typescript` | `global` | Always use strict TypeScript |
| `rule-web-rsc` | `/apps/web` | Use React Server Components by default |
| `rule-api-db-types` | `/apps/api` | Never expose internal DB types in API responses |
| `rule-ui-public-api` | `/packages/ui` | Export stable component APIs from packages/ui |

## Scope Resolution Example

```text
/apps/web
├── inherited: global -> Always use strict TypeScript
└── local: /apps/web -> Use React Server Components by default
```

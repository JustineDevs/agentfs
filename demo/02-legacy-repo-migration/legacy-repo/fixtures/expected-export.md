# AgentFS Export: Legacy Migration

## Imported Sources

| Source | Scope | Category |
| --- | --- | --- |
| `.cursorrules` | `global` | `rule` |
| `CLAUDE.md` | `global` | `rule` |
| `AGENTS.md` | `global` | `rule` |
| `.github/copilot-instructions.md` | `global` | `rule` |

## Migration Result

```text
before: 4 vendor-specific AI instruction files
after:  .agent.db + human docs
```

`CONTRIBUTING.md` remains because it is human-facing project documentation, not agent runtime policy.

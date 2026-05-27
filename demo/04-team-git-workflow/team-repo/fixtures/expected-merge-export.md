# Expected Merge Export

## Rules After Merge

| ID | Scope | Content | Source |
| --- | --- | --- | --- |
| `rule-global-vitest` | `global` | Use Vitest for all tests | `main` |
| `rule-src-jsdoc` | `/src` | Always write JSDoc for public APIs | `branch-alice` |

## Merge Driver Notes

```text
strategy: deterministic timestamp reconciliation
conflicts: 0
sqlite_integrity_check: ok
diff_export: markdown
```

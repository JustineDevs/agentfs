# Expected Ingest Preview

| Source | Proposed Scope | Records |
| --- | --- | --- |
| `.cursorrules` | `global` | 10 rules |
| `CLAUDE.md` | `global` | 10 rules |
| `AGENTS.md` | `global` | 10 rules |
| `.github/copilot-instructions.md` | `global` | 10 rules |

## Duplicate Candidates

- strict TypeScript appears in Cursor and Copilot guidance
- public API response discipline appears in Cursor and Claude guidance
- root clutter avoidance appears in Copilot and AgentFS migration goals

## Required Operator Step

The dry run must ask the operator to review duplicates and contradictions before `--clean`.

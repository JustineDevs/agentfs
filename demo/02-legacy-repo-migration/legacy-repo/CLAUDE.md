# Claude Project Instructions

- Treat this repository as a production TypeScript service.
- Preserve existing module boundaries.
- Explain database and authentication changes before editing.
- Prefer deterministic tests over snapshot-only tests.
- Do not change public response shapes without updating documentation.
- Use the smallest working patch for hotfixes.
- Add rollback notes for migrations.
- Keep secrets out of examples and logs.
- Verify command output before claiming success.
- Ask for clarification only when a decision changes product behavior.

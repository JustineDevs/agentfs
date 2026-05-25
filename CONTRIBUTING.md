# Contributing to AgentFS

## Before You Start

AgentFS is currently in a documentation-first stage. That matters for contributors:

- architecture clarity is part of the product
- docs must not overclaim shipped functionality
- implementation work should align with the vocabulary and boundaries already established in the repository

Read these files first:

- [README.md](README.md)
- [docs/Concept.md](docs/Concept.md)
- [docs/Architect-File-tree.md](docs/Architect-File-tree.md)
- [docs/Roadmap.md](docs/Roadmap.md)
- [SECURITY.md](SECURITY.md)

## What Good Contributions Look Like

High-signal contributions usually do one or more of the following:

- clarify architecture or trust boundaries
- improve repository honesty and technical precision
- add implementation that follows the documented subsystem ownership
- improve tests, validation, or release safety
- reduce ambiguity in commands, scope rules, or Git lifecycle behavior

## Before Opening a Pull Request

1. Check whether the change matches the current roadmap.
2. Keep the change narrowly scoped.
3. Update documentation when behavior, commands, or project claims change.
4. Do not mark features as complete unless they are actually implemented and verifiable.

## Repository Conventions

### Documentation honesty

Do not present planned features as already shipped.

If a section describes a target architecture or planned command surface, label it that way. This is especially important in:

- `README.md`
- release notes
- command examples
- roadmap status sections

### Boundary ownership

Follow the architectural ownership described in `docs/Architect-File-tree.md`.

Examples:

- runtime correctness belongs in the native core
- Git merge and filter behavior belong in the Git integration layer
- wrappers should orchestrate rather than duplicate core logic

### Security expectations

Do not weaken the human-versus-agent trust boundary without explicit documentation and review.

Security-sensitive areas include:

- hook execution
- rule ownership
- signature checks
- export surfaces
- logs and local state handling

## Pull Request Guidance

Pull requests should include:

- what changed
- why it changed
- what was verified
- any known gaps

If your change affects commands, flows, or trust assumptions, update the relevant docs in the same PR.

## Commit Guidance

This repository uses structured commit messages with decision-record trailers. Follow the repo’s Lore commit protocol from `AGENTS.md` when creating commits.

At minimum, commit messages should explain:

- why the change exists
- key constraints
- what was tested
- what was intentionally not tested

## Suggested Workflow

```bash
git checkout -b <your-branch>
# make focused changes
git diff
git status
```

If the repository gains build, lint, or test commands, run the relevant verification before asking for review.

## Reporting Bugs and Proposing Features

When opening issues:

- provide a clear summary
- explain the expected behavior
- explain the current behavior
- include reproduction steps for bugs
- keep proposals concrete and scoped

For major architecture or workflow changes, start with a design discussion before writing a large patch.

## Style Expectations

- prefer precise wording over hype
- keep diffs reviewable
- avoid unnecessary dependencies
- preserve repo-native terminology
- use explicit names and honest status labels

## Security Reports

Do not open public issues for suspected vulnerabilities. Follow [SECURITY.md](SECURITY.md).

## Questions

If a contribution would materially change the architecture, trust model, or public project claims, raise that early before broad implementation work begins.

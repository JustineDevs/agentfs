# Security Policy

## Scope

AgentFS is an infrastructure-oriented project. Security issues include, but are not limited to:

- unauthorized modification of protected rules or hooks
- trust-boundary failures between human-owned policy and agent-owned state
- command injection paths in hook or CLI flows
- unsafe Git merge, filter, or diff behavior
- local data leakage through exports, logs, or generated reflection layers
- signature verification bypasses

If you find a potential security issue, report it privately. Do not open a public issue first.

## Supported Security Review Window

This repository is currently documentation-first and pre-implementation. That means there is no stable release branch yet, but security reports are still welcome for:

- documentation that encourages unsafe behavior
- design flaws in the proposed trust model
- implementation work added in future branches or pull requests

Once tagged releases exist, this file should be updated with a version support table.

## Reporting a Vulnerability

Send reports to:

- `Justinedevs@jstn.site`

Use the subject line:

```text
[AgentFS Security] <short summary>
```

Include as much of the following as possible:

- affected file, command, or subsystem
- clear description of the issue
- reproduction steps or proof of concept
- impact assessment
- suggested fix, if you have one

## What To Expect

Best-effort response targets:

- initial acknowledgment within 7 days
- triage update after review
- coordinated disclosure after a fix or mitigation is available

These are targets, not guarantees, especially while the project is still in its early formation stage.

## Disclosure Guidelines

Please:

- avoid public disclosure until the issue has been reviewed
- avoid publishing exploit details before a fix or mitigation exists
- keep reports factual and reproducible

## Out of Scope

The following are generally out of scope unless they create a concrete trust or execution issue:

- spelling or grammar mistakes
- hypothetical attacks without a plausible execution path
- reports that depend on unsupported tooling or modified local environments only

## Security Principles

AgentFS is being designed around these core security constraints:

- humans own protected policy and executable hook targets
- agents may write runtime memory but must not silently escalate into policy ownership
- Git integration must remain auditable and deterministic
- local-first operation must not imply unsafe secret handling

Security-sensitive changes should preserve those constraints.

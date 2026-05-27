# AgentFS Benchmarks

> [!IMPORTANT]
> Benchmarks are not product code. They are evidence infrastructure for latency, throughput, contention, and end-to-end workload claims.

> [!NOTE]
> The repository is still documentation-first. These benchmark files define the intended measurement contract and scenario shape before the Rust crates exist.

## Purpose

AgentFS makes performance-sensitive claims:

- local context lookup should be fast enough for agent turns
- SQLite WAL writes must remain predictable
- lock contention must be measurable before multi-agent claims are made
- Git lifecycle filters must not make normal development feel slow
- vector retrieval must be optional, bounded, and measurable

The `benchmarks/` tree exists to make those claims testable.

## Layout

| Directory | Owns |
| --- | --- |
| `harness/` | benchmark config, lifecycle, warmup, iteration control, report aggregation |
| `micro/` | isolated subsystem benchmarks |
| `macro/` | end-to-end workflow simulations |
| `fixtures/` | synthetic repos and workload traces |
| `results/` | generated runs and committed baselines |

## Benchmark Categories

| Category | Files | Measures |
| --- | --- | --- |
| Database read path | `micro/db_read.rs` | scoped rule lookup latency |
| WAL write path | `micro/db_write.rs` | write throughput and checkpoint pressure |
| Vector retrieval | `micro/vector_query.rs` | semantic query latency and memory pressure |
| Lock contention | `micro/lock_contention.rs` | lock acquisition and retry behavior |
| Agent session | `macro/agent_session.rs` | full context resolution workload |
| Git lifecycle | `macro/git_lifecycle.rs` | clean/smudge/diff/merge overhead |

## Expected Harness Flow

```text
load config
  -> prepare fixture repo
  -> warm up runtime
  -> run iterations
  -> aggregate p50 / p95 / p99 / mean / stddev
  -> emit report
```

## Result Policy

Raw generated outputs are ignored by Git. Baselines are committed only when intentionally used as release evidence.

| Path | Git Policy |
| --- | --- |
| `benchmarks/results/*.json` | ignored |
| `benchmarks/results/*.md` | ignored |
| `benchmarks/results/baselines/*` | committed intentionally |
| `benchmarks/results/.gitkeep` | committed |

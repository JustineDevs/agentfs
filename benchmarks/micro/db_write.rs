//! Micro-benchmark: WAL-backed write path.
//!
//! Target:
//! - insert rule, hook, memory, and event records
//! - measure WAL growth and checkpoint overhead
//! - verify crash-safe write policy assumptions

pub const BENCHMARK_NAME: &str = "micro/db_write";
pub const WRITE_BATCH_SIZE: u32 = 1_000;
pub const CHECKPOINT_POLICY: &str = "deterministic-phase-boundary";

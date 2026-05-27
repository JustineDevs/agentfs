//! Micro-benchmark: multi-agent lock contention.
//!
//! Target:
//! - simulate several agents attempting scope locks
//! - measure acquisition latency, retry count, timeout rate, and stale-lock cleanup

pub const BENCHMARK_NAME: &str = "micro/lock_contention";
pub const AGENT_COUNT: u32 = 8;
pub const LOCK_SCOPE: &str = "/src";
pub const LOCK_TTL_SECONDS: u32 = 30;

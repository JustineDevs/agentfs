//! Micro-benchmark: scoped database read path.
//!
//! Target:
//! - open `.agent.db`
//! - resolve `global -> parent scopes -> current scope`
//! - return matching rules for a path such as `/apps/web/src/routes`
//!
//! Primary metrics:
//! - p50 / p95 / p99 lookup latency
//! - mapped-page read pressure
//! - result count correctness

pub const BENCHMARK_NAME: &str = "micro/db_read";
pub const TARGET_SCOPE: &str = "/apps/web/src/routes";
pub const P95_BUDGET_MS: f64 = 1.0;

//! Benchmark configuration schema for AgentFS performance runs.
//!
//! This module is a design contract for the future Rust harness. It defines
//! the dimensions every benchmark must make explicit before it can produce a
//! trustworthy result.

pub struct BenchmarkConfig {
    pub name: &'static str,
    pub fixture: &'static str,
    pub warmup_iterations: u32,
    pub measured_iterations: u32,
    pub concurrency: u32,
    pub duration_seconds: u32,
}

pub const DEFAULT_LOCAL_READ: BenchmarkConfig = BenchmarkConfig {
    name: "db_read",
    fixture: "benchmarks/fixtures/small_repo",
    warmup_iterations: 100,
    measured_iterations: 1_000,
    concurrency: 1,
    duration_seconds: 30,
};

pub const DEFAULT_MULTI_AGENT: BenchmarkConfig = BenchmarkConfig {
    name: "lock_contention",
    fixture: "benchmarks/fixtures/concurrent_agents",
    warmup_iterations: 25,
    measured_iterations: 250,
    concurrency: 8,
    duration_seconds: 60,
};

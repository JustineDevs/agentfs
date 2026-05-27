//! Benchmark lifecycle runner.
//!
//! The production harness should own setup, warmup, measured iterations, and
//! teardown. Runtime crates should expose measurable APIs; they should not own
//! benchmark orchestration.

use crate::config::BenchmarkConfig;

pub struct BenchmarkRun {
    pub config: BenchmarkConfig,
}

impl BenchmarkRun {
    pub fn new(config: BenchmarkConfig) -> Self {
        Self { config }
    }

    pub fn lifecycle_steps(&self) -> [&'static str; 5] {
        ["prepare_fixture", "warmup", "measure", "aggregate", "teardown"]
    }
}

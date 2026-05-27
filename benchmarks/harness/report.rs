//! Statistical report contract for AgentFS benchmarks.

pub struct BenchmarkReport {
    pub benchmark: &'static str,
    pub p50_ms: f64,
    pub p95_ms: f64,
    pub p99_ms: f64,
    pub mean_ms: f64,
    pub stddev_ms: f64,
}

impl BenchmarkReport {
    pub fn is_release_candidate_ready(&self, p95_budget_ms: f64) -> bool {
        self.p95_ms <= p95_budget_ms
    }
}

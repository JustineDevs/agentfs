//! Macro-benchmark: Git lifecycle integration.
//!
//! Target:
//! - run clean filter simulation
//! - run smudge filter simulation
//! - run diff export simulation
//! - run merge-driver simulation on two diverged stores

pub const BENCHMARK_NAME: &str = "macro/git_lifecycle";
pub const FIXTURE: &str = "benchmarks/fixtures/large_repo";
pub const REQUIRED_GIT_ATTRIBUTES: &str = ".agent.db merge=afs-merge diff=afs-diff filter=afs";

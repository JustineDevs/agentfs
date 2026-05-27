//! Macro-benchmark: complete agent context session.
//!
//! Target:
//! - initialize fixture repo
//! - resolve active scope
//! - fetch inherited rules
//! - fetch hooks and relevant memory
//! - emit pruned context payload

pub const BENCHMARK_NAME: &str = "macro/agent_session";
pub const SESSION_SCOPE: &str = "/apps/web/src/routes";
pub const EXPECTED_OUTPUT: &str = "pruned_context_payload";

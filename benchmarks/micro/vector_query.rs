//! Micro-benchmark: semantic vector retrieval.
//!
//! Target:
//! - compare raw BLOB lookup, quantized payload lookup, and optional
//!   sqlite-vec/sqlite-vss provider-backed lookup
//! - measure latency and memory growth independently from agent workflow cost

pub const BENCHMARK_NAME: &str = "micro/vector_query";
pub const VECTOR_DIMENSIONS: u16 = 256;
pub const PROVIDERS: [&str; 3] = ["blob_exact", "sqlite_vec", "sqlite_vss"];

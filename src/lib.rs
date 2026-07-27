// ConaBuildConfig - Resolution Cascade cho build paths
// Tier 1: Explicit config, Tier 2: ENV, Tier 3: Workspace fallback

pub mod build_config;

pub use build_config::{ConaBuildConfig, ConaBuildError, ConaWasmBuilder};
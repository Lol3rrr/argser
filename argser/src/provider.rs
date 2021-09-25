//! This represents the Collection of Providers that collect Arguments from
//! various sources that are commonly used.

mod cli;
pub use cli::Cli;

mod envs;
pub use envs::Env;

mod fixed;
pub use fixed::*;

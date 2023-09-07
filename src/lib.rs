pub mod config;
pub(crate) mod strings;

pub use config::*;
pub use strings::*;

// use std::*;

use anyhow::Result;

pub async fn run(_config: RelayerConfig) -> Result<()> {
    Ok(())
}

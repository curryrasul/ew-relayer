pub mod config;
pub(crate) mod imap_client;
pub(crate) mod smtp_client;
pub(crate) mod strings;

pub(crate) use imap_client::*;
pub(crate) use smtp_client::*;
pub(crate) use strings::*;

pub use config::*;

use anyhow::Result;

pub async fn run(_config: RelayerConfig) -> Result<()> {
    Ok(())
}

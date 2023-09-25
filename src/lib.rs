#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]

pub(crate) mod chain;
pub mod config;
pub(crate) mod database;
pub(crate) mod imap_client;
pub(crate) mod smtp_client;
pub(crate) mod strings;

pub(crate) use chain::*;
pub(crate) use database::*;
pub(crate) use imap_client::*;
pub(crate) use smtp_client::*;
pub(crate) use strings::*;

pub use config::*;

use anyhow::{anyhow, Result};

const MAX_CHANNEL_LENGTH: usize = 250;

pub async fn run(config: RelayerConfig) -> Result<()> {
    /*
        (tx, rx) = mpsc::new(100)
        first = task::spawn({
            spawn_blocking(imap::new())
            loop {
                new_email = spawn_blocking(imap.wait_new_email).await
                tx.send(new_email)
            }
        });

        second = tokio::spawn({
            loop {
                new_email = rx.recv().await;
                tokio::spawn({ handle_email(new_email).await })
            }
        });
    */

    let (tx, mut rx) = tokio::sync::mpsc::channel(MAX_CHANNEL_LENGTH);

    let email_receiver_task = tokio::task::spawn(async move {
        let mut email_receiver = ImapClient::new(config.imap_config)?;
        loop {
            let emails = email_receiver.retrieve_new_emails()?;
            for email in emails {
                tx.send(email).await?;
            }
        }
        Ok::<(), anyhow::Error>(())
    });

    let email_handler_task = tokio::task::spawn(async move {
        loop {
            let email = rx
                .recv()
                .await
                .ok_or(anyhow!(CANNOT_GET_EMAIL_FROM_QUEUE))?;
            handle_email().await?
        }
        Ok::<(), anyhow::Error>(())
    });

    let _ = tokio::join!(email_receiver_task, email_handler_task);

    Ok(())
}

async fn handle_email() -> Result<()> {
    todo!()
}

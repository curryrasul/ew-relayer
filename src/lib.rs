#![allow(dead_code)]
#![allow(unused_variables)]

pub mod config;
pub(crate) mod imap_client;
pub(crate) mod smtp_client;
pub(crate) mod strings;

pub(crate) use imap_client::*;
pub(crate) use smtp_client::*;
pub(crate) use strings::*;

pub use config::*;

use anyhow::Result;

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

    let receiver = tokio::task::spawn_blocking(|| imap_client::ImapClient::new(config.imap_config));
    let sender = tokio::task::spawn_blocking(|| smtp_client::SmtpClient::new(config.smtp_config));

    let _ = tokio::join!(receiver, sender);
    println!("Sender and receiver connected succesfully");

    Ok(())
}

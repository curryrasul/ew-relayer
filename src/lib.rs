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

    let mut receiver =
        tokio::task::spawn_blocking(|| imap_client::ImapClient::new(config.imap_config)).await??;

    let v = tokio::task::spawn_blocking(move || receiver.retrieve_new_emails()).await??;
    println!("Got new message!");
    for mail in v {
        for m in mail.iter() {
            if let Some(b) = m.body() {
                println!("{}", String::from_utf8(b.to_vec()).unwrap());
            }
        }
    }

    Ok(())
}

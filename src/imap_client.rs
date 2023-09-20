use std::net::SocketAddr;

use anyhow::Result;
use tokio::net::TcpStream;
use tokio_native_tls::native_tls;
use tokio_native_tls::TlsConnector;

pub(crate) enum ImapAuth {
    Password {
        user_id: String,
        password: String,
    },
    Oauth {
        user_id: String,
        client_id: String,
        client_secret: String,
        auth_url: String,
        token_url: String,
        redirect_url: String,
    },
}

pub(crate) struct ImapConfig {
    pub(crate) imap_domain_name: String,
    pub(crate) imap_port: u16,
    pub(crate) imap_auth_type: ImapAuth,
}

pub(crate) struct ImapClient {}

impl ImapClient {
    pub(crate) async fn new(imap_config: ImapConfig) -> Result<Self> {
        let addr = (&*imap_config.imap_domain_name, imap_config.imap_port);
        let stream = TcpStream::connect(&addr).await?;
        let tls_stream = TlsConnector::from(native_tls::TlsConnector::new()?)
            .connect(&imap_config.imap_domain_name, stream)
            .await?;

        Ok(Self {})
    }
}

use lettre::{
    message::{
        header::{Cc, From, Header, HeaderName, InReplyTo, ReplyTo, To},
        Mailbox, Mailboxes, MessageBuilder,
    },
    transport::smtp::{
        authentication::Credentials, client::SmtpConnection, commands::*, extension::ClientId,
        SMTP_PORT,
    },
    Address, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

use crate::*;

pub(crate) struct SmtpConfig {
    pub(crate) domain_name: String,
    pub(crate) id: String,
    pub(crate) password: String,
}

pub(crate) struct SmtpClient {
    email_id: String,
    transport: AsyncSmtpTransport<Tokio1Executor>,
}

impl SmtpClient {
    pub(crate) fn new(config: SmtpConfig) -> Result<Self> {
        let creds = Credentials::new(config.id.clone(), config.password);
        let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(&config.domain_name)?
            .credentials(creds)
            .build();

        Ok(Self {
            email_id: config.id,
            transport,
        })
    }

    pub(crate) async fn send_new_email(
        &self,
        email_subject: &str,
        email_body: &str,
        email_to: &str,
    ) -> Result<()> {
        let from_mbox = Mailbox::new(None, self.email_id.parse::<Address>()?);
        let to_mbox = Mailbox::new(None, email_to.parse::<Address>()?);

        let email = Message::builder()
            .from(from_mbox)
            .subject(email_subject)
            .to(to_mbox)
            .body(email_body.to_string())?;

        self.transport.send(email).await?;

        Ok(())
    }
}

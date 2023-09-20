use crate::*;

use std::net::TcpStream;

use anyhow::{anyhow, Result};
use imap::Session;
use native_tls::TlsStream;
use oauth2::reqwest::http_client;
use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    PkceCodeChallenge, RedirectUrl, Scope, TokenResponse, TokenUrl,
};

#[derive(Clone)]
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

#[derive(Clone)]
pub(crate) struct ImapConfig {
    pub(crate) domain_name: String,
    pub(crate) port: u16,
    pub(crate) auth: ImapAuth,
}

struct OAuth2 {
    user_id: String,
    access_token: String,
}

impl imap::Authenticator for OAuth2 {
    type Response = String;
    fn process(&self, _: &[u8]) -> Self::Response {
        format!(
            "user={}\x01auth=Bearer {}\x01\x01",
            self.user_id, self.access_token
        )
    }
}

pub(crate) struct ImapClient {
    session: Session<TlsStream<TcpStream>>,
    config: ImapConfig,
}

impl ImapClient {
    pub(crate) fn new(config: ImapConfig) -> Result<Self> {
        let client = imap::ClientBuilder::new(&*config.domain_name, config.port)
            .native_tls()
            .expect("Could not connect to imap server");

        let mut session = match config.auth.clone() {
            ImapAuth::Password { user_id, password } => {
                client.login(user_id, password).map_err(|e| e.0)
            }
            ImapAuth::Oauth {
                user_id,
                client_id,
                client_secret,
                auth_url,
                token_url,
                redirect_url,
            } => {
                let oauth_client = BasicClient::new(
                    ClientId::new(client_id),
                    Some(ClientSecret::new(client_secret)),
                    AuthUrl::new(auth_url)?,
                    Some(TokenUrl::new(token_url)?),
                )
                .set_redirect_uri(RedirectUrl::new(redirect_url)?);

                let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
                let (auth_url, _) = oauth_client
                    .authorize_url(CsrfToken::new_random)
                    .add_scope(Scope::new("https://mail.google.com/".to_string()))
                    .set_pkce_challenge(pkce_challenge)
                    .url();

                let mut auth_code = String::new();
                std::io::stdin().read_line(&mut auth_code)?;

                let token_result = oauth_client
                    .exchange_code(AuthorizationCode::new(auth_code))
                    // Set the PKCE code verifier.
                    .set_pkce_verifier(pkce_verifier)
                    .request(http_client)?;

                let access_token = serde_json::to_string(token_result.access_token())?;
                let oauthed = OAuth2 {
                    user_id,
                    access_token,
                };

                client.authenticate("XOAUTH2", &oauthed).map_err(|e| e.0)
            }
        }?;

        session.debug = true;
        session.select("INBOX")?;

        Ok(Self { session, config })
    }

    fn reconnect(&mut self) -> Result<()> {
        const MAX_RETRIES: u32 = 5;
        let mut retry_count = 0;

        while retry_count < MAX_RETRIES {
            match ImapClient::new(self.config.clone()) {
                Ok(new_client) => {
                    self.session = new_client.session;
                    return Ok(());
                }
                Err(_) => {
                    retry_count += 1;
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                }
            }
        }

        Err(anyhow!("{IMAP_RECONNECT_ERROR}"))
    }
}

use crate::*;

use std::env;

use dotenv::dotenv;

pub struct RelayerConfig {
    pub(crate) imap_config: ImapConfig,
    pub(crate) smtp_config: SmtpConfig,

    pub(crate) circom_prover_path: String,

    pub(crate) etherscan_key: String,
}

impl RelayerConfig {
    pub fn new() -> Self {
        dotenv().ok();

        let imap_auth_type = env::var(IMAP_AUTH_TYPE_KEY).unwrap();
        let imap_auth_type = match &*imap_auth_type {
            "password" => ImapAuth::Password {
                user_id: env::var(LOGIN_ID_KEY).unwrap(),
                password: env::var(LOGIN_PASSWORD_KEY).unwrap(),
            },
            "oauth" => ImapAuth::Oauth {
                user_id: env::var(LOGIN_ID_KEY).unwrap(),
                client_id: env::var(IMAP_CLIENT_ID_KEY).unwrap(),
                client_secret: env::var(IMAP_CLIENT_SECRET_KEY).unwrap(),
                auth_url: env::var(IMAP_AUTH_URL_KEY).unwrap(),
                token_url: env::var(IMAP_TOKEN_URL_KEY).unwrap(),
                redirect_url: env::var(IMAP_REDIRECT_URL_KEY).unwrap(),
            },
            _ => panic!("{WRONG_AUTH_METHOD}"),
        };

        let imap_config = ImapConfig {
            imap_domain_name: env::var(IMAP_DOMAIN_NAME_KEY).unwrap(),
            imap_port: env::var(IMAP_PORT_KEY).unwrap().parse().unwrap(),
            imap_auth_type,
        };

        let smtp_config = SmtpConfig {
            smtp_domain_name: env::var(SMTP_DOMAIN_NAME_KEY).unwrap(),
            id: env::var(LOGIN_ID_KEY).unwrap(),
            password: env::var(LOGIN_PASSWORD_KEY).unwrap(),
        };

        Self {
            imap_config,
            smtp_config,

            circom_prover_path: env::var(ZK_EMAIL_PATH_KEY).unwrap(),
            etherscan_key: env::var(ETHERSCAN_KEY).unwrap(),
        }
    }
}

impl Default for RelayerConfig {
    fn default() -> Self {
        Self::new()
    }
}

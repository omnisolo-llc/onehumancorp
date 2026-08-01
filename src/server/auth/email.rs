use async_trait::async_trait;
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor, message::Mailbox,
    transport::smtp::authentication::Credentials,
};

#[async_trait]
pub trait VerificationMailer: Send + Sync {
    async fn send_verification_code(&self, recipient: &str, code: &str) -> Result<(), String>;
    fn configured(&self) -> bool;
}

pub struct UnconfiguredMailer;

#[async_trait]
impl VerificationMailer for UnconfiguredMailer {
    async fn send_verification_code(&self, _recipient: &str, _code: &str) -> Result<(), String> {
        Err("email delivery unavailable".to_string())
    }

    fn configured(&self) -> bool {
        false
    }
}

pub struct SmtpVerificationMailer {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    from: Mailbox,
}

impl SmtpVerificationMailer {
    pub fn from_environment(cloud: bool) -> Result<Option<Self>, String> {
        let server = optional_env("SMTP_SERVER")?;
        let from_address = optional_env("SMTP_FROM")?;
        let (Some(server), Some(from_address)) = (server, from_address) else {
            return Ok(None);
        };
        if server.len() > 253 || from_address.len() > 254 {
            return Err("invalid SMTP configuration".to_string());
        }

        let security = optional_env("SMTP_SECURITY_TYPE")?
            .unwrap_or_else(|| "starttls".to_string())
            .to_ascii_lowercase();
        let default_port = if matches!(security.as_str(), "tls" | "ssl") {
            465
        } else {
            587
        };
        let port = optional_env("SMTP_PORT")?
            .map(|value| {
                value
                    .parse::<u16>()
                    .map_err(|_| "invalid SMTP configuration".to_string())
            })
            .transpose()?
            .unwrap_or(default_port);

        let mut builder = match security.as_str() {
            "starttls" => AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&server),
            "tls" | "ssl" => AsyncSmtpTransport::<Tokio1Executor>::relay(&server),
            "none" if !cloud => Ok(AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(
                &server,
            )),
            _ => return Err("invalid SMTP security configuration".to_string()),
        }
        .map_err(|_| "invalid SMTP configuration".to_string())?
        .port(port);

        let username = optional_env("SMTP_USERNAME")?;
        let password = optional_env("SMTP_PASSWORD")?;
        match (username, password) {
            (Some(username), Some(password)) => {
                builder = builder.credentials(Credentials::new(username, password));
            }
            (None, None) if !cloud => {}
            _ => return Err("invalid SMTP credential configuration".to_string()),
        }

        let from_name =
            optional_env("SMTP_FROM_NAME")?.unwrap_or_else(|| "OneHumanCorp".to_string());
        if from_name.len() > 128 || from_name.chars().any(char::is_control) {
            return Err("invalid SMTP configuration".to_string());
        }
        let from = Mailbox::new(
            Some(from_name),
            from_address
                .parse()
                .map_err(|_| "invalid SMTP configuration".to_string())?,
        );

        Ok(Some(Self {
            transport: builder.build(),
            from,
        }))
    }
}

#[async_trait]
impl VerificationMailer for SmtpVerificationMailer {
    async fn send_verification_code(&self, recipient: &str, code: &str) -> Result<(), String> {
        let recipient: Mailbox = recipient
            .parse()
            .map_err(|_| "email delivery unavailable".to_string())?;
        let message = Message::builder()
            .from(self.from.clone())
            .to(recipient)
            .subject("Verify your OneHumanCorp email")
            .body(format!(
                "Your OneHumanCorp verification code is {code}. It expires in 15 minutes. If you did not request this, ignore this email."
            ))
            .map_err(|_| "email delivery unavailable".to_string())?;
        self.transport.send(message).await.map_err(|_error| {
            tracing::error!(event = "auth.registration.email_delivery_failed");
            "email delivery unavailable".to_string()
        })?;
        Ok(())
    }

    fn configured(&self) -> bool {
        true
    }
}

fn optional_env(name: &str) -> Result<Option<String>, String> {
    match std::env::var(name) {
        Ok(value) if value.trim().is_empty() => Ok(None),
        Ok(value) => Ok(Some(value)),
        Err(std::env::VarError::NotPresent) => Ok(None),
        Err(std::env::VarError::NotUnicode(_)) => Err("invalid SMTP configuration".to_string()),
    }
}

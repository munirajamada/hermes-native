use serde::Deserialize;

use crate::{Error, Result};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct DialGrant {
    pub url: String,
    pub protocols: Vec<String>,
}

impl DialGrant {
    pub fn from_attach_json(text: &str) -> Result<Self> {
        #[derive(Deserialize)]
        struct Attach {
            url: String,
            protocols: Vec<String>,
        }

        let attach: Attach = serde_json::from_str(text)?;
        if attach.protocols.is_empty() {
            return Err(Error::Grant("attach returned no subprotocols".into()));
        }
        Ok(Self {
            url: attach.url,
            protocols: attach.protocols,
        })
    }
}

pub trait GrantProvider: Send + Sync {
    fn grant(&self) -> impl std::future::Future<Output = Result<DialGrant>> + Send;
}

#[derive(Clone, Debug)]
pub struct CliGrantProvider {
    command: String,
}

impl Default for CliGrantProvider {
    fn default() -> Self {
        Self::from_env()
    }
}

impl CliGrantProvider {
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
        }
    }

    pub fn from_env() -> Self {
        Self::new(std::env::var("HERMES_BIN").unwrap_or_else(|_| "hermes".to_string()))
    }
}

impl GrantProvider for CliGrantProvider {
    #[allow(clippy::manual_async_fn)]
    fn grant(&self) -> impl std::future::Future<Output = Result<DialGrant>> + Send {
        async move {
            let output = tokio::process::Command::new(&self.command)
                .args(["gateway", "attach", "--json"])
                .output()
                .await
                .map_err(|error| Error::Grant(format!("cannot run {}: {error}", self.command)))?;
            if !output.status.success() {
                return Err(Error::Grant(format!(
                    "{} gateway attach exited {}",
                    self.command, output.status
                )));
            }
            let text = String::from_utf8(output.stdout)
                .map_err(|error| Error::Grant(format!("non-utf8 attach output: {error}")))?;
            DialGrant::from_attach_json(&text)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_attach_output() {
        let grant = DialGrant::from_attach_json(
            r#"{"url":"ws://127.0.0.1:9/api/ws","protocols":["hermes-gateway-v1","hermes-gateway-ticket.x"]}"#,
        )
        .unwrap();
        assert_eq!(grant.url, "ws://127.0.0.1:9/api/ws");
        assert_eq!(grant.protocols.len(), 2);
    }

    #[test]
    fn rejects_a_grant_without_protocols() {
        assert!(
            DialGrant::from_attach_json(r#"{"url":"ws://127.0.0.1:9/api/ws","protocols":[]}"#)
                .is_err()
        );
    }
}

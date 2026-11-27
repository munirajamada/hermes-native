#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("grant: {0}")]
    Grant(String),
    #[error("websocket: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("rpc error {code}: {message}")]
    Rpc { code: i64, message: String },
    #[error("gateway closed")]
    Closed,
    #[error("timed out")]
    Timeout,
}

impl Error {
    pub(crate) fn bad_subprotocol(error: impl std::fmt::Display) -> Self {
        Self::Grant(format!("invalid subprotocol: {error}"))
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

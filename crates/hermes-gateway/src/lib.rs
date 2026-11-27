//! JSON-RPC over WebSocket transport for a Hermes gateway.

mod client;
mod error;
mod grant;

pub use client::{Event, GatewayClient};
pub use error::{Error, Result};
pub use grant::{CliGrantProvider, DialGrant, GrantProvider};

pub type Json = serde_json::Value;

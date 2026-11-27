use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use serde_json::{Value, json};
use tokio::sync::{broadcast, mpsc, oneshot};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::HeaderValue;

use crate::{DialGrant, Error, GrantProvider, Result};

type Socket =
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;

#[derive(Clone, Debug)]
pub enum Event {
    Ready,
    Notification {
        method: String,
        params: Value,
    },
    ServerRequest {
        id: Value,
        method: String,
        params: Value,
    },
    Closed {
        reason: String,
    },
}

enum Command {
    Request {
        method: String,
        params: Value,
        reply: oneshot::Sender<Result<Value>>,
    },
}

pub struct GatewayClient {
    commands: mpsc::Sender<Command>,
    events: broadcast::Sender<Event>,
    ready: Arc<AtomicBool>,
}

impl GatewayClient {
    pub async fn connect<P>(provider: P) -> Result<Self>
    where
        P: GrantProvider + 'static,
    {
        let (commands, command_rx) = mpsc::channel(64);
        let (events, _) = broadcast::channel(256);
        let ready = Arc::new(AtomicBool::new(false));
        tokio::spawn(run(provider, command_rx, events.clone(), ready.clone()));
        Ok(Self {
            commands,
            events,
            ready,
        })
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.events.subscribe()
    }

    pub async fn request(&self, method: impl Into<String>, params: Value) -> Result<Value> {
        let (reply, answer) = oneshot::channel();
        self.commands
            .send(Command::Request {
                method: method.into(),
                params,
                reply,
            })
            .await
            .map_err(|_| Error::Closed)?;
        answer.await.map_err(|_| Error::Closed)?
    }

    pub async fn wait_ready(&self, timeout: Duration) -> Result<()> {
        let mut events = self.events.subscribe();
        if self.ready.load(Ordering::SeqCst) {
            return Ok(());
        }
        let wait = async {
            loop {
                match events.recv().await {
                    Ok(Event::Ready) => return Ok(()),
                    Ok(_) | Err(broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(broadcast::error::RecvError::Closed) => return Err(Error::Closed),
                }
            }
        };
        tokio::time::timeout(timeout, wait)
            .await
            .map_err(|_| Error::Timeout)?
    }
}

async fn run<P>(
    provider: P,
    mut commands: mpsc::Receiver<Command>,
    events: broadcast::Sender<Event>,
    ready: Arc<AtomicBool>,
) where
    P: GrantProvider + 'static,
{
    let mut backoff = Duration::from_millis(250);
    loop {
        let grant = match provider.grant().await {
            Ok(grant) => {
                backoff = Duration::from_millis(250);
                grant
            }
            Err(error) => {
                eprintln!("gateway grant failed: {error}");
                let _ = events.send(Event::Closed {
                    reason: error.to_string(),
                });
                tokio::time::sleep(backoff).await;
                backoff = (backoff * 2).min(Duration::from_secs(10));
                continue;
            }
        };

        let socket = match connect(&grant).await {
            Ok(socket) => socket,
            Err(error) => {
                eprintln!("gateway grant failed: {error}");
                let _ = events.send(Event::Closed {
                    reason: error.to_string(),
                });
                tokio::time::sleep(backoff).await;
                backoff = (backoff * 2).min(Duration::from_secs(10));
                continue;
            }
        };

        ready.store(false, Ordering::SeqCst);
        let (mut sink, mut stream) = socket.split();
        let mut pending: HashMap<u64, oneshot::Sender<Result<Value>>> = HashMap::new();
        let mut next_id: u64 = 1;

        loop {
            tokio::select! {
                command = commands.recv() => {
                    let Some(Command::Request { method, params, reply }) = command else {
                        return;
                    };
                    let id = next_id;
                    next_id += 1;
                    let frame = json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "method": method,
                        "params": params,
                    });
                    if let Err(error) = sink.send(Message::Text(frame.to_string().into())).await {
                        let _ = reply.send(Err(Error::WebSocket(error)));
                        break;
                    }
                    pending.insert(id, reply);
                }
                message = stream.next() => {
                    match message {
                        Some(Ok(Message::Text(text))) => {
                            handle(text.as_str(), &mut pending, &events, &ready);
                        }
                        Some(Ok(Message::Binary(bytes))) => {
                            if let Ok(text) = std::str::from_utf8(&bytes) {
                                handle(text, &mut pending, &events, &ready);
                            }
                        }
                        Some(Ok(Message::Ping(payload))) => {
                            let _ = sink.send(Message::Pong(payload)).await;
                        }
                        Some(Ok(Message::Close(_))) | None => break,
                        Some(Err(error)) => {
                            let _ = events.send(Event::Closed { reason: error.to_string() });
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }

        for (_, reply) in pending.drain() {
            let _ = reply.send(Err(Error::Closed));
        }
        ready.store(false, Ordering::SeqCst);
        let _ = events.send(Event::Closed {
            reason: "reconnecting".into(),
        });
        tokio::time::sleep(backoff).await;
        backoff = (backoff * 2).min(Duration::from_secs(10));
    }
}

async fn connect(grant: &DialGrant) -> Result<Socket> {
    let mut request = grant.url.as_str().into_client_request()?;
    let protocols = grant.protocols.join(", ");
    let value = HeaderValue::from_str(&protocols).map_err(Error::bad_subprotocol)?;
    request
        .headers_mut()
        .insert("Sec-WebSocket-Protocol", value);
    let (socket, _) = tokio_tungstenite::connect_async(request).await?;
    Ok(socket)
}

fn handle(
    text: &str,
    pending: &mut HashMap<u64, oneshot::Sender<Result<Value>>>,
    events: &broadcast::Sender<Event>,
    ready: &AtomicBool,
) {
    let Ok(value) = serde_json::from_str::<Value>(text) else {
        return;
    };
    let method = value.get("method").and_then(Value::as_str);

    match (value.get("id"), method) {
        (Some(id), None) => {
            if let Some(id) = id.as_u64()
                && let Some(reply) = pending.remove(&id)
            {
                let result = match value.get("error") {
                    Some(error) => Err(Error::Rpc {
                        code: error.get("code").and_then(Value::as_i64).unwrap_or(-32603),
                        message: error
                            .get("message")
                            .and_then(Value::as_str)
                            .unwrap_or("rpc error")
                            .to_string(),
                    }),
                    None => Ok(value.get("result").cloned().unwrap_or(Value::Null)),
                };
                let _ = reply.send(result);
            }
        }
        (Some(id), Some(method)) if method != "event" => {
            let _ = events.send(Event::ServerRequest {
                id: id.clone(),
                method: method.to_string(),
                params: value.get("params").cloned().unwrap_or(Value::Null),
            });
        }
        (None, Some("event")) => {
            let event = value.get("params");
            let kind = event
                .and_then(|params| params.get("type"))
                .and_then(Value::as_str)
                .unwrap_or_default();
            let payload = event
                .and_then(|params| params.get("payload"))
                .cloned()
                .unwrap_or(Value::Null);
            if kind == "gateway.ready" {
                ready.store(true, Ordering::SeqCst);
                let _ = events.send(Event::Ready);
            }
            let _ = events.send(Event::Notification {
                method: kind.to_string(),
                params: payload,
            });
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::result_large_err)]

    use super::*;
    use tokio::net::TcpListener;
    use tokio::time::sleep;
    use tokio_tungstenite::tungstenite::handshake::server::{ErrorResponse, Request, Response};

    struct StaticGrant(String);

    impl GrantProvider for StaticGrant {
        fn grant(&self) -> impl std::future::Future<Output = Result<DialGrant>> + Send {
            let url = self.0.clone();
            async move {
                Ok(DialGrant {
                    url,
                    protocols: vec![
                        "hermes-gateway-v1".to_string(),
                        "hermes-gateway-ticket.test".to_string(),
                    ],
                })
            }
        }
    }

    async fn ready_socket(
        stream: tokio::net::TcpStream,
    ) -> tokio_tungstenite::WebSocketStream<tokio::net::TcpStream> {
        let mut socket = tokio_tungstenite::accept_hdr_async(
            stream,
            |request: &Request, mut response: Response| {
                let offered = request
                    .headers()
                    .get_all("Sec-WebSocket-Protocol")
                    .iter()
                    .filter_map(|value| value.to_str().ok())
                    .collect::<Vec<_>>()
                    .join(", ");
                assert!(
                    offered.contains("hermes-gateway-ticket.test"),
                    "ticket subprotocol missing: {offered}"
                );
                response.headers_mut().insert(
                    "Sec-WebSocket-Protocol",
                    HeaderValue::from_static("hermes-gateway-v1"),
                );
                Ok::<_, ErrorResponse>(response)
            },
        )
        .await
        .unwrap();
        socket
            .send(Message::Text(
                json!({"jsonrpc":"2.0","method":"event","params":{"type":"gateway.ready","payload":{}}})
                    .to_string()
                    .into(),
            ))
            .await
            .unwrap();
        socket
    }

    #[tokio::test]
    async fn attaches_and_round_trips() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            let mut socket = ready_socket(stream).await;
            while let Some(Ok(Message::Text(text))) = socket.next().await {
                let value: Value = serde_json::from_str(text.as_str()).unwrap();
                let reply = json!({
                    "jsonrpc": "2.0",
                    "id": value.get("id").cloned(),
                    "result": {"sessions": [{"id": "a"}, {"id": "b"}]},
                });
                socket
                    .send(Message::Text(reply.to_string().into()))
                    .await
                    .unwrap();
            }
        });

        let client = GatewayClient::connect(StaticGrant(format!("ws://{address}")))
            .await
            .unwrap();
        client.wait_ready(Duration::from_secs(3)).await.unwrap();
        let result = client.request("session.list", json!({})).await.unwrap();
        assert_eq!(result["sessions"].as_array().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn reconnects_after_a_close() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            let mut first = ready_socket(stream).await;
            first.close(None).await.unwrap();

            let (stream, _) = listener.accept().await.unwrap();
            let mut second = ready_socket(stream).await;
            while let Some(Ok(Message::Text(text))) = second.next().await {
                let value: Value = serde_json::from_str(text.as_str()).unwrap();
                let reply = json!({
                    "jsonrpc": "2.0",
                    "id": value.get("id").cloned(),
                    "result": {"sessions": []},
                });
                second
                    .send(Message::Text(reply.to_string().into()))
                    .await
                    .unwrap();
            }
        });

        let client = GatewayClient::connect(StaticGrant(format!("ws://{address}")))
            .await
            .unwrap();
        let mut events = client.subscribe();
        client.wait_ready(Duration::from_secs(3)).await.unwrap();

        let mut closed = false;
        while !closed {
            match tokio::time::timeout(Duration::from_secs(3), events.recv()).await {
                Ok(Ok(Event::Closed { .. })) => closed = true,
                Ok(Ok(_)) | Ok(Err(broadcast::error::RecvError::Lagged(_))) => continue,
                _ => break,
            }
        }
        assert!(closed, "the client did not observe the disconnect");

        let mut ready_again = false;
        while !ready_again {
            match tokio::time::timeout(Duration::from_secs(5), events.recv()).await {
                Ok(Ok(Event::Ready)) => ready_again = true,
                Ok(Ok(_)) | Ok(Err(broadcast::error::RecvError::Lagged(_))) => continue,
                _ => break,
            }
        }
        assert!(ready_again, "the client did not reconnect");

        let result = client.request("session.list", json!({})).await.unwrap();
        assert_eq!(result["sessions"].as_array().unwrap().len(), 0);
    }

    #[tokio::test]
    async fn correlates_out_of_order() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            let mut socket = ready_socket(stream).await;
            let mut requests = Vec::new();
            while requests.len() < 2 {
                if let Some(Ok(Message::Text(text))) = socket.next().await {
                    requests.push(serde_json::from_str::<Value>(text.as_str()).unwrap());
                }
            }
            for request in requests.iter().rev() {
                let reply = json!({
                    "jsonrpc": "2.0",
                    "id": request.get("id").cloned(),
                    "result": {"method": request.get("method").cloned()},
                });
                socket
                    .send(Message::Text(reply.to_string().into()))
                    .await
                    .unwrap();
                sleep(Duration::from_millis(20)).await;
            }
            while socket.next().await.is_some() {}
        });

        let client = GatewayClient::connect(StaticGrant(format!("ws://{address}")))
            .await
            .unwrap();
        client.wait_ready(Duration::from_secs(3)).await.unwrap();
        let (first, second) = tokio::join!(
            client.request("first", json!({})),
            client.request("second", json!({})),
        );
        assert_eq!(first.unwrap()["method"], "first");
        assert_eq!(second.unwrap()["method"], "second");
    }
}

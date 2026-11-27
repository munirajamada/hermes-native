# Use tokio and tokio-tungstenite for the gateway transport

The gateway client uses tokio for its runtime and `tokio-tungstenite` for the WebSocket. The protocol needs long-lived sockets, concurrent in-flight requests, bounded reconnect backoff, and WebSocket subprotocols for the ticket; the UI stays synchronous GPUI.

**Considered options:** GPUI's HTTP client (no WebSocket), async-std (smaller ecosystem, no advantage here).

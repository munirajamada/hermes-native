## Purpose

Defines how the client obtains a dial grant, connects to a Hermes gateway, and exchanges JSON-RPC requests, responses, and events.

## ADDED Requirements

### Requirement: Local dial grant

The client SHALL obtain the WebSocket URL, the single-use ticket, and the subprotocols from `hermes gateway attach --json`, and SHALL dial with those subprotocols.

#### Scenario: Ticket is required

- **WHEN** the client dials without the ticket subprotocols
- **THEN** the gateway rejects the socket
- **AND** the client reports an authentication error instead of retrying forever

#### Scenario: Successful attach

- **WHEN** a local gateway is ready
- **THEN** the client connects and receives `gateway.ready`

### Requirement: Request correlation

The client SHALL match each JSON-RPC response to its request by id, and SHALL deliver notifications and server requests on separate paths.

#### Scenario: Concurrent requests

- **WHEN** two requests are in flight
- **THEN** each response resolves its own caller
- **AND** an unsolicited notification is not delivered as a response

### Requirement: Reconnect

The client SHALL reconnect after a dropped socket with bounded backoff and SHALL not deliver a completed turn's events twice.

#### Scenario: Gateway restart

- **WHEN** the socket closes and the gateway returns
- **THEN** the client reconnects and re-subscribes without replaying delivered events

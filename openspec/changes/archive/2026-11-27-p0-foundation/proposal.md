# P0 Foundation

## Why

The stack is verified but there is no code. P0 proves the two dependencies every later phase rests on — generated contract types and a real gateway connection — before any UI work is built on guesses.

## What Changes

- Add a generator that reads the vendored OpenRPC document and writes
  `hermes-contract/src/generated.rs`; CI regenerates and diffs it.
- Add `hermes-gateway`: obtain a local dial grant, connect with the ticket
  subprotocols, correlate JSON-RPC responses, and fan out events.
- Add the first `hermes-ui` window on X11/XWayland, with the product identity
  and the GPUI stack initialized.
- Wire `hermes-native` to open the window and complete one `session.list`
  round-trip.

## Capabilities

### New Capabilities

- `contract-generation`: deterministic Rust types and a method inventory from
  the gateway OpenRPC document, with a drift check.
- `gateway-connection`: attach, dial, exchange JSON-RPC, and surface events.
- `app-shell`: a single native window with the product identity.

### Modified Capabilities

None.

## Impact

- `hermes-contract` gains a generated module; `hermes-gateway` gains the
  transport; `hermes-ui` gains its first GPUI dependency (`gpui-kit` with
  `gpui-fast`).
- CI gains a regenerate-and-diff check and software rendering for UI tests.
- Depends on `hermes gateway attach --json` for the local dial.

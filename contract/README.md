# Gateway contract

`gateway-contract.openrpc.json` is a vendored copy of the Hermes gateway's
OpenRPC document. It is the input to the `hermes-contract` generator.

- Upstream: `NousResearch/hermes-agent`, commit `46c47112` (2026-11-26)
- Source path: `apps/shared/src/gateway-contract.openrpc.json`
- Contents: 265 methods, 79 notifications, 16 server requests, 915 schemas

Refresh from the reference clone:

```sh
cp ~/Desktop/Hyntix/agents/_reference/hermes-agent/apps/shared/src/gateway-contract.openrpc.json \
   contract/gateway-contract.openrpc.json
```

The Rust generator reads this file directly. Python and the upstream generator
are not part of the build; the vendored JSON is the boundary.

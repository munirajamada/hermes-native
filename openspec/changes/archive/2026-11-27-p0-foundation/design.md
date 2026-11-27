# Design

## Context

See `proposal.md` for motivation. The stack is verified in `docs/VERIFICATION.md`; the contract is vendored at `contract/gateway-contract.openrpc.json` (upstream `46c47112`); `hermes gateway attach --json` is the verified way to obtain a dial grant.

## Goals / Non-Goals

- Goal: generated types, a connected client, and one window.
- Non-goal: chat, panes, settings, persistence, packaging.

## Decisions

- **gpui-kit with `gpui-fast`.** See `docs/adr/0001-use-gpui-kit-with-gpui-fast.md`.
- **Standalone generator, committed output, CI diff.** See
  `docs/adr/0004-generate-contract-types-from-openrpc.md`.
- **tokio + tokio-tungstenite.** See `docs/adr/0005-use-tokio-and-tungstenite.md`.
- **Shell out to `hermes gateway attach --json` in P0.** See
  `docs/adr/0003-connect-to-an-existing-gateway.md`; the native control socket
  is a later refinement.
- **XDG paths, keyring later.** See
  `docs/adr/0006-store-state-under-xdg-and-secrets-in-the-keyring.md`.

## Risks / Trade-offs

- Contract drift → CI regenerates and diffs the committed file.
- The attach CLI is a subprocess dependency → keep it behind one trait so the
  native control socket can replace it.
- No GPU in CI → software Vulkan; see
  `docs/adr/0007-test-ui-with-headless-software-rendering.md`.
- The first GPUI dependency is a large build → cache it in CI from the start.

## Migration Plan

None: no released behaviour exists.

## Open Questions

None that change the specs or the task order.

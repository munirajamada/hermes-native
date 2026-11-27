# HermesNative

A native desktop client for [Hermes](https://github.com/NousResearch/hermes-agent),
written in Rust with [gpui-kit](https://gpui-kit.com). Linux first.

> **Status: pre-alpha scaffold.** The workspace, documentation, and tooling
> exist. There is no UI yet. The Python agent is unmodified and runs
> separately (`hermes serve`); this project is the desktop shell only.
> The core stack is verified end to end — see
> [`docs/VERIFICATION.md`](docs/VERIFICATION.md).

HermesNative is an independent client. Hermes is a project of Nous Research.

## Why

The Electron desktop app carries Chromium's memory and startup cost. This
client keeps the same agent and the same gateway protocol, in a native shell.

## Requirements

- Linux on X11 or XWayland (native Wayland is out of scope for v1; on a
  Wayland session the webview needs `GDK_BACKEND=x11`)
- Rust 1.100+ (edition 2024) and cargo
- A running Hermes gateway, local or remote
- For the preview pane: WebKitGTK 4.1 and GTK3 development headers
  (verified with WebKitGTK 2.54.3 / GTK 3.24.52)

## Build

```sh
scripts/build.sh      # cargo build --workspace --locked
scripts/test.sh       # cargo test --workspace --locked
scripts/ui-test.sh    # tests under Xvfb (or the current display)
scripts/check.sh      # the gate: fmt, clippy, docs, tests
```

## Layout

| Crate | Responsibility |
|---|---|
| `hermes-contract` | Generated gateway contract types (OpenRPC) |
| `hermes-gateway` | JSON-RPC/WebSocket transport for `hermes serve` |
| `hermes-host` | Backend resolution, profiles, connections, secrets |
| `hermes-platform` | Tray, notifications, deep links, hotkeys |
| `hermes-git` | Git status, diff, worktrees |
| `hermes-terminal` | PTY and VT terminal |
| `hermes-preview` | `PreviewEngine` and the embedded webview |
| `hermes-update` | Update checks and handoff |
| `hermes-ui` | gpui-kit UI: shell, panes, chat, pages |
| `hermes-native` | The binary |

The vendored gateway contract is in [`contract/`](contract/README.md).

## Documentation

- [`docs/PRD.md`](docs/PRD.md) — requirements and acceptance
- [`docs/PLAN.md`](docs/PLAN.md) — architecture, crates, phases
- [`docs/VERIFICATION.md`](docs/VERIFICATION.md) — stack assumptions, tested
- [`docs/adr/`](docs/adr/README.md) — architecture decisions
- [`docs/TOOLING.md`](docs/TOOLING.md) — the gate and supply chain
- [`AGENTS.md`](AGENTS.md) — instructions for coding agents

## Contributing

See [`CONTRIBUTING.md`](CONTRIBUTING.md). By participating you agree to the
[`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md). Report vulnerabilities per
[`SECURITY.md`](SECURITY.md).

## Licence

MIT — see [`LICENSE`](LICENSE). Third-party components are listed in
[`THIRD-PARTY.md`](THIRD-PARTY.md).

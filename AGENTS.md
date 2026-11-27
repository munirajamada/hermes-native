# HermesNative — agent instructions

Native desktop client for Hermes: Rust + [gpui-kit](https://gpui-kit.com),
Linux first. The Python agent (`hermes serve`) is unmodified; this repository
replaces the Electron shell and React renderer.

## Start here

- `docs/PRD.md` — requirements, acceptance, scope. Canonical for *what*.
- `docs/PLAN.md` — architecture, crates, phases. Canonical for *how*.
- `docs/VERIFICATION.md` — stack assumptions, tested end to end.
- `docs/TOOLING.md` — the gate and supply chain.

## Commands

```sh
scripts/build.sh        # cargo build --workspace --locked
scripts/lint.sh         # fmt check + clippy -D warnings
scripts/test.sh         # cargo test --workspace --locked
scripts/check.sh        # the gate: lint, then tests
```

To talk to a local gateway:

```sh
hermes gateway attach --json   # url + 30s ticket + subprotocols; dial with them
```

Run `scripts/check.sh` before claiming work is complete, and quote its raw
output. A compressed summary is not evidence.

## Layout

| Crate | Responsibility |
|---|---|
| `hermes-contract` | Generated gateway contract types (OpenRPC) |
| `hermes-gateway` | JSON-RPC/WebSocket transport |
| `hermes-host` | Backend resolution, profiles, connections, secrets |
| `hermes-platform` | Tray, notifications, deep links, hotkeys |
| `hermes-git` | Git status, diff, worktrees |
| `hermes-terminal` | PTY and VT terminal |
| `hermes-preview` | `PreviewEngine` and the embedded webview |
| `hermes-update` | Update checks and handoff |
| `hermes-ui` | gpui-kit UI |
| `hermes-native` | The binary |

## Invariants

- The Hermes agent is never modified; talk to it through the gateway contract.
- Contract types are generated, never hand-written.
- crates.io only; no git dependencies. No wildcard versions.
- Pinned stack: `gpui-kit` / `gpui-webview` / `gpui-component` 0.11.0,
  `gpui-fast` 0.1.19. Versions must agree.
- Linux target is X11/XWayland. Native Wayland is out of scope for v1.
- On a Wayland session, set `GDK_BACKEND=x11` and start with
  `gpui_kit::platform::linux(WindowingModes::X11)`. Without it, `gtk::init()`
  fails with "Failed to initialize GTK" and the webview never builds (verified
  on sway/XWayland).
- Ponytail is active: the smallest complete change; no speculative features.

## Working notes

- `.planning/` is git-ignored working state (`/pwf`); `openspec/` is the
  tracked record of decisions, and `docs/adr/` records the architectural ones.
- `.codegraph/` is a git-ignored index (`codegraph explore "..."`).

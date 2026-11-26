# Third-party components

This project is MIT licensed. It uses and links the following, and their
licences apply to them.

## Build and runtime dependencies

| Component | Licence | Notes |
|---|---|---|
| Rust standard library | MIT OR Apache-2.0 | |
| `gpui-kit`, `gpui-component` | Apache-2.0 | UI framework and components |
| `gpui-webview` (via `lb-wry`) | Apache-2.0 / MIT | embedded webview wrapper |
| `wry` | Apache-2.0 / MIT | webview bindings |
| `serde`, `serde_json` | MIT OR Apache-2.0 | serialization |
| `tokio` | MIT | async runtime |
| `portable-pty` | MIT | terminal PTY |
| `git2` or `gix` | MIT OR Apache-2.0 | git operations |

The exact tree is whatever `Cargo.lock` resolves; `cargo deny check` enforces
the licence allowlist in `deny.toml`.

## System libraries

| Component | Licence | Notes |
|---|---|---|
| WebKitGTK | LGPL-2.1-or-later | system library, dynamically linked; provides the embedded browser on Linux |
| GTK 3 | LGPL-2.1-or-later | system library, dynamically linked |
| glibc | LGPL-2.1-or-later | system C library |

These are not vendored and are not covered by this project's MIT licence.

## Related projects

- [Hermes](https://github.com/NousResearch/hermes-agent) — MIT. The agent this
  client talks to, and the Electron/React implementation whose behaviour this
  client aims to match. No source is copied from it into this project.
- [GPUI](https://github.com/zed-industries/zed) — Apache-2.0. The UI framework
  underneath `gpui-kit`.

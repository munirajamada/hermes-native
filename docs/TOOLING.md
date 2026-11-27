# Tooling

## System requirements

- X11 or XWayland at runtime. On a Wayland session the webview needs
  `GDK_BACKEND=x11`; without it GTK initialization fails.
- WebKitGTK 4.1 (`webkit2gtk-4.1`) and GTK 3 (`gtk+-3.0`) development headers
  for `gpui-webview`.
- Disk: a gpui / gpui-fast build is roughly 6.6 GB of `target/`. Keep it on a
  real filesystem; `/tmp` is a 6.8 GB tmpfs here and the linker dies.
- Headless UI tests need a display: `xvfb-run` (`xorg-server-xvfb`), a live
  X11/XWayland session, or `sway` with its headless backend.
  `scripts/ui-test.sh` picks the first available.

## The gate

| Script | Job |
|---|---|
| `scripts/build.sh` | `cargo build --workspace --locked` |
| `scripts/lint.sh` | `cargo fmt --all --check` + `cargo clippy --workspace --all-targets -- -D warnings`; `--fix` formats |
| `scripts/test.sh` | `cargo test --workspace --locked` |
| `scripts/ui-test.sh` | tests under Xvfb, the current display, or headless sway |
| `scripts/check-docs.sh` | required docs exist and relative links resolve |
| `scripts/check.sh` | lint, docs, then tests |
| `scripts/install-hooks.sh` | sets `core.hooksPath` to `.githooks` |

`.githooks/pre-push` runs `scripts/check.sh`; `SKIP_GATE=1` is the recorded
bypass.

## Branch protection

`main` requires the six CI checks (Lint, Test, Build (release), Supply chain,
Specs, Docs) and a pull request, requires the branch to be up to date, and
blocks force-push and deletion. The rules apply to admins too, so the owner
cannot bypass them; changes land through a pull request.

Verification honesty: the gate runs the real tool and prints its output
verbatim. A compressed summary is not evidence; a suite that does not exist is
not reported as passing.

## Supply chain

`deny.toml` is the policy: advisories, a licence allowlist, no wildcard
versions, crates.io only. Run it with `cargo deny check` (install
`cargo-deny` first). WebKitGTK is LGPL-2.1+ and is a system library, so it is
recorded here rather than seen by `cargo deny`.
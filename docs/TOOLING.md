# Tooling

## The gate

| Script | Job |
|---|---|
| `scripts/build.sh` | `cargo build --workspace --locked` |
| `scripts/lint.sh` | `cargo fmt --all --check` + `cargo clippy --workspace --all-targets -- -D warnings`; `--fix` formats |
| `scripts/test.sh` | `cargo test --workspace --locked` |
| `scripts/check.sh` | lint, then tests |
| `scripts/install-hooks.sh` | sets `core.hooksPath` to `.githooks` |

`.githooks/pre-push` runs `scripts/check.sh`; `SKIP_GATE=1` is the recorded
bypass.

Verification honesty: the gate runs the real tool and prints its output
verbatim. A compressed summary is not evidence; a suite that does not exist is
not reported as passing.

## Supply chain

`deny.toml` is the policy: advisories, a licence allowlist, no wildcard
versions, crates.io only. Run it with `cargo deny check` (install
`cargo-deny` first). WebKitGTK is LGPL-2.1+ and is a system library, so it is
recorded here rather than seen by `cargo deny`.
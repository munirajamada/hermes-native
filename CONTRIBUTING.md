# Contributing

Thanks for helping. This project is pre-alpha: interfaces and crate
boundaries will move, and a small, focused change is easier to land than a
large one.

## Before you start

- Open an issue for anything larger than a fix.
- For a change that needs design agreement, propose it as an OpenSpec change
  (`openspec/`), then implement it. A decision worth keeping becomes an ADR in
  `docs/`.
- Keep one concern per pull request.

## Setup

```sh
git clone https://github.com/munirajamada/hermes-native.git
cd hermes-native
scripts/install-hooks.sh   # core.hooksPath = .githooks
```

You need Rust 1.100+ and cargo. The preview pane needs WebKitGTK 4.1 and GTK3
development headers, and runs on X11/XWayland; on a Wayland session start with
`GDK_BACKEND=x11`. Nothing else has platform dependencies yet.

To exercise the real backend:

```sh
hermes gateway attach --json   # URL, 30s ticket, subprotocols to dial with
```

`docs/VERIFICATION.md` records what has been tested end to end.

## The gate

Run it before opening a pull request:

```sh
scripts/check.sh
```

It runs `cargo fmt --check`, `cargo clippy -D warnings`, the documentation
check, and the workspace tests. `scripts/ui-test.sh` runs the tests under a
display (Xvfb or the current session) so window tests work. Quote the raw
output in your pull request; a summary is not evidence.
`cargo deny check` enforces the dependency policy when `cargo-deny` is
installed.

## Style

- `cargo fmt` and `cargo clippy` decide formatting and lints.
- Conventional Commits: `feat:`, `fix:`, `docs:`, `chore:`, `refactor:`,
  `test:`.
- Dependencies: crates.io only, no wildcard versions, permissive licences
  only. See `deny.toml`.
- No secrets, tokens, or personal paths in code, tests, fixtures, or logs.

## Pull requests

- Small, one concern, with a test for new behaviour.
- The template's checklist must be satisfied.
- A maintainer reviews; the gate must be green.

## Licence

By contributing you agree that your contribution is licensed under the
project's MIT licence, and that you have the right to submit it.

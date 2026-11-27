# Changelog

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Cargo workspace with the planned crate layout (contract, gateway, host,
  platform, git, terminal, preview, update, ui) and the `hermes-native` binary.
- Product requirements (`docs/PRD.md`), delivery plan (`docs/PLAN.md`), and
  tooling notes (`docs/TOOLING.md`).
- The gate: `scripts/{build,lint,test,check}.sh`, a pre-push hook, and
  `deny.toml`.
- Open-source project files: licence, contributing guide, code of conduct,
  security policy, issue and pull-request templates.
- Pinned toolchain (`rust-toolchain.toml`, `rust-version = "1.100"`).
- Six-job CI: lint, test under Xvfb, release build, supply chain, specs, docs.
- `scripts/check-docs.sh` (required files and relative links) and
  `scripts/ui-test.sh` (tests under a display).

### Verified

- End-to-end stack verification: webview on XWayland, composition overlays
  above the page, layer-shell on native Wayland, and a gateway `session.list`
  round-trip. See `docs/VERIFICATION.md`.

## [0.1.0] - unreleased

No release has been cut. A tag is a decision, not a pipeline test.

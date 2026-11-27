# HermesNative — Final Plan

A native desktop client for Hermes, written in Rust, using **gpui-kit** for UI.
Full feature parity with the Electron-based Hermes Desktop, Linux first.

Status: planning. No implementation started.

---

## 1. Scope and boundaries

The Electron app has three boundaries. We replace two.

| Boundary | Technology | Decision |
|---|---|---|
| Hermes Agent | Python, `hermes serve`, JSON-RPC over WebSocket (`tui_gateway`) | **Stays as-is.** Untouched. |
| Electron main | Node: backend lifecycle, native capabilities | **Rewrite in Rust.** |
| React renderer | React + nanostores | **Rewrite in gpui-kit.** |

Reference implementation: `~/.hermes/hermes-agent/apps/desktop` (3,798 files in `src/`).
Backend API surface: 265 methods, 79 notifications, 16 server→client requests, 915 schemas.

**Not in scope:** the agent, its skills/memory/tools, the Python package, the web dashboard.

---

## 2. Locked decisions

| Decision | Choice |
|---|---|
| UI framework | `gpui-kit` **0.11.0** (+ `gpui-component` 0.11.0, `gpui-fast` 0.1.19) |
| In-client browser | **`gpui-webview` 0.11.0** (wry → WebKitGTK on Linux) |
| Webview display server | **X11 / XWayland** (`gpui_kit::platform::linux(WindowingModes::X11)`) |
| Webview runtime on Wayland | **`GDK_BACKEND=x11`**, or GTK init fails |
| Overlays above webview | `gpui-fast` feature on **both** `gpui-kit` and `gpui-webview` (versions must match) |
| Backend acquisition | Connect to an existing local `hermes serve` or a saved remote/cloud gateway. No bundled Python. |
| Contract types | Generated from the backend's OpenRPC contract, not hand-written |
| Parity target | Full parity including extras, delivered in phases |
| First platform | Linux |
| Obscura | **Not** the in-app browser. Agent-side headless browser / capture only. |
| Tooling | HyntixCode's `docs/TOOLING.md` conventions: `scripts/` gate, six-job CI, `deny.toml`, `--locked`, hook-enforced gate |

### Why wry/WebKitGTK and not the alternatives

- **Obscura** is headless (no widget, experimental renderer). It would mean screencast frames + synthetic input — a video pretending to be a browser. Fine for the agent, wrong for the preview pane.
- **CEF OSR** is full Chromium into a texture, but adds 200 MB–1 GB, multi-process plumbing, and Wayland pain. Longbridge (authors of gpui-component/gpui-kit) tried it and shipped wry instead.
- **Servo** is the only pure-Rust in-window engine; unstable on JS-heavy pages. Deferred, kept possible behind the engine trait.
- **WebKitGTK is C.** This is the "as much Rust as possible" tradeoff, accepted.

### Known platform limits (from upstream docs)

- Linux **Wayland** is not supported by the webview wrapper; ship on XWayland. (The underlying `gpui-fast` composition already has `wayland.rs`; the wrapper still requires X11.)
- Windows WebView2 covers GPUI overlays; composition is not ready there. Linux/macOS work with `gpui-fast`.
- Without composition, a native view extends past a clipping container.
- One `Entity<WebView>` per view; never rebuild per render; drop handles before the window.

---

## 3. Workspace layout

One Cargo workspace, one concern per crate.

```
hermes-contract    Generated types + method/event/server-request enums (from OpenRPC)
hermes-gateway     WS JSON-RPC + HTTP, reconnect/backoff, event replay, server-request dispatch
hermes-host        Backend resolve/probe/spawn (or remote), profiles + connections, keyring tokens
hermes-platform    Tray, notifications, deep links, global hotkeys, clipboard, keep-awake, window state
hermes-git         status/diff/worktree/review (git2 or gix)
hermes-terminal    portable-pty + VT emulator + gpui cell renderer
hermes-preview     PreviewEngine trait: Wry (default), ExternalWindow (fallback), Obscura (capture)
hermes-update      Updater / checkout handoff
hermes-ui          Shell, sidebar, workspace dock, right rail, chat, composer, pages, overlays
hermes-native      Binary
```

State model: gpui `Entity` / `Global` stores fed by a single gateway event stream.
Mirror the React split — durable stores vs transport adapters — but do not port React structure 1:1.

---

## 4. Gateway and contract layer

This is the highest-leverage part. Do it first and do it properly.

- **Transport:** WebSocket JSON-RPC, plus HTTP for file/image/artifact endpoints.
- **Bidirectional:** handle the 16 server→client requests (`approval`, `clarify`, `preview.act`, `preview.read`, `display.install.sudo`, …). These are not notifications; they need responses.
- **Replay and reconnect:** mirror `session.events.since` + replay semantics; exponential backoff; dedupe by event id.
- **Codegen:** a Rust emitter reads the vendored `contract/gateway-contract.openrpc.json` and writes `hermes-contract/src/generated.rs`. Port the supported JSON-Schema subset (objects, enums, const, anyOf-null, arrays, `$ref`, oneOf+discriminator) from the upstream Python generator; do not run Python in the build.
- **Sync check:** CI regenerates and diffs the Rust output, exactly like the TypeScript lane does.
- **Auth:** token and OAuth modes; store secrets in the OS keyring (equivalent of Electron `safeStorage`).
- **Local dial (verified):** `hermes gateway attach --json` returns the WebSocket URL, a 30-second single-use ticket, and the subprotocols; dial with those or the handshake is `403`. Confirmed: `gateway.ready`, then `session.list`.
- **Profiles/connections:** multiple gateways, soft switch (wipe gateway-bound stores, invalidate queries, repopulate) without cold boot or cancelling turns.

---

## 5. UI architecture

Built on gpui-kit. Most React components have a direct counterpart; use it.

| Surface | gpui-component |
|---|---|
| Chat transcript | `message`, `bubble`, `message_scroller`, `virtual_list` |
| Markdown + code | `text` (MarkdownPlugin, tree-sitter/syntect code blocks) |
| Composer | `input/editor`, `textarea`, completions, history |
| Shell | `dock`, `sidebar`, `resizable`, `title_bar`, `status_bar` |
| Right rail | `tree`, `list`, `diff`, `tab` |
| Palette / pickers | `command`, `searchable_list`, `combobox`, `select` |
| Settings | `setting` (page/group/item) |
| Charts | `chart`, `plot` |
| Voice UI | `speech` |
| Diff/review | `diff` (parser + presentation + review) |

Gaps to build: **terminal**, **preview webview**, **markdown math (KaTeX)**.

Theme: port the token set from `src/styles.css` (`--stroke-nous`, `shadow-nous`, etc.) into a gpui theme so one place controls the look.

---

## 6. Preview / browser design

`hermes-preview` wraps `gpui-webview`:

```
PreviewEngine (trait)
├── Wry            default; WebKitGTK; native child + gpui-fast composition
├── ExternalWindow fallback; own OS window if composition fails
└── Obscura        capture / non-interactive only
```

- `WebView::build()` performs the GTK + wry platform setup.
- Call `set_bounds` on prepaint; X11 does not auto-resize.
- With `gpui-fast`: popovers/dialogs render above the page; view clips to its container.
- Persist cookies/storage per profile for logins.
- OAuth/login flows use the real webview, never Obscura.

**Verified on this machine** (sway session, XWayland, WebKitGTK 2.54.3,
GTK 3.24.52, 2026-11-27): `gpui-webview` 0.11.0 builds and renders
`example.com` only with `GDK_BACKEND=x11`; without it `gtk::init()` fails with
"Failed to initialize GTK". Native Wayland is impossible for this pane because
GTK cannot embed into a Wayland surface it does not own.

---

## 7. Parity checklist

Tick items as they land. This is the definition of "full parity".

### A. Shell and navigation
- [ ] Chat home (new chat, `/sessionId` routes)
- [ ] Sidebar: session tree, profile/connection groups, projects, search/filter
- [ ] Workspace dock: tabs, splits, pane tree, drag
- [ ] Right rail: preview, files, review, terminal
- [ ] Titlebar, window controls, multi-window, tray, minimize-to-tray
- [ ] Command palette and Command Center
- [ ] Context menus, full keyboard shortcut map
- [ ] Toasts, notifications, approval/clarify card stack
- [ ] Sessions: create/open/rename/archive/delete/branch/resume/interrupt/steer/undo/compress, cwd
- [ ] Profiles and connections (local/remote/cloud), soft switch
- [ ] Projects: folders, repos, worktrees
- [ ] Model picker, reasoning effort, tools configuration
- [ ] i18n, themes/skins, accessibility, zoom

### B. Chat
- [ ] Streaming markdown, reasoning, tool calls and tool output
- [ ] Approvals and clarify (server→client requests)
- [ ] Attachments and images (paste, drag-drop, attach bytes)
- [ ] Slash commands, `@path` completion, prompt history
- [ ] Subagents (`subagent.list/roster/tail/steer/interrupt`)
- [ ] `/background` and `/btw` side agents
- [ ] Artifacts, usage/cost, context breakdown, TODO
- [ ] Handoff, rollback, undo, compress
- [ ] Voice: `voice.record`, `voice.tts` + speech UI

### C. Panes
- [ ] Files: tree, text/image/PDF preview, file watch
- [ ] Review: git diff, per-file view, worktrees
- [ ] Terminal: PTY, tabs, resize, links, session persist
- [ ] Preview: webview navigation, capture, embeds, sign-ins

### D. Pages and overlays
- [ ] Settings: providers/models, tools, credentials, connections, appearance, keyboard, billing, memory, delegation, updates, about, advanced
- [ ] Capabilities: connectors, MCP, plugins, skills, toolsets, catalog
- [ ] Profiles
- [ ] Session import (foreign)
- [ ] Artifacts
- [ ] Cron
- [ ] Agents
- [ ] Starmap
- [ ] Messaging
- [ ] Webhooks
- [ ] Command Center
- [ ] Updates overlay

### E. Native extras
- [ ] Voice capture/playback
- [ ] Pet overlay + pet generate
- [ ] HUD (floating, snap, games, Hyprland)
- [ ] Wake indicator
- [ ] Quick entry
- [ ] Session windows
- [ ] Deep links
- [ ] Global screenshot shortcut (Linux equivalent)
- [ ] SSH / remote / cloud
- [ ] Updater / checkout handoff
- [ ] Tray, notifications, keep-awake, power handling
- [ ] Spellcheck, drag-drop, clipboard images, F12/devtools
- [ ] Crash and log diagnostics

---

## 8. Phases

Each phase ends with a demoable build and explicit exit criteria.

### P0 — Foundation
Workspace, pinned versions, app boots one themed window, contract codegen (Rust), gateway client skeleton, logging and panic handling.
**Exit:** generated types compile; connect/disconnect to a running gateway works; window renders.

### P1 — Chat core
Session list and open; `prompt.submit`; streaming transcript; tool events; **approval + clarify**; basic composer (text, images, attachments, slash, `@path`); model picker.
**Exit:** a real conversation with tool calls and an approval, end to end.

### P2 — Shell
Sidebar (sessions, groups, projects), workspace dock, right-rail shell, command palette, shortcuts, multi-window, tray, notifications, profile/connection switching.
**Exit:** navigate and switch profiles without losing turns or leaking state.

### P3 — Panes
Files, review/diff, terminal, preview (webview).
**Exit:** all four panes usable in X11/XWayland with overlays above the webview.

### P4 — Pages
Settings (all sections), Capabilities, Profiles, Session import, Artifacts, Cron, Agents, Starmap, Messaging, Webhooks, Command Center, Updates.
**Exit:** every route and overlay in `routes.ts` is reachable and functional.

### P5 — Extras and hardening
Voice, pet, HUD, wake indicator, quick entry, session windows, deep links, screenshot shortcut, SSH/cloud, updater, i18n, accessibility pass, spellcheck, drag-drop, zoom, diagnostics.
**Exit:** parity checklist complete on Linux; known exceptions documented.

Sizing is deliberately relative. P1 and P3 carry the technical risk; P4 is broad but low-risk; P5 is long-tail.

---

## 9. Risks and mitigations

| Risk | Mitigation |
|---|---|
| Webview wrapper requires X11; Wayland users | Ship XWayland; track upstream composition Wayland support |
| `gpui-fast` / `gpui-webview` version mismatch | Pin all three to 0.11.0; CI check |
| Contract drift across 265 methods | Generated types + regenerate-and-diff CI |
| GPUI weaker than Chromium on a11y/IME | Dedicated P5 accessibility pass; document gaps |
| GPUI/gpui-component churn | Pin versions; isolate in `hermes-ui`; upgrade deliberately |
| Terminal is absent from gpui-component | `portable-pty` + a VT crate; budget real time |
| Obscura renderer experimental | Only used for capture/agent-side, never login |
| Linux-only platform quirks (tray, Hyprland) | Platform module with explicit fallbacks |
| **Licence**: WebKitGTK is LGPL-2.1+; `lb-wry` is a fork | Dynamic linking, explicit record, `deny.toml` allowlist review |
| **Git dependencies forbidden** by `deny.toml` | Obscura stays a sidecar binary; no git-only crates (also rules out Servo/Blitz embedding) |

---

## 10. Tooling, CI, and delivery

Adopted from HyntixCode's `docs/TOOLING.md`. Only the parts that fit a Rust workspace apply; the HyntixCode-specific bits (Obsidian vault ownership, skills inventory, GitHub MCP install) are not carried over.

### Scripts — one entry point per job

| Script | Job |
|---|---|
| `scripts/build.sh` | Workspace build; `--release` for release, other args go to cargo |
| `scripts/lint.sh` | `cargo fmt --all --check` + `cargo clippy --workspace --all-targets -- -D warnings`; `--fix` formats |
| `scripts/test.sh` | The workspace test suite |
| `scripts/check.sh` | The gate: lint, then tests |
| `scripts/install-hooks.sh` | Point git at `.githooks` |
| `scripts/lib.sh` | Shared setup, sourced by the others |

### CI — six jobs, Linux only

| Job | What it proves | Local equivalent |
|---|---|---|
| Lint | formatting standard, clippy clean | `scripts/lint.sh` |
| Test | workspace suite passes under a display | `scripts/ui-test.sh` |
| Build (release) | `cargo build --workspace --release --locked` | same |
| Supply chain | `cargo deny check` | same |
| Specs | `openspec validate --specs` | same |
| Docs | required docs exist and relative links resolve | `scripts/check-docs.sh` |

The Test job installs `xvfb` and runs under `xvfb-run`, so UI tests can create
windows. The Specs job pins the npm CLI to the installed version
(`@fission-ai/openspec@1.17.0`), so CI and the local binary test the same
thing. Lint, Test, and Build install the WebKitGTK/GTK/X11 development headers
the first `gpui-kit` dependency will need.

Pinned Rust 1.100.0 (also in `rust-toolchain.toml`), `Swatinem/rust-cache`,
cancel superseded runs, `contents: read`. A single `ubuntu-latest` matrix — no
platform hidden behind a green check. Linux-first makes that honest.

### Dependency policy

`--locked` everywhere: commit `Cargo.lock`; a build that would resolve a new version fails instead.

`deny.toml` is the executable policy: known advisories, a licence allowlist matching the tree, **no wildcard versions, crates.io only**. Every crate is `publish = false`.

Two consequences specific to this project:

- **Obscura as a sidecar, not a crate.** The `obscura` crate is a **git dependency**, which this policy forbids. The binary is not a crate, so the sidecar stays allowed.
- **`gpui-webview` pulls `lb-wry`**, a crates.io fork of wry. Allowed, but it goes on the licence review.
- **WebKitGTK is LGPL-2.1+ and is a system library** — invisible to `cargo deny`. Record it explicitly and link dynamically.

### The gate

`.githooks/pre-push` runs `scripts/check.sh` and refuses the push when it fails; `SKIP_GATE=1` is the deliberate, recorded bypass. Branch protection is a paid feature on private repos, so the hook plus CI are the real gate.

**GPUI link weight.** TOOLING says HyntixCode has "no GPUI-sized link". HermesNative does. Set a non-tmpfs `CARGO_TARGET_DIR`, plan for `lld`/`mold`, and treat CI cache as load-bearing from P0.

### Verification honesty (the RTK rule)

The gate runs real `cargo` and prints its output verbatim. A compressed "0 warnings" is not evidence; a suite that does not exist is not reported as passing. Any completion claim quotes raw `cargo` output.

### Specs and working plans

- **OpenSpec** owns specs: `openspec/` is tracked, `openspec validate --specs` runs in CI, and generated `opsx-*` commands/skills are regenerated with `openspec update`, never hand-edited.
- **planning-with-files** owns working plans: `.planning/`, git-ignored, `/pwf [--gated|--autonomous]`, and `pwf_check` before reporting done.
- **CodeGraph** is per-project: `.codegraph/` git-ignored; `codegraph init`, then `codegraph explore`.
- **Ponytail** governs how code is written, not what must exist. The PRD, specs, and ADRs are deliverables.

### Release

`.github/workflows/release.yml` on a `v*` tag: tag must equal the crate version → gate + `cargo deny` → build `x86_64-unknown-linux-gnu` + `SHA256SUMS` → GitHub release with notes from Conventional Commits. Do not tag to test the pipeline. A prerelease (`0.1.0-alpha.1`) is the honest first artifact.

### Hired agents (subagents)

Useful for this repo: `rust-refactoring-specialist`, `software-architect`, `code-reviewer`, `evidence-collector`, `reality-checker`, `performance-benchmarker`, `application-security-engineer`, `security-architect`, `technical-writer`.

### Headless verification

The house pair is `obscura serve -p 9222 --allow-file-access` + `agent-browser connect 9222`. Use it to verify rendered docs/diagrams and agent browser flows. It **cannot** screenshot the native gpui window; native UI checks use Xvfb/cage plus a screenshot tool.

If HermesNative joins the shared knowledge vault, it owns `knowledge/HermesNative/` and nothing else, mirroring the same one-way sync rule.

## 11. Verification

- **Unit:** gateway transport, replay, dedupe, server-request handling (mock server).
- **Contract:** regenerate and diff in CI; fail on drift.
- **UI:** gpui-kit test harness for components; `cargo test` per crate.
- **Visual:** headless X (Xvfb/cage) screenshot tests for key surfaces.
- **Parity:** manual matrix against the checklist, per phase.

---

## 12. Immediate next actions

1. Create the Cargo workspace and pin `gpui-kit` / `gpui-webview` / `gpui-fast` at 0.11.0.
2. Drop in the `scripts/` gate (`build`, `lint`, `test`, `check`, `install-hooks`, `lib`), `.githooks/pre-push`, `deny.toml`, and `rust-toolchain.toml`.
3. Boot a themed window on X11 with `gpui_kit::init`; set a non-tmpfs `CARGO_TARGET_DIR` first.
4. Add the Rust emitter to `gen_gateway_contracts.py`; generate `hermes-contract`.
5. Implement connect + handshake + one round-trip (`session.list`).
6. Prove the webview: `gpui-webview` in a pane with a GPUI popover above it on X11.
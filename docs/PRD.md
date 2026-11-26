# PRD — HermesNative

**A native desktop client for Hermes, written in Rust with gpui-kit.**

| Field | Value |
|---|---|
| ID | PRD-HN-001 |
| Status | Draft (pre-implementation) |
| Date | 2026-11-27 |
| Owner | Unassigned |
| Reference implementation | `NousResearch/hermes-agent` @ `caafe5389a` (2026-11-19), `apps/desktop` |
| Delivery plan (canonical for architecture, crates, phases) | `docs/PLAN.md` |
| Backend | Hermes Agent (`hermes serve`), unmodified |

**Canonical sources.** This PRD is canonical for *product requirements, scope, and acceptance*. The plan is canonical for *architecture, crate layout, phases, and tooling*. Where they overlap, this document links rather than restates; a change to a requirement lands here first.

**Label legend (required).** Every behavior is `Live now` (exists in the Electron app today), `Target` (must exist in HermesNative), or `Transition` (temporary, with owner + end condition). Every metric is `Reference signal` (tracked, not gating) or `Release blocker` (gates v1).

---

## 1. Problem

Hermes Desktop ships as an Electron shell around a React renderer. That stack carries Chromium's memory and startup cost, a large native dependency surface, and a UI layer that cannot use the OS compositor or native controls. Users who run Hermes all day pay that cost in RAM, cold-start time, and battery.

**Who has the problem.** Hermes power users on Linux desktops (the first target), who keep a long-lived agent session open alongside an editor and browser.

**Context.** Desktop-first, all-day use: many sessions, long transcripts, tool output, file review, terminals, and a preview pane.

**Why now.** The agent and its gateway protocol already exist and are stable enough to be a client contract (263 methods, 79 notifications, 16 server→client requests, 910 schemas). GPUI now supports embedding a native webview in a composed scene, so the last blocking gap — in-client browsing — is closed.

**Baseline.** `Live now`: Electron Hermes Desktop. `Target`: HermesNative with the same agent behavior in a smaller, native shell.

---

## 2. Goals / Non-goals

### Goals

- **G1 — Functional parity.** Every user-visible capability of the Electron app is reachable in HermesNative: chat, sessions, projects, tools/approvals, four panes, all pages/overlays, and the platform extras. (`Target`)
- **G2 — Native footprint.** Lower idle memory and faster cold start than the Chromium-based shell, measured, not asserted. (`Target`)
- **G3 — One protocol, one truth.** All backend interaction goes through the gateway's OpenRPC contract, with types generated from it. (`Target`)
- **G4 — Real browser in the client.** Interactive pages, embeds, and login flows render in an embedded WebKitGTK view, not a frame stream. (`Target`)
- **G5 — Honest, enforced quality gate.** Format, lint, tests, supply chain, and spec validation run locally and in CI; completion claims quote raw output. (`Target`)

### Non-goals

- **N1** Rewriting or modifying the Hermes agent, its skills, tools, or memory.
- **N2** A bundled Python payload in v1. Connect to an existing local or remote gateway.
- **N3** Native Wayland in v1. Ship on X11/XWayland; Wayland is a follow-up.
- **N4** macOS and Windows in v1. Linux-first, single CI platform until the work is real.
- **N5** A pure-Rust browser engine. WebKitGTK (C) is accepted; Servo is deferred.
- **N6** Marketing claims about efficiency before controlled benchmarks exist.

---

## 3. Users, JTBD, and success context

| User | Job to be done | Current workaround | Success looks like |
|---|---|---|---|
| Hermes power user (Linux) | Run long agent sessions with tools, files, and a terminal in one window | Electron app, or CLI + separate tools | One native window; sessions survive restarts; no Chromium tax |
| Developer reviewing agent work | See diffs, files, and terminal output beside the transcript | Switch apps; copy output | Review and files panes attached to the session |
| Operator of a remote/cloud gateway | Drive a remote Hermes host from a local client | Browser dashboard or SSH | Same UI against a remote connection, no local runtime |
| Plugin/skill author | Inspect capabilities and artifacts | CLI / config files | Capabilities page manages connectors, MCP, plugins, skills, toolsets |

---

## 4. Requirements

### 4.1 Functional requirements

Status is the target state for HermesNative. "Source" names the Electron module family being matched.

| ID | Requirement | Acceptance (testable) | Source | Status |
|---|---|---|---|---|
| FR-1 | Connect to an existing local `hermes serve` or a saved remote/cloud gateway; probe before use | A saved connection reconnects after restart; a dead endpoint reports an actionable error, never a blank window | `local-gateway*`, `remote-*`, `connection-*` | Target |
| FR-2 | Session lifecycle: list, create, open, rename, archive, delete, branch, resume, set cwd | Each operation reflected in the sidebar and survives restart | `session.*` | Target |
| FR-3 | Submit prompts and stream assistant text, reasoning, and tool calls/output live | Tokens render incrementally; tool rows appear as events arrive; no dropped or reordered output | `prompt.submit`, `message.*`, `tool.*` | Target |
| FR-4 | Blocking approval and clarify cards, answered back to the server | A dangerous command cannot proceed until answered; the decision reaches the server | `approval`, `clarify` | Target |
| FR-5 | Composer: text, images, attachments, slash commands, `@path` completion, prompt history | Attachments reach the gateway; slash and path completion resolve; history recall works | `image.attach*`, `file.attach`, `complete.*` | Target |
| FR-6 | Model picker and reasoning effort | Selection applies to the next turn and persists per profile | `model.*`, `reasoning.*` | Target |
| FR-7 | Sidebar: session tree, profile/connection groups, projects (folders, repos, worktrees) | Groups collapse/expand; a project switch changes workspace cwd | `groups.*`, `projects.*` | Target |
| FR-8 | Workspace dock: tabs, splits, pane tree | Tabs persist across session switches; closing restores focus | pane shell | Target |
| FR-9 | Right rail: preview, files, review, terminal | Each pane mounts independently and survives temporary hiding | right sidebar | Target |
| FR-10 | Files pane: tree plus text/image/PDF preview and file watch | Editing a file on disk updates the preview | `fs-*`, `preview-file-watch` | Target |
| FR-11 | Review pane: git status/diff per file, worktrees | Diff matches `git diff`; worktree switch is reflected | `git-*` | Target |
| FR-12 | Terminal pane: PTY, tabs, resize, links | Runs an interactive shell; resize propagates; links open externally | `terminal-*` | Target |
| FR-13 | Preview pane: embedded WebKitGTK browser with navigation, capture, and sign-in persistence | A page renders and accepts input inside the window; cookies survive restart | `preview-*`, `embed-*` | Target |
| FR-14 | Settings: providers/models, tools, credentials, connections, appearance, keyboard, billing, memory, delegation, updates, about, advanced | Every Electron settings section has an equivalent that persists | `settings/*` | Target |
| FR-15 | Capabilities: connectors, MCP, plugins, skills, toolsets, catalog | Install/remove/configure round-trips through the gateway | `capabilities/*` | Target |
| FR-16 | Profiles and connections with soft switch | Switching wipes gateway-bound state and repopulates without a cold boot or cancelling turns | `profiles/*` | Target |
| FR-17 | Pages: session import, artifacts, cron, agents, starmap, messaging, webhooks, command center | Every route in `routes.ts` is reachable and functional | `app/*` | Target |
| FR-18 | Updates overlay and updater | An available update is reported; applying it restarts cleanly or hands off to checkout update | `updater/*` | Target |
| FR-19 | Voice input and speech output | Record → transcribe → insert; TTS plays | `voice.*` | Target |
| FR-20 | Native shell: tray, notifications, deep links, multi-window, keep-awake, global shortcut | Each is togglable in settings and behaves on Linux | `tray-*`, `notification-*`, `*-shortcut` | Target |
| FR-21 | Extras: pet overlay, HUD, wake indicator, quick entry, session windows | Each feature has a working Linux implementation or a documented exclusion | `pet-*`, `hud-*`, `wake-*`, `quick-entry` | Target |
| FR-22 | i18n, themes/skins, accessibility | Locale switch updates UI; theme tokens apply globally; keyboard-only operation of core flows | `locales/`, `themes/` | Target |
| FR-23 | Diagnostics: rotated logs, crash report surface | A failed boot writes a readable log with the backend's recent output | `desktop.log`, `crash-*` | Target |

### 4.2 Parity scope

The definition of "all features" is the checklist in the plan (§7): shell, chat, four panes, twelve pages, and native extras. An item is done only when it appears in the checklist as complete and its acceptance above is met.

### 4.3 Non-functional requirements

| Area | Requirement | Metric class |
|---|---|---|
| Startup | Cold start to an interactive window is measured and below the Electron baseline on the same machine | Release blocker (calibrate in P0) |
| Memory | Idle RSS is measured and below the Electron baseline with one session open | Release blocker (calibrate in P0) |
| Streaming | Transcript keeps up with the gateway; no visible token backlog at normal rates; p95 keystroke-to-render under one frame | Reference signal |
| Reliability | Gateway restart reconnects within 5s without losing or duplicating turns | Release blocker |
| Security | Secrets live only in the OS keyring; no plaintext tokens on disk; remote headers drop transport-managed names | Release blocker |
| Licensing | `cargo deny` passes: advisories, licence allowlist, no wildcard versions, crates.io only | Release blocker |
| Compatibility | X11 and XWayland on Linux; native Wayland explicitly unsupported in v1 | Release blocker (scope) |
| Accessibility | Keyboard-only operation of chat, navigation, and approvals; gaps documented rather than claimed | Reference signal in v1, blocker for GA |
| i18n | No hard-coded user-facing strings in new UI code | Reference signal |

### 4.4 Data and analytics

- **Default:** no telemetry. Local logs only, rotated, never uploaded.
- **If telemetry is added:** opt-in, aggregate, no transcript content, no file paths, no tokens. Events: app start, gateway connect/reconnect, pane open, error class. (`Target`)
- **Secrets:** OS keyring only. Logs redact tokens and remote headers. (`Target`)

### 4.5 Dependencies and constraints

- `hermes serve` exposing the `tui_gateway` JSON-RPC/WebSocket API and HTTP endpoints.
- `gpui-kit` 0.11.0, `gpui-component` 0.11.0, `gpui-webview` 0.11.0, `gpui-fast` 0.1.19 (versions must agree).
- WebKitGTK and GTK3 development headers; X11/XWayland at runtime.
- `portable-pty` and a VT crate for the terminal.
- `git2`/`gix` for review.
- Obscura sidecar (agent-side only; never the in-app browser).
- Tooling: `scripts/` gate, six-job CI, `deny.toml`, `--locked`, hook-enforced pre-push (see plan §10).

---

## 5. Scope, plan, and open questions

### Milestones

Phases and exit criteria live in the plan (§8). Summary:

| Phase | Outcome |
|---|---|
| P0 | Workspace, pinned stack, contract codegen, gateway handshake, themed window |
| P1 | Chat core with streaming, tools, approvals, composer |
| P2 | Shell: sidebar, dock, right rail, palette, profiles, projects |
| P3 | Panes: files, review, terminal, preview |
| P4 | All pages and overlays |
| P5 | Extras, i18n, accessibility, hardening |

### Open questions

| # | Question | Owner | Due |
|---|---|---|---|
| Q1 | Does the webview wrapper need to stay on X11, or can it follow `gpui-fast` Wayland composition? | Platform | Before P3 |
| Q2 | Is native Wayland a v1 requirement or a follow-up? (Current answer: follow-up.) | Product | Before P3 |
| Q3 | Bundled Python payload ever, or permanent connect-to-existing? | Product | Before v1 |
| Q4 | What is the accessibility bar for v1 — core flows only, or full WCAG 2.2 AA? | Product | Before P2 |
| Q5 | Telemetry: none forever, or opt-in aggregate? | Product | Before P4 |
| Q6 | Do pet/HUD ship in v1 or as a post-v1 extra? | Product | Before P5 |

### Assumptions

- The gateway contract is stable enough to generate Rust types from; drift is caught by regeneration in CI.
- `gpui-webview` builds and runs on the target X11/XWayland machine with WebKitGTK installed.
- The Electron reference clone is available for parity review (not yet cloned into the project).

---

## 6. Acceptance criteria and success metrics

### v1 acceptance (all must hold)

- [ ] `scripts/check.sh` passes on a clean clone; CI is green on all six jobs.
- [ ] A user can connect to a gateway, hold a conversation with tool calls and an approval, and restart without losing history. (`FR-1`–`FR-4`)
- [ ] All four panes work: files, review, terminal, preview. (`FR-10`–`FR-13`)
- [ ] Every route in `routes.ts` is reachable and functional. (`FR-17`)
- [ ] Settings, profiles/connections, and capabilities round-trip through the gateway. (`FR-14`–`FR-16`)
- [ ] `cargo deny check` passes with WebKitGTK recorded as an LGPL system dependency.
- [ ] The parity checklist in plan §7 is complete on Linux, with exclusions documented.
- [ ] No completion claim is made without quoted raw `cargo` output.

### Success metrics

| Metric | Class | Baseline | Target | Window |
|---|---|---|---|---|
| Idle RSS, one session | Release blocker | Electron app, same machine | Lower; exact value set in P0 | P0 calibration |
| Cold start to interactive | Release blocker | Electron app, same machine | Lower; exact value set in P0 | P0 calibration |
| Reconnect after gateway restart | Release blocker | n/a | < 5s, no lost turns | Every run |
| Parity checklist completion | Release blocker | 0% | 100% with documented exclusions | v1 |
| Crash-free sessions | Reference signal | n/a | > 99% over a week of dogfooding | Rolling |

Baselines are measured, not copied from vendor claims. A metric without a measured baseline is not a release blocker.

---

## 7. Decision rules

- **Start P1 only if** the contract generator emits compiling Rust and the gateway handshake round-trips.
- **Ship v1 only if** every v1 acceptance box is checked and the gate is green.
- **Stop or roll back a pane/feature if** it cannot meet its acceptance on X11/XWayland after one phase, and move it to a documented exclusion.
- **Do not tag a release to test the pipeline.** Test the steps locally.
- **Do not claim parity** for a feature that is not in the checklist as complete.

---

## 8. Risks

| Risk | Impact | Mitigation |
|---|---|---|
| Webview wrapper requires X11; Wayland users blocked | Adoption on native Wayland | Ship XWayland; track upstream composition; Q1/Q2 |
| `gpui-*` crates churn or version drift | Build breaks mid-phase | Pin 0.11.0; isolate in `hermes-ui`; upgrade deliberately |
| Contract drift across 263 methods | Silent protocol breakage | Generated types; regenerate-and-diff CI |
| GPUI accessibility/IME weaker than Chromium | Usability gap | Dedicated P5 pass; document gaps; Q4 |
| Terminal is not provided by gpui-component | P3 slip | Budget real time; `portable-pty` + VT crate |
| Licence: WebKitGTK LGPL; `lb-wry` fork | Supply-chain/legal | Dynamic link; record explicitly; `deny.toml` allowlist |
| crates.io-only policy blocks git deps | Cannot embed Obscura/Servo/Blitz | Obscura stays a sidecar; no git-only crates |
| Scope: full parity is multi-quarter | Never ships | Phase exits; documented exclusions; Q6 |

---

## Appendix A — Reference facts

- Electron source: `https://github.com/NousResearch/hermes-agent`, monorepo, desktop at `apps/desktop`; local checkout HEAD `caafe5389a` (2026-11-19).
- Renderer size: 3,798 files under `apps/desktop/src`.
- Gateway contract: `apps/shared/src/gateway-contract.openrpc.json` — 263 methods, 79 notifications, 16 server requests, 910 schemas.
- Contract generator: `scripts/gen_gateway_contracts.py` (Pydantic → TypeScript + OpenRPC). Extend with a Rust emitter.
- Installed Obscura: `0.1.0-dev+74b4bec` (pre-rendering). Rendering/screencast arrived in v0.2.0. Agent-side only.
- gpui-kit / gpui-webview / gpui-component `0.11.0`; gpui-fast `0.1.19` (released 2026-11-23).
- Verified 2026-11-27 (sway, XWayland): `gpui-webview` 0.11.0 builds in 3m56s and renders `example.com`; requires `GDK_BACKEND=x11`. Without it, `gtk::init()` fails. `gpui-fast` exposes `wlr-layer-shell` (Background/Bottom/Top/Overlay), so HUD overlays are possible; the webview itself still refuses Wayland.
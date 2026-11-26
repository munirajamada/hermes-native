# Verification: stack assumptions

Tested before writing product code, on 2026-11-27.

Environment: Arch Linux, **sway (Wayland)** with XWayland at `:0`, Rust 1.100.0,
WebKitGTK 2.54.3, GTK 3.24.52, glibc 2.44, GCC 16.2.1. gpui-kit /
gpui-component / gpui-webview 0.11.0, gpui-fast 0.1.19.

| # | Assumption | Result |
|---|---|---|
| 1 | The workspace builds and its gate passes | `cargo check` 0 errors; `scripts/check.sh` exit 0 |
| 2 | `gpui-webview` builds on this machine | Built against gpui-kit 0.11.0 in 3m56s |
| 3 | The webview renders on XWayland | `example.com` rendered inside the GPUI window |
| 4 | GPUI overlays render above the native webview | `deferred()` red panel drawn over the page |
| 5 | A layer-shell overlay works on native Wayland | `Layer::Overlay` window drawn above the desktop |
| 6 | The gateway speaks JSON-RPC/WS and answers `session.list` | `gateway.ready` + a live session list |

## 2–4. Embedded browser

`gpui-webview` 0.11.0 (wry → WebKitGTK) as a child of a GPUI window:

```rust
gpui_kit::platform::linux(WindowingModes::X11).run(|cx| {
    gpui_kit::init(cx);
    gpui_kit::open_window(WindowOptions::default(), cx, |window, cx| {
        let webview = cx.new(|cx| {
            let builder = wry::WebViewBuilder::new().with_url("https://example.com");
            WebView::build(builder, window, cx).expect("webview")
        });
        cx.new(|_| Root { webview })
    });
});
```

- Without `GDK_BACKEND=x11`, `gtk::init()` fails: `Failed to initialize GTK`.
  With it, the page renders.
- Native Wayland is impossible for this pane: GTK cannot embed into a Wayland
  surface it does not own (`gpui-webview/src/linux.rs`).

**Overlay above the page.** With the `gpui-fast` feature on both crates, a
`deferred(anchored().position(...).child(...))` element paints in the overlay
scene, above the native surface:

```rust
div().relative().size_full()
    .child(div().absolute().inset_0().child(self.webview.clone()))
    .child(deferred(anchored().position(point(px(24.), px(24.)))
        .child(div().w(px(460.)).h(px(170.)).bg(rgb(0xcc2222))
            .child("OVERLAY ABOVE WEBVIEW"))))
```

Observed: the red panel with `OVERLAY ABOVE WEBVIEW` covered the top-left of
the rendered page.

## 5. Layer-shell on native Wayland

```rust
gpui_kit::platform::linux(WindowingModes::WAYLAND).run(|cx| {
    cx.open_window(WindowOptions {
        kind: WindowKind::LayerShell(LayerShellOptions {
            namespace: "hermes-stacktest".into(),
            anchor: Anchor::TOP | Anchor::LEFT,
            keyboard_interactivity: KeyboardInteractivity::None,
            ..Default::default()          // Layer::Overlay
        }),
        ..Default::default()
    }, |_, cx| cx.new(|_| View)).unwrap();
});
```

Observed: an overlay-layer window drawn above the desktop on sway. GPUI exposes
`wlr-layer-shell` with Background/Bottom/Top/Overlay and anchors, so the
HUD/pet surfaces do not depend on the webview.

## 6. Gateway round-trip

`hermes gateway attach --json` returns the URL, a 30-second single-use ticket,
and the subprotocols to dial with. Connecting without the ticket is `403`.

```python
grant = json.loads(run(["hermes", "gateway", "attach", "--json"]).stdout)
ws = await websockets.connect(grant["url"], subprotocols=grant["protocols"])
# negotiated: hermes-gateway-v1
# ping -> event gateway.ready (with skin)
# session.list -> {"sessions": [...], "scope": "live"}
```

Observed: `gateway.ready` arrived, then `session.list` returned one live
session. The Rust client must dial with the ticket subprotocol and perform the
same attach dance.

## Not tested

- gpui-fast's own headless composition unit tests (the end-to-end overlay test
  above exercises the same path).
- A webview on native Wayland — not possible with this stack.
- macOS and Windows.

## Reproducing

The throwaway crates lived in `/tmp/opencode/stacktest` with a target dir on
the root filesystem (`/tmp` is a 6.8 GB tmpfs and a gpui-fast build fills it).
The snippets above are the whole test.
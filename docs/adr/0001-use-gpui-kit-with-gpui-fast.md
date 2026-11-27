# Use gpui-kit with the gpui-fast backend

The UI is built on gpui-kit 0.11 with the `gpui-fast` feature. The preview pane needs a native webview composited into the GPUI scene and the HUD needs `wlr-layer-shell`; both exist only in the fast backend, and switching platform crates later would touch window composition.

**Considered options:** plain `gpui-pre` (no composition), egui (no native webview), Tauri (puts a web renderer back under the whole UI).

# Embed WebKitGTK through gpui-webview on X11

The preview pane embeds `gpui-webview` 0.11 (wry → WebKitGTK) as a native child view, so the app runs on X11/XWayland. Native Wayland is impossible for this pane because GTK cannot embed into a Wayland surface it does not own; the verified fallback would be frame-streaming a headless engine, which loses selection, IME, and login flows.

**Considered options:** CEF offscreen (200 MB–1 GB, multi-process plumbing), Obscura frame streaming (headless), opening pages in the system browser.

//! gpui-kit UI: shell, panes, chat, and pages.

use gpui_kit::*;

/// Product name.
pub fn name() -> &'static str {
    "HermesNative"
}

struct Root {
    sessions: Option<usize>,
}

impl Render for Root {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let status = match self.sessions {
            Some(count) => format!("gateway connected · {count} live session(s)"),
            None => "gateway not connected".to_string(),
        };
        div()
            .size_full()
            .bg(rgb(0x111318))
            .text_color(rgb(0xe6e8ee))
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_3()
            .child(
                div()
                    .text_size(px(40.))
                    .font_weight(FontWeight::BOLD)
                    .child(name()),
            )
            .child(
                div()
                    .text_size(px(16.))
                    .text_color(rgb(0x9aa3b2))
                    .child("P0 foundation"),
            )
            .child(
                div()
                    .text_size(px(14.))
                    .text_color(rgb(0x7d8798))
                    .child(status),
            )
    }
}

/// Open the first window. `sessions` is the probe result, or `None` if the
/// gateway was not reachable.
pub fn run(sessions: Option<usize>) {
    gpui_kit::platform::linux(WindowingModes::X11).run(move |cx| {
        gpui_kit::init(cx);
        gpui_kit::open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some(name().into()),
                    ..Default::default()
                }),
                app_id: Some("com.hyntix.hermesnative".into()),
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(0.), px(0.)),
                    size: Size::new(px(1100.), px(760.)),
                })),
                window_min_size: Some(Size::new(px(720.), px(480.))),
                ..Default::default()
            },
            cx,
            move |_, cx| cx.new(|_| Root { sessions }),
        )
        .expect("failed to open the window");
    });
}

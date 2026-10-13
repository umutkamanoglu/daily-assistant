mod components;

use components::dashboard::Dashboard;
use gpui::*;
use gpui_component::Root;

fn main() {
    Application::new().run(|cx: &mut App| {
        gpui_component::init(cx);

        let bounds = Bounds::centered(None, size(px(1100.), px(750.)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                let view = cx.new(|_| Dashboard::new());
                cx.new(|cx| Root::new(view, window, cx))
            },
        )
        .unwrap();
    });
}

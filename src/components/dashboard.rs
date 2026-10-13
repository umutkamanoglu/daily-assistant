use gpui::*;
use gpui_component::{h_flex, v_flex, ActiveTheme};

use crate::components::sidebar::sidebar;

pub struct Dashboard {
    pub sidebar_collapsed: bool,
}

impl Dashboard {
    pub fn new() -> Self {
        Self { sidebar_collapsed: false }
    }
}
const TASKS: [(&str, &str); 6] = [
    ("08:00", "Sabah rutini"),
    ("09:00", "Rust çalış"),
    ("11:00", "Proje üzerinde çalış"),
    ("13:00", "Öğle arası"),
    ("14:00", "Video çekimi"),
    ("17:00", "Günü değerlendir"),
];

fn header(cx: &App) -> impl IntoElement {
    v_flex()
        .gap_1()
        .child(div().text_2xl().child("Bugün"))
        .child(
            div()
                .text_color(cx.theme().muted_foreground)
                .child("Günlük özet ve plan"),
        )
}

fn stat_card(title: &'static str, value: &'static str, cx: &App) -> impl IntoElement {
    v_flex()
        .flex_1()
        .gap_1()
        .p_4()
        .rounded_lg()
        .border_1()
        .border_color(cx.theme().border)
        .child(
            div()
                .text_sm()
                .text_color(cx.theme().muted_foreground)
                .child(title),
        )
        .child(div().text_2xl().child(value))
}

fn timeline(cx: &App) -> impl IntoElement {
    let border = cx.theme().border;
    let muted = cx.theme().muted_foreground;

    v_flex()
        .flex_1()
        .min_h_0()
        .gap_2()
        .child(div().text_lg().child("Timeline"))
        .child(
            v_flex()
                .id("timeline-list")
                .flex_1()
                .min_h_0()
                .overflow_y_scroll()
                .children(TASKS.iter().map(move |(time, title)| {
                    h_flex()
                        .gap_4()
                        .py_3()
                        .border_b_1()
                        .border_color(border)
                        .child(div().w(px(60.)).text_color(muted).child(*time))
                        .child(div().flex_1().child(*title))
                })),
        )
}

impl Render for Dashboard {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(sidebar(self.sidebar_collapsed, cx))
            .child(
                v_flex()
                    .flex_1()
                    .h_full()
                    .p_6()
                    .gap_6()
                    .child(header(cx))
                    .child(
                        h_flex()
                            .gap_4()
                            .child(stat_card("Görevler", "6", cx))
                            .child(stat_card("Tamamlanan", "2", cx))
                            .child(stat_card("Focus süresi", "1s 30dk", cx)),
                    )
                    .child(timeline(cx)),
            )
    }
}

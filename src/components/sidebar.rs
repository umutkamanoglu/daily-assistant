use gpui::*;
use gpui_component::{button::*, v_flex, ActiveTheme};

pub fn sidebar(cx: &App) -> impl IntoElement {
    v_flex()
        .w(px(220.))
        .h_full()
        .p_3()
        .gap_1()
        .border_r_1()
        .border_color(cx.theme().border)
        .child(div().px_2().py_3().text_lg().child("Daily Assistant"))
        .child(Button::new("nav-dashboard").label("Dashboard").primary().w_full())
        .child(Button::new("nav-goals").label("Hedefler").ghost().w_full())
        .child(Button::new("nav-notes").label("Notlar").ghost().w_full())
}

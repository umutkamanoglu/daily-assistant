use gpui::*;
use gpui_component::{button::*, h_flex, v_flex, ActiveTheme, IconName};

use crate::components::dashboard::Dashboard;

const WIDTH_EXPANDED: f32 = 220.;
const WIDTH_COLLAPSED: f32 = 64.;

pub fn sidebar(collapsed: bool, cx: &mut Context<Dashboard>) -> AnyElement {
    let border = cx.theme().border;

    let toggle_icon = if collapsed {
        IconName::PanelLeftOpen
    } else {
        IconName::PanelLeftClose
    };

    let toggle = Button::new("sidebar-toggle")
        .ghost()
        .icon(toggle_icon)
        .cursor_pointer()
        .on_click(cx.listener(|this, _, _, cx| {
            this.sidebar_collapsed = !this.sidebar_collapsed;
            cx.notify();
        }));

    let header = if collapsed {
        h_flex().justify_center().pb_2().child(toggle)
    } else {
        h_flex()
            .items_center()
            .justify_between()
            .pb_2()
            .child(div().px_2().text_lg().child("Daily Assistant"))
            .child(toggle)
    };

    v_flex()
        .w(px(if collapsed { WIDTH_COLLAPSED } else { WIDTH_EXPANDED }))
        .h_full()
        .flex_shrink_0()
        .p_2()
        .gap_1()
        .border_r_1()
        .border_color(border)
        .child(header)
        .child(nav_item("nav-dashboard", "Dashboard", IconName::ChartPie, true, collapsed))
        .child(nav_item("nav-goals", "Hedefler", IconName::CircleCheck, false, collapsed))
        .child(nav_item("nav-notes", "Notlar", IconName::BookOpen, false, collapsed))
        .into_any_element()
}

fn nav_item(
    id: &'static str,
    label: &'static str,
    icon: IconName,
    active: bool,
    collapsed: bool,
) -> impl IntoElement {
    let mut button = Button::new(id)
        .icon(icon)
        .w_full()
        .h(px(40.))
        .cursor_pointer();

    if !collapsed {
        button = button.label(label);   // daralınca sadece ikon kalır
    }

    if active { button.primary() } else { button.ghost() }
}

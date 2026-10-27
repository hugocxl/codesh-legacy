use crate::views::menu::{Command, Item};
use gpui::prelude::FluentBuilder;
use gpui::{
    AnyElement, Context, InteractiveElement, IntoElement, ParentElement,
    StatefulInteractiveElement, Styled, Window, canvas, div, px,
};
use muxy_app_core::settings::SidebarHost;
use muxy_ui::components::IconGlyph;
use muxy_ui::icon::Icon;

use super::style;
use crate::model::AppModel;
use crate::views::titlebar;

pub(crate) fn top(
    model: &AppModel,
    width: f32,
    _window: &Window,
    cx: &mut Context<AppModel>,
) -> AnyElement {
    let m = model.metrics;
    titlebar::background("titlebar-navigation")
        .absolute()
        .top_0()
        .left_0()
        .w(px(width.max(if cfg!(target_os = "macos") {
            f32::from(m.traffic_light_width()) + 44.0
        } else {
            44.0
        })))
        .h(m.title_bar_height())
        .px(m.scaled(if model.appearance.sidebar_expanded {
            style::SIDEBAR_PADDING
        } else {
            8.0
        }))
        .when(cfg!(target_os = "macos"), |bar| {
            bar.pl(m.traffic_light_width())
        })
        .flex()
        .items_center()
        .child(titlebar::sidebar_toggle(model, cx))
        .when(model.appearance.sidebar_expanded, |bar| {
            bar.child(div().flex_1())
                .child(model.sidebar_extension_toolbar(cx))
        })
        .into_any_element()
}

pub(crate) fn bottom(model: &AppModel, _window: &Window, cx: &mut Context<AppModel>) -> AnyElement {
    let m = model.metrics;
    let theme = &model.theme;
    let anchor = model.remote_anchor.clone();
    div()
        .flex()
        .flex_none()
        .items_center()
        .w_full()
        .min_w(px(0.0))
        .when(!model.appearance.sidebar_expanded, Styled::flex_col)
        .px(m.scaled(if model.appearance.sidebar_expanded {
            style::SIDEBAR_PADDING
        } else {
            4.0
        }))
        .py(m.scaled(8.0))
        .gap(m.scaled(4.0))
        .child(
            titlebar::icon_button("sidebar-host", Icon::Overlap, "Hosts", model)
                .mr(px(0.0))
                .debug_selector(|| "sidebar-host".into())
                .child(
                    canvas(
                        move |bounds, _, _| anchor.set(Some(bounds)),
                        |_, (), _, _| {},
                    )
                    .absolute()
                    .size_full(),
                )
                .on_click(cx.listener(|model, event: &gpui::ClickEvent, window, cx| {
                    let selected = model.appearance.sidebar_host;
                    let mut items: Vec<_> = [
                        ("All hosts", SidebarHost::All),
                        ("This Mac", SidebarHost::Local),
                    ]
                    .into_iter()
                    .map(|(title, host)| {
                        Item::action(title, Command::SidebarHost(host)).checked_if(selected == host)
                    })
                    .collect();
                    items.extend(model.remote_servers().into_iter().map(|server| {
                        let host = SidebarHost::Server(server.entry.id);
                        Item::action(server.entry.name, Command::SidebarHost(host))
                            .checked_if(selected == host)
                    }));
                    items
                        .push(Item::action("Manage servers…", Command::SidebarServers).separated());
                    model.open_menu(items, event.position(), window, cx);
                })),
        )
        .when(model.appearance.sidebar_expanded, |bar| {
            bar.child(
                div()
                    .id("sidebar-name-filter")
                    .debug_selector(|| "sidebar-name-filter".into())
                    .flex()
                    .flex_1()
                    .min_w(px(0.0))
                    .items_center()
                    .h(m.scaled(28.0))
                    .px(m.scaled(8.0))
                    .gap(m.scaled(6.0))
                    .rounded(m.scaled(14.0))
                    .bg(theme.surface)
                    .child(IconGlyph::new(Icon::Filter, m.scaled(16.0), theme.fg_muted))
                    .child(
                        div()
                            .flex_1()
                            .min_w(px(0.0))
                            .overflow_hidden()
                            .child(model.sidebar_filter_input.clone()),
                    ),
            )
        })
        .child(
            titlebar::icon_button("codesh-add", Icon::Plus, "Add Project", model)
                .mr(px(0.0))
                .on_click(cx.listener(|model, event: &gpui::ClickEvent, window, cx| {
                    cx.stop_propagation();
                    model.choose_project_source(event.position(), window, cx);
                })),
        )
        .child(
            titlebar::icon_button("new-browser-tab", Icon::Globe, "New Browser", model)
                .debug_selector(|| "browser-button".into())
                .mr(px(0.0))
                .on_click(cx.listener(|model, _, _, cx| {
                    cx.stop_propagation();
                    model.new_browser_tab(cx);
                })),
        )
        .into_any_element()
}

use gpui::prelude::FluentBuilder;
use gpui::{
    AnyElement, Context, IntoElement, ParentElement, StatefulInteractiveElement, Styled, Window,
    div, px,
};
use muxy_ui::icon::Icon;

use super::style;
use crate::model::AppModel;
use crate::views::{tab_strip, titlebar};

pub(crate) fn top(
    model: &AppModel,
    width: f32,
    _window: &Window,
    cx: &mut Context<AppModel>,
) -> AnyElement {
    let m = model.metrics;
    let theme = &model.theme;
    titlebar::background("titlebar-navigation")
        .absolute()
        .top_0()
        .left_0()
        .w(px(width.max(44.0)))
        .h(m.title_bar_height() + m.scaled(32.0))
        .pt(m.scaled(32.0))
        .px(m.scaled(if model.appearance.sidebar_expanded {
            style::SIDEBAR_PADDING
        } else {
            8.0
        }))
        .flex()
        .items_center()
        .child(titlebar::sidebar_toggle(model, cx))
        .when(model.appearance.sidebar_expanded, |bar| {
            bar.child(div().flex_1())
                .child(
                    titlebar::action_button("codesh-command", "Command Palette", model)
                        .text_color(theme.fg_muted)
                        .child("⌘")
                        .on_click(cx.listener(|model, _, window, cx| {
                            cx.stop_propagation();
                            model.toggle_command_palette(window, cx);
                        })),
                )
                .child(
                    titlebar::icon_button(
                        "codesh-focus",
                        Icon::Eye,
                        "Focus Current Project",
                        model,
                    )
                    .on_click(cx.listener(|model, _, _, cx| {
                        cx.stop_propagation();
                        model.set_project_focus(!model.appearance.sidebar_focus, cx);
                    })),
                )
                .child(
                    titlebar::icon_button("codesh-add", Icon::Plus, "Add Project", model).on_click(
                        cx.listener(|model, event: &gpui::ClickEvent, window, cx| {
                            cx.stop_propagation();
                            model.choose_project_source(event.position(), window, cx);
                        }),
                    ),
                )
        })
        .into_any_element()
}

pub(crate) fn bottom(model: &AppModel, window: &Window, cx: &mut Context<AppModel>) -> AnyElement {
    let (actions, _) = titlebar::actions(model, window, cx);
    div()
        .flex()
        .flex_none()
        .items_center()
        .flex_wrap()
        .w_full()
        .min_w(px(0.0))
        .when(!model.appearance.sidebar_expanded, Styled::flex_col)
        .px(model.metrics.scaled(if model.appearance.sidebar_expanded {
            style::SIDEBAR_PADDING
        } else {
            4.0
        }))
        .py(model.metrics.scaled(8.0))
        .gap(model.metrics.scaled(4.0))
        .child(tab_strip::settings_button(model, cx))
        .child(div().flex_1())
        .child(
            actions
                .flex_wrap()
                .min_w(px(0.0))
                .when(!model.appearance.sidebar_expanded, |actions| {
                    actions.w_full()
                }),
        )
        .child(crate::views::sidebar::header(model, cx))
        .child(titlebar::arrow(model, false, cx))
        .child(titlebar::arrow(model, true, cx))
        .child(crate::views::remote_servers::section(model, cx))
        .into_any_element()
}
